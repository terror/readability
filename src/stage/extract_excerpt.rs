use super::*;

/// Falls back to the text of the article's first paragraph as the excerpt when
/// metadata did not provide a non-empty one.
pub(crate) struct ExtractExcerpt;

impl Stage for ExtractExcerpt {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    if context
      .metadata
      .excerpt
      .as_ref()
      .is_some_and(|excerpt| !excerpt.is_empty())
    {
      return Ok(());
    }

    let Some(article) = context.extraction.article else {
      return Ok(());
    };

    let paragraph = context
      .node(article)
      .descendant_nodes()
      .into_iter()
      .find(|node| node.tag() == "P")
      .map(|node| node.text().trim().to_string());

    if let Some(paragraph) = paragraph {
      context.metadata.excerpt = Some(paragraph);
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[track_caller]
  fn excerpt(html: &str, metadata: Metadata) -> Option<String> {
    let mut document = dom_query::Document::from(html);

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    context.metadata = metadata;

    SelectTopCandidate.run(&mut context).unwrap();
    ExtractArticle.run(&mut context).unwrap();
    ExtractExcerpt.run(&mut context).unwrap();

    context.metadata.excerpt
  }

  #[test]
  fn first_article_paragraph() {
    assert_eq!(
      excerpt(
        "<html><body><div> foo </div><p> bar </p><p>baz</p></body></html>",
        Metadata::default(),
      )
      .as_deref(),
      Some("bar"),
    );
  }

  #[test]
  fn no_paragraphs_leaves_excerpt_none() {
    assert_eq!(
      excerpt(
        "<html><body><div>foo</div></body></html>",
        Metadata::default()
      ),
      None,
    );
  }

  #[test]
  fn existing_excerpt_not_overwritten() {
    assert_eq!(
      excerpt(
        "<html><body><p>bar</p></body></html>",
        Metadata {
          excerpt: Some("foo".into()),
          ..Metadata::default()
        },
      )
      .as_deref(),
      Some("foo"),
    );
  }

  #[test]
  fn empty_excerpt_overwritten() {
    assert_eq!(
      excerpt(
        "<html><body><p>bar</p></body></html>",
        Metadata {
          excerpt: Some(String::new()),
          ..Metadata::default()
        },
      )
      .as_deref(),
      Some("bar"),
    );
  }
}
