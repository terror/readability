use super::*;

const CLASSES_TO_PRESERVE: &[&str] = &["page"];

fn parse_fragment(markup: &str) -> (Html, NodeId) {
  let fragment = Html::parse_fragment(markup);

  let target_id = fragment
    .tree
    .root()
    .descendants()
    .find(|node| {
      matches!(node.value(), Node::Element(element) if element.name() == "body")
    })
    .map_or_else(|| fragment.tree.root().id(), |node| node.id());

  (fragment, target_id)
}

fn extract_markup(fragment: &Html, target_id: NodeId) -> Option<String> {
  let Some(target) = fragment.tree.get(target_id) else {
    return None;
  };

  Some(serialize_children(target))
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

pub struct FixRelativeUrisStage<'a> {
  base_url: Option<&'a Url>,
}

impl Stage for FixRelativeUrisStage<'_> {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(markup) = context.take_article_markup() else {
      return Ok(());
    };

    let processed = self.process_markup(markup);

    context.set_article_markup(processed);

    Ok(())
  }
}

impl<'a> FixRelativeUrisStage<'a> {
  pub fn new(base_url: Option<&'a Url>) -> Self {
    Self { base_url }
  }

  fn process_markup(&self, markup: String) -> String {
    let original = markup;

    let Some(base_url) = self.base_url else {
      return original;
    };

    let (mut fragment, target_id) = parse_fragment(&original);

    Self::fix_relative_uris(&mut fragment, target_id, base_url);

    match extract_markup(&fragment, target_id) {
      Some(processed) if !processed.is_empty() => processed,
      _ => original,
    }
  }

  fn fix_relative_uris(fragment: &mut Html, root_id: NodeId, base_url: &Url) {
    let Some(root) = fragment.tree.get(root_id) else {
      return;
    };

    let node_ids: Vec<NodeId> = root
      .descendants()
      .filter(|node| matches!(node.value(), Node::Element(_)))
      .map(|node| node.id())
      .collect();

    for node_id in node_ids {
      let Some(mut node) = fragment.tree.get_mut(node_id) else {
        continue;
      };

      let Node::Element(element) = node.value() else {
        continue;
      };

      if let Some(index) = Self::find_attribute_index(element, "href") {
        let href_value = element.attrs[index].1.to_string();

        if !href_value.starts_with('#') && !Self::is_javascript_uri(&href_value)
        {
          let resolved = Self::resolve_uri(base_url, &href_value);
          element.attrs[index].1.clear();
          element.attrs[index].1.push_slice(&resolved);
        }
      }

      if let Some(index) = Self::find_attribute_index(element, "src") {
        let src_value = element.attrs[index].1.to_string();
        let resolved = Self::resolve_uri(base_url, &src_value);
        element.attrs[index].1.clear();
        element.attrs[index].1.push_slice(&resolved);
      }

      if let Some(index) = Self::find_attribute_index(element, "poster") {
        let poster_value = element.attrs[index].1.to_string();
        let resolved = Self::resolve_uri(base_url, &poster_value);
        element.attrs[index].1.clear();
        element.attrs[index].1.push_slice(&resolved);
      }

      if let Some(index) = Self::find_attribute_index(element, "srcset") {
        let srcset_value = element.attrs[index].1.to_string();
        let resolved = Self::rewrite_srcset(&srcset_value, base_url);
        element.attrs[index].1.clear();
        element.attrs[index].1.push_slice(&resolved);
      }
    }
  }

  fn find_attribute_index(element: &Element, name: &str) -> Option<usize> {
    element
      .attrs
      .iter()
      .position(|(attr_name, _)| attr_name.local.as_ref() == name)
  }

  fn is_javascript_uri(value: &str) -> bool {
    value
      .trim_start()
      .to_ascii_lowercase()
      .starts_with("javascript:")
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
        .map_or_else(|_| value.to_string(), |url| url.to_string())
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
}

pub struct CleanClassAttributesStage;

impl CleanClassAttributesStage {
  fn process_markup(&self, markup: String) -> String {
    let original = markup;
    let (mut fragment, target_id) = parse_fragment(&original);

    Self::clean_classes(&mut fragment, target_id);

    match extract_markup(&fragment, target_id) {
      Some(processed) if !processed.is_empty() => processed,
      _ => original,
    }
  }

  fn clean_classes(fragment: &mut Html, root_id: NodeId) {
    let Some(root) = fragment.tree.get(root_id) else {
      return;
    };

    let node_ids: Vec<NodeId> = root
      .descendants()
      .filter(|node| matches!(node.value(), Node::Element(_)))
      .map(|node| node.id())
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
}

impl Stage for CleanClassAttributesStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(markup) = context.take_article_markup() else {
      return Ok(());
    };

    let processed = self.process_markup(markup);
    context.set_article_markup(processed);

    Ok(())
  }
}

pub struct NormalizeArticleWhitespaceStage;

impl NormalizeArticleWhitespaceStage {
  fn process_markup(&self, markup: String) -> String {
    let original = markup;
    let (mut fragment, target_id) = parse_fragment(&original);

    Self::normalize_whitespace_nodes(&mut fragment, target_id);

    match extract_markup(&fragment, target_id) {
      Some(processed) if !processed.is_empty() => processed,
      _ => original,
    }
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
          '\n' | '\r' | '\t' | ' ' => {
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
}

impl Stage for NormalizeArticleWhitespaceStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(markup) = context.take_article_markup() else {
      return Ok(());
    };

    let processed = self.process_markup(markup);
    context.set_article_markup(processed);

    Ok(())
  }
}

pub struct FinalizeArticleMarkupStage;

impl FinalizeArticleMarkupStage {
  fn finalize_markup(&self, markup: String) -> String {
    let original = markup;
    let (fragment, target_id) = parse_fragment(&original);

    if let Ok(selector) = Selector::parse("#readability-page-1")
      && let Some(element) = fragment.select(&selector).next()
    {
      let inner = element.inner_html();
      let markup =
        format!("<div id=\"readability-page-1\" class=\"page\">{inner}</div>");

      return Self::enforce_void_self_closing(markup);
    }

    match extract_markup(&fragment, target_id) {
      Some(processed) if !processed.is_empty() => {
        Self::enforce_void_self_closing(processed)
      }
      _ => original,
    }
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
}

impl Stage for FinalizeArticleMarkupStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(markup) = context.take_article_markup() else {
      return Ok(());
    };

    let processed = self.finalize_markup(markup);
    context.set_article_markup(processed);

    Ok(())
  }
}
