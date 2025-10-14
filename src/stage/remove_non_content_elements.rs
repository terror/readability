use super::*;

/// Removes elements that typically contain sharing controls or boilerplate.
pub struct RemoveNonContentElementsStage;

impl Stage for RemoveNonContentElementsStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    Self::remove(fragment);

    Ok(())
  }
}

impl RemoveNonContentElementsStage {
  const TAGS_TO_REMOVE: &'static [&'static str] = &[
    "aside", "button", "fieldset", "footer", "form", "iframe", "input", "link",
    "object", "select", "textarea",
  ];

  fn remove(fragment: &mut ArticleFragment) {
    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return;
    };

    let nodes: Vec<NodeId> = root
      .descendants()
      .filter(|node| {
        matches!(
          node.value(),
          Node::Element(element)
            if Self::TAGS_TO_REMOVE.contains(&element.name())
        )
      })
      .map(|node| node.id())
      .collect();

    for node_id in nodes {
      if let Some(mut node) = fragment.html.tree.get_mut(node_id) {
        node.detach();
      }
    }
  }
}
