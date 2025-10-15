use super::*;

pub struct FixRelativeUrisStage<'a> {
  base_url: Option<&'a Url>,
}

impl Stage for FixRelativeUrisStage<'_> {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(base_url) = self.base_url else {
      return Ok(());
    };

    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    Self::fix_relative_uris(fragment, base_url);

    Ok(())
  }
}

impl<'a> FixRelativeUrisStage<'a> {
  fn find_attribute_index(element: &Element, name: &str) -> Option<usize> {
    element
      .attrs
      .iter()
      .position(|(attr_name, _)| attr_name.local.as_ref() == name)
  }

  fn fix_relative_uris(fragment: &mut ArticleFragment, base_url: &Url) {
    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return;
    };

    let node_ids: Vec<NodeId> = root
      .descendants()
      .filter(|node| matches!(node.value(), Node::Element(_)))
      .map(|node| node.id())
      .collect();

    for node_id in node_ids {
      let Some(mut node) = fragment.html.tree.get_mut(node_id) else {
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

  fn is_javascript_uri(value: &str) -> bool {
    value
      .trim_start()
      .to_ascii_lowercase()
      .starts_with("javascript:")
  }

  pub fn new(base_url: Option<&'a Url>) -> Self {
    Self { base_url }
  }

  fn resolve_uri(base_url: &Url, value: &str) -> String {
    if value.is_empty() {
      return value.to_string();
    }

    match Url::parse(value) {
      Ok(url) => url.to_string(),
      Err(_) => base_url
        .join(value)
        .map_or_else(|_| value.to_string(), |url| url.to_string()),
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
