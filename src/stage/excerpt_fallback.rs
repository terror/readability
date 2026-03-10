use super::*;

/// Falls back to the first non-empty paragraph's text as the excerpt when no
/// excerpt has been extracted from metadata.
pub(crate) struct ExtractExcerpt;

impl Stage for ExtractExcerpt {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    if context.metadata.excerpt.is_some() {
      return Ok(());
    }

    context.metadata.excerpt = context
      .document
      .select("p")
      .nodes()
      .iter()
      .find_map(|node| {
        let text = node.text();

        let trimmed = text.trim();

        if trimmed.is_empty() {
          None
        } else {
          Some(trimmed.to_string())
        }
      });

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn fallback_from_first_paragraph() {
    Test::new()
      .stage(ExcerptFallback)
      .document("<html><body><p>foo</p><p>bar</p></body></html>")
      .expected_metadata(Metadata {
        excerpt: Some("foo".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn skips_empty_paragraphs() {
    Test::new()
      .stage(ExcerptFallback)
      .document("<html><body><p>   </p><p>bar</p></body></html>")
      .expected_metadata(Metadata {
        excerpt: Some("bar".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn no_paragraphs_leaves_excerpt_none() {
    Test::new()
      .stage(ExcerptFallback)
      .document("<html><body><div>foo</div></body></html>")
      .expected_metadata(Metadata::default())
      .run();
  }

  #[test]
  fn existing_excerpt_not_overwritten() {
    Test::new()
      .stage(ExcerptFallback)
      .document("<html><body><p>bar</p></body></html>")
      .metadata(Metadata {
        excerpt: Some("foo".into()),
        ..Metadata::default()
      })
      .expected_metadata(Metadata {
        excerpt: Some("foo".into()),
        ..Metadata::default()
      })
      .run();
  }
}
