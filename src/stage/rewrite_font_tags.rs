use super::*;

/// Rewrites deprecated `<font>` elements into semantic-neutral `<span>` tags.
///
/// This preserves document content and hierarchy while normalizing obsolete
/// markup into a tag form that later cleanup and scoring stages can process
/// consistently.
pub(crate) struct RewriteFontTags;

impl Stage for RewriteFontTags {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    context.document.select("font").rename("span");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn converts_font_to_span() {
    Test::new()
      .stage(RewriteFontTags)
      .document("<html><body><font>Hello</font></body></html>")
      .expected_html(
        "<html><head></head><body><span>Hello</span></body></html>",
      )
      .run();
  }

  #[test]
  fn preserves_font_tag_content() {
    Test::new()
      .stage(RewriteFontTags)
      .document("<html><body><font>Hello <b>world</b></font></body></html>")
      .expected_html(
        "<html><head></head><body><span>Hello <b>world</b></span></body></html>",
      )
      .run();
  }

  #[test]
  fn converts_multiple_font_tags() {
    Test::new()
      .stage(RewriteFontTags)
      .document(
        "<html><body><font>One</font><p>Middle</p><font>Two</font></body></html>",
      )
      .expected_html(
        "<html><head></head><body><span>One</span><p>Middle</p><span>Two</span></body></html>",
      )
      .run();
  }

  #[test]
  fn handles_nested_font_tags() {
    Test::new()
      .stage(RewriteFontTags)
      .document(
        "<html><body><font>Outer <font>Inner</font></font></body></html>",
      )
      .expected_html(
        "<html><head></head><body><span>Outer <span>Inner</span></span></body></html>",
      )
      .run();
  }
}
