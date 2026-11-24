use super::*;

pub(crate) struct RemoveComments;

impl Stage for RemoveComments {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    for node in context.document.root().descendants() {
      if node.is_comment() {
        node.remove_from_parent();
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_comments() {
    Test::new()
      .stage(RemoveComments)
      .document("<html><body><!-- foo --><p>bar<!-- baz --></p></body></html>")
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }
}
