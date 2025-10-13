use super::*;

pub struct SanitizationStage;

impl Stage for SanitizationStage {
  fn run(&mut self, ctx: &mut Context<'_>) -> Result<()> {
    let html = ctx.html_mut();

    let mut to_remove = Vec::new();

    for node in html.tree.root().descendants() {
      if let Node::Element(element) = node.value() {
        match element.name() {
          "script" | "noscript" | "style" => to_remove.push(node.id()),
          _ => {}
        }
      }
    }

    for id in to_remove {
      if let Some(mut node) = html.tree.get_mut(id) {
        node.detach();
      }
    }

    Ok(())
  }
}
