use super::*;

const NON_CONTENT_ELEMENTS: &[&str] = &[
  "aside", "button", "embed", "fieldset", "footer", "form", "iframe", "input",
  "link", "object", "select", "textarea",
];

/// Removes elements that typically contain sharing controls or boilerplate.
pub struct RemoveNonContentElementsStage;

impl Stage for RemoveNonContentElementsStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return Ok(());
    };

    let nodes = root
      .descendants()
      .filter(|node| {
        matches!(
          node.value(),
          Node::Element(element)
            if NON_CONTENT_ELEMENTS.contains(&element.name())
        )
      })
      .map(|node| node.id())
      .collect::<Vec<NodeId>>();

    for node_id in nodes {
      if let Some(mut node) = fragment.html.tree.get_mut(node_id) {
        node.detach();
      }
    }

    Ok(())
  }
}
