use super::*;

/// Normalizes the outer container element for the extracted article.
pub struct NormalizeArticleRootStage;

impl Stage for NormalizeArticleRootStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return Ok(());
    };

    let main_elements = root
      .children()
      .filter(|node| {
        matches!(
          node.value(),
          Node::Element(element) if element.name() == "main"
        )
      })
      .map(|node| node.id())
      .collect::<Vec<NodeId>>();

    for id in main_elements {
      let Some(mut node) = fragment.html.tree.get_mut(id) else {
        continue;
      };

      let Node::Element(element) = node.value() else {
        continue;
      };

      element.name = QualName::new(None, ns!(html), LocalName::from("div"));
    }

    Ok(())
  }
}
