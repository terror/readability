use super::*;

pub(crate) struct RemoveEmptyParagraphs;

impl Stage for RemoveEmptyParagraphs {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(article) = context.extraction.article else {
      return Ok(());
    };

    for paragraph in context
      .node(article)
      .elements_with_tags(&["P"])
      .iter()
      .rev()
    {
      if paragraph
        .elements_with_tags(&["IMG", "EMBED", "OBJECT", "IFRAME"])
        .is_empty()
        && paragraph.text().trim().is_empty()
      {
        paragraph.remove_from_parent();
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_empty_paragraphs() {
    Test::new()
      .stage(RemoveEmptyParagraphs)
      .article()
      .document("<html><body><p> </p><p><img></p><p>foo</p></body></html>")
      .expected_html(
        "<html><head></head><body><p><img></p><p>foo</p></body></html>",
      )
      .run();
  }
}
