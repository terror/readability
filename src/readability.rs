use super::*;
use ego_tree::{NodeRef, iter::Edge};
use html5ever::serialize::{
  Serialize, SerializeOpts, Serializer, TraversalScope, serialize,
};

const CLASSES_TO_PRESERVE: &[&str] = &["page"];

static REGEX_NORMALIZE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\s{2,}").unwrap());

pub struct Readability {
  _base_url: Option<Url>,
  article_dir: Option<String>,
  html: Html,
  options: ReadabilityOptions,
}

impl Readability {
  /// Creates a new readability parser instance.
  ///
  /// # Errors
  ///
  /// Returns an error when the optional `base_url` cannot be parsed.
  pub fn new(
    html: &str,
    base_url: Option<&str>,
    options: ReadabilityOptions,
  ) -> Result<Self> {
    let base_url = base_url
      .map(|value| Url::parse(value).map_err(Error::from))
      .transpose()?;

    Ok(Self {
      _base_url: base_url,
      article_dir: None,
      html: Html::parse_document(html),
      options,
    })
  }

  /// Extracts the article contents using the configured pipeline.
  ///
  /// # Errors
  ///
  /// Returns an error when the pipeline cannot resolve article content.
  pub fn parse(&mut self) -> Result<Article> {
    let mut context = Pipeline::with_default_stages(Context::new(
      &mut self.html,
      &self.options,
    ))
    .run()?;

    let mut markup = context
      .take_article_markup()
      .ok_or(Error::MissingArticleContent)?;

    markup = Self::post_process_markup(markup, self._base_url.as_ref());

    let title = context
      .metadata()
      .title
      .clone()
      .filter(|value| !value.is_empty())
      .or(context.document().document_title())
      .unwrap_or(String::new());

    let lang = context
      .body_lang()
      .cloned()
      .or(context.document_lang().cloned());

    let fragment = Html::parse_fragment(&markup);

    let mut text = String::new();

    for node in fragment.tree.root().descendants() {
      if let Node::Text(value) = node.value() {
        if !text.is_empty() {
          text.push(' ');
        }

        text.push_str(value.trim());
      }
    }

    let text_content = REGEX_NORMALIZE
      .replace_all(&text, " ")
      .into_owned()
      .trim()
      .to_string();

    let selector = Selector::parse("p")
      .map_err(|error| Error::InvalidSelector(error.to_string()))?;

    let fragment = Html::parse_fragment(&markup);

    let first_paragraph = fragment
      .select(&selector)
      .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
      .find(|text| !text.is_empty());

    let excerpt = context.metadata().excerpt.clone().or(first_paragraph);

    Ok(Article {
      title,
      byline: context.metadata().byline.clone(),
      dir: self.article_dir.clone(),
      lang,
      content: markup,
      text_content: text_content.clone(),
      length: text_content.chars().count(),
      excerpt,
      site_name: context.metadata().site_name.clone(),
      published_time: context.metadata().published_time.clone(),
    })
  }
}

impl Readability {
  fn clean_classes(fragment: &mut Html, root_id: NodeId) {
    let Some(root) = fragment.tree.get(root_id) else {
      return;
    };

    let node_ids: Vec<NodeId> = root
      .descendants()
      .map(|descendant| descendant.id())
      .collect();

    for node_id in node_ids {
      let Some(mut node) = fragment.tree.get_mut(node_id) else {
        continue;
      };

      let Node::Element(element) = node.value() else {
        continue;
      };

      let class_index = element
        .attrs
        .iter()
        .position(|(name, _)| name.local.as_ref() == "class");

      let Some(index) = class_index else {
        continue;
      };

      let class_value = element.attrs[index].1.to_string();
      let preserved: Vec<&str> = class_value
        .split_whitespace()
        .filter(|class_name| CLASSES_TO_PRESERVE.contains(class_name))
        .collect();

      if preserved.is_empty() {
        element.attrs.remove(index);
      } else {
        let new_value = preserved.join(" ");
        element.attrs[index].1.clear();
        element.attrs[index].1.push_slice(&new_value);
      }
    }
  }

  fn find_attribute_index(element: &Element, name: &str) -> Option<usize> {
    element
      .attrs
      .iter()
      .position(|(attr_name, _)| attr_name.local.as_ref() == name)
  }

  fn fix_relative_uris(fragment: &mut Html, root_id: NodeId, base_url: &Url) {
    let Some(root) = fragment.tree.get(root_id) else {
      return;
    };

    let node_ids: Vec<NodeId> = root
      .descendants()
      .map(|descendant| descendant.id())
      .collect();

    for node_id in node_ids {
      let Some(mut node) = fragment.tree.get_mut(node_id) else {
        continue;
      };

      let Node::Element(element) = node.value() else {
        continue;
      };

      if let Some(index) = Self::find_attribute_index(&element, "href") {
        let href_value = element.attrs[index].1.to_string();

        if !href_value.starts_with('#') && !Self::is_javascript_uri(&href_value)
        {
          let resolved = Self::resolve_uri(base_url, &href_value);
          element.attrs[index].1.clear();
          element.attrs[index].1.push_slice(&resolved);
        }
      }

      if let Some(index) = Self::find_attribute_index(&element, "src") {
        let src_value = element.attrs[index].1.to_string();
        let resolved = Self::resolve_uri(base_url, &src_value);
        element.attrs[index].1.clear();
        element.attrs[index].1.push_slice(&resolved);
      }

      if let Some(index) = Self::find_attribute_index(&element, "poster") {
        let poster_value = element.attrs[index].1.to_string();
        let resolved = Self::resolve_uri(base_url, &poster_value);
        element.attrs[index].1.clear();
        element.attrs[index].1.push_slice(&resolved);
      }

      if let Some(index) = Self::find_attribute_index(&element, "srcset") {
        let srcset_value = element.attrs[index].1.to_string();
        let resolved = Self::rewrite_srcset(&srcset_value, base_url);
        element.attrs[index].1.clear();
        element.attrs[index].1.push_slice(&resolved);
      }
    }
  }

  fn is_javascript_uri(value: &str) -> bool {
    value
      .trim_start()
      .to_ascii_lowercase()
      .starts_with("javascript:")
  }

  fn post_process_markup(markup: String, base_url: Option<&Url>) -> String {
    let mut fragment = Html::parse_fragment(&markup);

    let target_id = fragment
      .tree
      .root()
      .descendants()
      .find(|node| {
        matches!(node.value(), Node::Element(element) if element.name() == "body")
      })
      .map(|node| node.id())
      .unwrap_or_else(|| fragment.tree.root().id());

    if let Some(base_url) = base_url {
      Self::fix_relative_uris(&mut fragment, target_id, base_url);
    }

    Self::clean_classes(&mut fragment, target_id);
    Self::normalize_whitespace_nodes(&mut fragment, target_id);

    if let Ok(selector) = Selector::parse("#readability-page-1") {
      if let Some(element) = fragment.select(&selector).next() {
        let inner = element.inner_html();

        let markup = format!(
          "<div id=\"readability-page-1\" class=\"page\">{inner}</div>"
        );

        return Self::enforce_void_self_closing(markup);
      }
    }

    let Some(target) = fragment.tree.get(target_id) else {
      return markup;
    };

    let processed = Self::serialize_children(target);

    if processed.is_empty() {
      markup
    } else {
      Self::enforce_void_self_closing(processed)
    }
  }

  fn resolve_uri(base_url: &Url, value: &str) -> String {
    if value.is_empty() {
      return value.to_string();
    }

    if Url::parse(value).is_ok() {
      value.to_string()
    } else {
      base_url
        .join(value)
        .map(|url| url.to_string())
        .unwrap_or_else(|_| value.to_string())
    }
  }

  fn rewrite_srcset(srcset: &str, base_url: &Url) -> String {
    srcset
      .split(',')
      .map(|candidate| {
        let candidate = candidate.trim();

        if candidate.is_empty() {
          return String::new();
        }

        let mut parts = candidate.split_whitespace();

        let Some(url_part) = parts.next() else {
          return candidate.to_string();
        };

        let descriptor = parts.collect::<Vec<_>>().join(" ");
        let resolved = Self::resolve_uri(base_url, url_part);

        if descriptor.is_empty() {
          resolved
        } else {
          format!("{resolved} {descriptor}")
        }
      })
      .filter(|candidate| !candidate.is_empty())
      .collect::<Vec<_>>()
      .join(", ")
  }

  fn enforce_void_self_closing(markup: String) -> String {
    const BR_PLACEHOLDER: &str = "__readability_br_placeholder__";

    let intermediate = markup
      .replace("<br />", BR_PLACEHOLDER)
      .replace("<br>", "<br />")
      .replace(BR_PLACEHOLDER, "<br />");

    let mut result = String::with_capacity(intermediate.len());
    let mut remainder = intermediate.as_str();

    while let Some(idx) = remainder.find("<img") {
      let (before, after) = remainder.split_at(idx);
      result.push_str(before);

      if let Some(end) = after.find('>') {
        let (tag, rest) = after.split_at(end + 1);
        if tag.trim_end().ends_with("/>") {
          result.push_str(tag);
        } else {
          let trimmed = tag.trim_end_matches('>');
          result.push_str(trimmed);
          result.push_str(" />");
        }
        remainder = rest;
      } else {
        result.push_str(after);
        remainder = "";
        break;
      }
    }

    result.push_str(remainder);
    result
  }

  fn normalize_whitespace_nodes(fragment: &mut Html, root_id: NodeId) {
    let Some(root) = fragment.tree.get(root_id) else {
      return;
    };

    let text_nodes: Vec<NodeId> = root
      .descendants()
      .filter(|node| matches!(node.value(), Node::Text(_)))
      .map(|node| node.id())
      .collect();

    for node_id in text_nodes {
      let Some(node_ref) = fragment.tree.get(node_id) else {
        continue;
      };

      if Self::is_preserved_whitespace_context(node_ref) {
        continue;
      }

      let Some(mut node_mut) = fragment.tree.get_mut(node_id) else {
        continue;
      };

      let Node::Text(text_node) = node_mut.value() else {
        continue;
      };

      let original = text_node.to_string();

      if original.trim().is_empty() {
        continue;
      }

      let mut normalized = String::with_capacity(original.len());
      let mut last_was_space = false;

      for ch in original.chars() {
        match ch {
          '\n' | '\r' | '\t' => {
            if !last_was_space {
              normalized.push(' ');
              last_was_space = true;
            }
          }
          ' ' => {
            if !last_was_space {
              normalized.push(' ');
              last_was_space = true;
            }
          }
          ch => {
            normalized.push(ch);
            last_was_space = ch == ' ';
          }
        }
      }

      if normalized != original {
        text_node.text.clear();
        text_node.text.push_slice(&normalized);
      }
    }
  }

  fn is_preserved_whitespace_context(mut node: NodeRef<'_, Node>) -> bool {
    while let Some(parent) = node.parent() {
      if let Node::Element(element) = parent.value() {
        match element.name() {
          "pre" | "code" | "textarea" | "script" | "style" => return true,
          _ => {}
        }
      }

      node = parent;
    }

    false
  }

  fn serialize_children(node: NodeRef<'_, Node>) -> String {
    let opts = SerializeOpts {
      scripting_enabled: false,
      traversal_scope: TraversalScope::ChildrenOnly(None),
      create_missing_parent: false,
    };

    let mut buffer = Vec::new();

    let serializer = SerializableNode { node };

    if serialize(&mut buffer, &serializer, opts).is_ok() {
      String::from_utf8(buffer).unwrap_or_default()
    } else {
      String::new()
    }
  }
}

struct SerializableNode<'a> {
  node: NodeRef<'a, Node>,
}

impl<'a> Serialize for SerializableNode<'a> {
  fn serialize<S: Serializer>(
    &self,
    serializer: &mut S,
    traversal_scope: TraversalScope,
  ) -> std::io::Result<()> {
    serialize_node(self.node, serializer, traversal_scope)
  }
}

fn serialize_node<S: Serializer>(
  root: NodeRef<'_, Node>,
  serializer: &mut S,
  traversal_scope: TraversalScope,
) -> std::io::Result<()> {
  for edge in root.traverse() {
    match edge {
      Edge::Open(node) => {
        if node == root && traversal_scope == TraversalScope::ChildrenOnly(None)
        {
          continue;
        }

        match node.value() {
          Node::Doctype(doctype) => serializer.write_doctype(doctype.name())?,
          Node::Comment(comment) => serializer.write_comment(comment)?,
          Node::Text(text) => serializer.write_text(text)?,
          Node::Element(element) => {
            let attrs =
              element.attrs.iter().map(|(name, value)| (name, &value[..]));
            serializer.start_elem(element.name.clone(), attrs)?;
          }
          _ => {}
        }
      }
      Edge::Close(node) => {
        if node == root && traversal_scope == TraversalScope::ChildrenOnly(None)
        {
          continue;
        }

        if let Some(element) = node.value().as_element() {
          serializer.end_elem(element.name.clone())?;
        }
      }
    }
  }

  Ok(())
}
