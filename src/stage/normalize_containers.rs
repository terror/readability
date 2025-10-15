use super::*;

pub struct NormalizeContainersStage;

impl Stage for NormalizeContainersStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let html = context.html_mut();
    Self::normalize_containers(html);
    Ok(())
  }
}

impl NormalizeContainersStage {
  fn has_block_child(node: NodeRef<'_, Node>) -> bool {
    let mut child = node.first_child();

    while let Some(current) = child {
      let has_block = match current.value() {
        Node::Element(element) => {
          Self::is_block_element(element.name())
            || Self::has_block_child(current)
        }
        _ => false,
      };

      if has_block {
        return true;
      }

      child = current.next_sibling();
    }

    false
  }

  fn is_block_element(tag: &str) -> bool {
    matches!(
      tag,
      "blockquote" | "dl" | "div" | "img" | "ol" | "p" | "pre" | "table" | "ul"
    )
  }

  fn is_empty_container(node: NodeRef<'_, Node>) -> bool {
    let Node::Element(element) = node.value() else {
      return false;
    };

    let mut has_text = false;
    let mut allowed_children_only = true;

    for descendant in node.children() {
      match descendant.value() {
        Node::Text(text) => {
          if !text.trim().is_empty() {
            has_text = true;
            break;
          }
        }
        Node::Element(child_element) => {
          if !matches!(child_element.name(), "br" | "hr") {
            allowed_children_only = false;
            break;
          }
        }
        _ => {}
      }
    }

    if has_text {
      return false;
    }

    if !allowed_children_only {
      return false;
    }

    matches!(
      element.name(),
      "div" | "section" | "header" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
    )
  }

  fn normalize_containers(html: &mut Html) {
    let mut to_convert = Vec::new();

    for node in html.tree.root().descendants() {
      let Node::Element(element) = node.value() else {
        continue;
      };

      match element.name() {
        "div" if !Self::has_block_child(node) => {
          to_convert.push((node.id(), "p"));
        }
        "section" | "header" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
          if Self::is_empty_container(node) =>
        {
          to_convert.push((node.id(), "div"));
        }
        _ => {}
      }
    }

    for (id, tag) in to_convert {
      if let Some(mut node) = html.tree.get_mut(id)
        && let Node::Element(element) = node.value()
      {
        Self::set_element_tag(element, tag);
      }
    }
  }

  fn set_element_tag(element: &mut Element, tag: &str) {
    element.name = QualName::new(None, ns!(html), LocalName::from(tag));
  }
}
