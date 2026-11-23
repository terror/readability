use super::*;

pub(crate) struct RenameHeadings;

impl Stage for RenameHeadings {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(article) = context.extraction.article else {
      return Ok(());
    };

    for heading in context.node(article).elements_with_tags(&["H1"]) {
      heading.set_tag("h2");
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn renames_h1_to_h2() {
    Test::new()
      .stage(RenameHeadings)
      .article()
      .document(r#"<html><body><h1 class="foo">bar</h1></body></html>"#)
      .expected_html(
        r#"<html><head></head><body><h2 class="foo">bar</h2></body></html>"#,
      )
      .run();
  }
}
