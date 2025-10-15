use super::*;

pub struct RewriteFontTagsStage;

impl Stage for RewriteFontTagsStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let html = context.html_mut();

    let to_rewrite = html
      .tree
      .root()
      .descendants()
      .filter(|node| {
        matches!(
          node.value(),
          Node::Element(element) if element.name() == "font"
        )
      })
      .map(|node| node.id())
      .collect::<Vec<NodeId>>();

    for id in to_rewrite {
      let Some(mut node) = html.tree.get_mut(id) else {
        continue;
      };

      if let Node::Element(element) = node.value() {
        element.name = QualName::new(None, ns!(html), LocalName::from("span"));
      }
    }

    Ok(())
  }
}
