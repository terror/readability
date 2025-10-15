use super::*;

/// Rewrites legacy `<center>` elements to `<div>` to match browser behavior.
pub struct RewriteCenterTagsStage;

impl Stage for RewriteCenterTagsStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let html = context.html_mut();

    let centers = html
      .tree
      .root()
      .descendants()
      .filter(|node| {
        matches!(
          node.value(),
          Node::Element(element) if element.name() == "center"
        )
      })
      .map(|node| node.id())
      .collect::<Vec<NodeId>>();

    for id in centers {
      let Some(mut node) = html.tree.get_mut(id) else {
        continue;
      };

      if let Node::Element(element) = node.value() {
        element.name = QualName::new(None, ns!(html), LocalName::from("div"));
      }
    }

    Ok(())
  }
}
