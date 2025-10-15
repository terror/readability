use super::*;

pub struct ReplaceBreakSequencesStage;

impl Stage for ReplaceBreakSequencesStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let html = context.html_mut();
    Self::replace_break_sequences(html);
    Ok(())
  }
}

impl ReplaceBreakSequencesStage {
  fn convert_br_chain(html: &mut Html, br_id: NodeId) {
    let Some(br_ref) = html.tree.get(br_id) else {
      return;
    };

    let mut removal_ids = vec![br_id];
    let mut next = br_ref.next_sibling();

    while let Some(node) = next {
      if Self::is_whitespace_text(&node) || Self::is_br_element(&node) {
        removal_ids.push(node.id());
        next = node.next_sibling();
        continue;
      }

      break;
    }

    let mut nodes_to_move = Vec::new();
    let mut cursor = next;

    while let Some(node) = cursor {
      if Self::is_br_element(&node) {
        break;
      }

      if Self::is_phrasing_node(&node) || Self::is_whitespace_text(&node) {
        nodes_to_move.push(node.id());
        cursor = node.next_sibling();
      } else {
        break;
      }
    }

    if nodes_to_move.is_empty() && removal_ids.len() <= 1 {
      return;
    }

    let paragraph_id = {
      let Some(mut br_node) = html.tree.get_mut(br_id) else {
        return;
      };

      let new_id = {
        let paragraph = br_node.insert_before(Self::create_element("p"));
        paragraph.id()
      };

      br_node.detach();

      new_id
    };

    for removal_id in removal_ids.into_iter().skip(1) {
      if let Some(mut node) = html.tree.get_mut(removal_id) {
        node.detach();
      }
    }

    let valid_nodes: Vec<NodeId> = nodes_to_move
      .into_iter()
      .filter(|node_id| html.tree.get(*node_id).is_some())
      .collect();

    if let Some(mut paragraph) = html.tree.get_mut(paragraph_id) {
      for node_id in valid_nodes {
        paragraph.append_id(node_id);
      }
    }

    Self::trim_whitespace(html, paragraph_id);

    if let Some(parent_id) = html
      .tree
      .get(paragraph_id)
      .and_then(|node| node.parent().map(|parent| parent.id()))
      && let Some(mut parent) = html.tree.get_mut(parent_id)
      && let Node::Element(element) = parent.value()
      && element.name() == "p"
    {
      Self::set_element_tag(element, "div");
    }
  }

  fn create_element(tag: &str) -> Node {
    Node::Element(Element::new(
      QualName::new(None, ns!(html), LocalName::from(tag)),
      Vec::new(),
    ))
  }

  fn is_br_element(node: &NodeRef<'_, Node>) -> bool {
    matches!(node.value(), Node::Element(element) if element.name() == "br")
  }

  fn is_break_chain_start(node: NodeRef<'_, Node>) -> bool {
    let mut next = node.next_sibling();

    while let Some(sibling) = next {
      if Self::is_whitespace_text(&sibling) {
        next = sibling.next_sibling();
        continue;
      }

      return Self::is_br_element(&sibling);
    }

    false
  }

  fn is_phrasing_element(tag: &str) -> bool {
    matches!(
      tag,
      "abbr"
        | "audio"
        | "b"
        | "bdi"
        | "bdo"
        | "br"
        | "button"
        | "cite"
        | "code"
        | "data"
        | "datalist"
        | "dfn"
        | "em"
        | "embed"
        | "i"
        | "img"
        | "input"
        | "kbd"
        | "label"
        | "mark"
        | "math"
        | "meter"
        | "noscript"
        | "object"
        | "output"
        | "progress"
        | "q"
        | "ruby"
        | "samp"
        | "script"
        | "select"
        | "small"
        | "span"
        | "strong"
        | "sub"
        | "sup"
        | "textarea"
        | "time"
        | "var"
        | "wbr"
    )
  }

  fn is_phrasing_node(node: &NodeRef<'_, Node>) -> bool {
    match node.value() {
      Node::Text(_) => true,
      Node::Element(element) => Self::is_phrasing_element(element.name()),
      _ => false,
    }
  }

  fn is_whitespace_text(node: &NodeRef<'_, Node>) -> bool {
    matches!(
      node.value(),
      Node::Text(text) if text.trim().is_empty()
    )
  }

  fn node_mut_is_whitespace_text(
    node: &mut ego_tree::NodeMut<'_, Node>,
  ) -> bool {
    matches!(
      node.value(),
      Node::Text(text) if text.trim().is_empty()
    )
  }

  fn replace_break_sequences(html: &mut Html) {
    let mut chain_starts = Vec::new();

    for node in html.tree.root().descendants() {
      if Self::is_br_element(&node) && Self::is_break_chain_start(node) {
        chain_starts.push(node.id());
      }
    }

    for id in chain_starts {
      Self::convert_br_chain(html, id);
    }
  }

  fn set_element_tag(element: &mut Element, tag: &str) {
    element.name = QualName::new(None, ns!(html), LocalName::from(tag));
  }

  fn trim_whitespace(html: &mut Html, para_id: NodeId) {
    if let Some(mut paragraph) = html.tree.get_mut(para_id) {
      while let Some(mut child) = paragraph.first_child() {
        if Self::node_mut_is_whitespace_text(&mut child) {
          child.detach();
        } else {
          break;
        }
      }

      while let Some(mut child) = paragraph.last_child() {
        if Self::node_mut_is_whitespace_text(&mut child) {
          child.detach();
        } else {
          break;
        }
      }
    }
  }
}
