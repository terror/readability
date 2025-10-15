use super::*;

const DISALLOWED_NODES: &[&str] = &["script", "noscript", "style"];

pub struct RemoveDisallowedNodesStage;

impl Stage for RemoveDisallowedNodesStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let html = context.html_mut();

    let to_remove = html
      .tree
      .root()
      .descendants()
      .filter(|node| {
        matches!(
          node.value(),
          Node::Element(element) if DISALLOWED_NODES.contains(&element.name())
        )
      })
      .map(|node| node.id())
      .collect::<Vec<NodeId>>();

    for id in to_remove {
      if let Some(mut node) = html.tree.get_mut(id) {
        node.detach();
      }
    }

    Ok(())
  }
}
