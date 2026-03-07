use super::*;

/// Removes nodes that are never useful for readable article extraction.
///
/// This stage deletes all `script`, `style`, and `noscript` elements from the
/// parsed document so later stages do not need to account for executable code,
/// stylesheet content, or fallback markup.
pub(crate) struct RemoveDisallowedNodes;

impl Stage for RemoveDisallowedNodes {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    context.document.select("script, style, noscript").remove();

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_script_tags() {
    Test::new()
      .stage(RemoveDisallowedNodes)
      .document(
        "<html><body><script>alert('hi');</script><p>Content</p></body></html>",
      )
      .expected_html("<html><head></head><body><p>Content</p></body></html>")
      .run();
  }

  #[test]
  fn removes_style_tags() {
    Test::new()
      .stage(RemoveDisallowedNodes)
      .document("<html><head><style>body { color: red; }</style></head><body><p>Content</p></body></html>")
      .expected_html("<html><head></head><body><p>Content</p></body></html>")
      .run();
  }

  #[test]
  fn removes_noscript_tags() {
    Test::new()
      .stage(RemoveDisallowedNodes)
      .document("<html><body><noscript>Enable JS</noscript><p>Content</p></body></html>")
      .expected_html("<html><head></head><body><p>Content</p></body></html>")
      .run();
  }
}
