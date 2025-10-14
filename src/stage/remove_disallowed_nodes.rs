use super::*;

pub struct RemoveDisallowedNodesStage;

impl Stage for RemoveDisallowedNodesStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let html = context.html_mut();
    Self::remove_disallowed_nodes(html);
    Ok(())
  }
}

impl RemoveDisallowedNodesStage {
  fn remove_disallowed_nodes(html: &mut Html) {
    let mut to_remove = Vec::new();

    for node in html.tree.root().descendants() {
      if matches!(
        node.value(),
        Node::Element(element)
          if matches!(element.name(), "script" | "noscript" | "style")
      ) {
        to_remove.push(node.id());
      }
    }

    for id in to_remove {
      if let Some(mut node) = html.tree.get_mut(id) {
        node.detach();
      }
    }
  }
}
