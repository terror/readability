use super::*;

/// Normalizes the outer container element for the extracted article.
pub struct NormalizeArticleRootStage;

impl Stage for NormalizeArticleRootStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    Self::normalize_root(fragment);

    Ok(())
  }
}

impl NormalizeArticleRootStage {
  fn normalize_root(fragment: &mut ArticleFragment) {
    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return;
    };

    let direct_children: Vec<NodeId> = root
      .children()
      .filter(|node| matches!(node.value(), Node::Element(_)))
      .map(|node| node.id())
      .collect();

    for child_id in direct_children {
      if let Some(mut child) = fragment.html.tree.get_mut(child_id)
        && let Node::Element(element) = child.value()
        && element.name() == "main"
      {
        element.name = QualName::new(None, ns!(html), LocalName::from("div"));
      }
    }
  }
}
