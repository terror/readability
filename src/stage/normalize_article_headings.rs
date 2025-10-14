use super::*;

/// Normalizes heading levels within the extracted article fragment.
///
/// The original Readability implementation replaces `<h1>` elements inside the
/// article content with `<h2>` so that the standalone article title can remain
/// the only `<h1>` on the page. We replicate that behavior to stay aligned with
/// upstream expectations.
pub struct NormalizeArticleHeadingsStage;

impl Stage for NormalizeArticleHeadingsStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return Ok(());
    };

    let heading_ids: Vec<NodeId> = root
      .descendants()
      .filter(|node| {
        matches!(
          node.value(),
          Node::Element(element) if element.name() == "h1"
        )
      })
      .map(|node| node.id())
      .collect();

    for heading_id in heading_ids {
      let Some(mut node) = fragment.html.tree.get_mut(heading_id) else {
        continue;
      };

      let Node::Element(element) = node.value() else {
        continue;
      };

      element.name = QualName::new(None, ns!(html), LocalName::from("h2"));
    }

    Ok(())
  }
}
