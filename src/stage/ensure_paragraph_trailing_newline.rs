use super::*;

/// Ensures paragraphs that span multiple lines end with a newline before closing.
pub struct EnsureParagraphTrailingNewlineStage;

impl Stage for EnsureParagraphTrailingNewlineStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    Self::append_newlines(fragment);

    Ok(())
  }
}

impl EnsureParagraphTrailingNewlineStage {
  fn append_newlines(fragment: &mut ArticleFragment) {
    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return;
    };

    let paragraph_ids: Vec<NodeId> = root
      .descendants()
      .filter(
        |node| matches!(node.value(), Node::Element(el) if el.name() == "p"),
      )
      .map(|node| node.id())
      .collect();

    for paragraph_id in paragraph_ids {
      let Some(paragraph) = fragment.html.tree.get(paragraph_id) else {
        continue;
      };

      let Some(element_ref) = ElementRef::wrap(paragraph) else {
        continue;
      };

      let inner_html = element_ref.html();

      if !inner_html.contains('\n') {
        continue;
      }

      let has_trailing_newline = paragraph.last_child().is_some_and(
        |last| matches!(last.value(), Node::Text(text) if text.ends_with('\n')),
      );

      if has_trailing_newline {
        continue;
      }

      if let Some(mut paragraph_mut) = fragment.html.tree.get_mut(paragraph_id)
      {
        paragraph_mut
          .append(Node::Text(scraper::node::Text { text: "\n".into() }));
      }
    }
  }
}
