use super::*;

pub(crate) struct CleanHeaders;

impl Stage for CleanHeaders {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(article) = context.article else {
      return Ok(());
    };

    if !context.flags.weight_classes {
      return Ok(());
    }

    for header in context
      .node(article)
      .elements_with_tags(&["H1", "H2"])
      .iter()
      .rev()
    {
      if header.class_weight() < 0.0 {
        header.remove_from_parent();
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_negative_weight_headers() {
    Test::new()
      .stage(CleanHeaders)
      .article()
      .document(
        r#"<html><body><h1 class="comment">foo</h1><h2>bar</h2></body></html>"#,
      )
      .expected_html("<html><head></head><body><h2>bar</h2></body></html>")
      .run();
  }
}
