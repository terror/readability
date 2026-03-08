use super::*;

pub(crate) struct ExtractDir;

impl Stage for ExtractDir {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    context.dir = ["body", "html"].iter().find_map(|selector| {
      context
        .document()
        .attribute(selector, "dir")
        .map(|dir| dir.trim().to_string())
        .filter(|dir| !dir.is_empty())
    });

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn extracts_dir_from_html_element() {
    Test::new()
      .stage(ExtractDir)
      .document(r#"<html dir="rtl"><head></head><body></body></html>"#)
      .expected_dir(Some("rtl"))
      .run();
  }

  #[test]
  fn extracts_dir_from_body_before_html() {
    Test::new()
      .stage(ExtractDir)
      .document(
        r#"<html dir="ltr"><head></head><body dir="rtl"></body></html>"#,
      )
      .expected_dir(Some("rtl"))
      .run();
  }

  #[test]
  fn returns_none_when_no_dir() {
    Test::new()
      .stage(ExtractDir)
      .document(r"<html><head></head><body></body></html>")
      .expected_dir(None)
      .run();
  }

  #[test]
  fn returns_none_when_dir_empty() {
    Test::new()
      .stage(ExtractDir)
      .document(r#"<html dir=""><head></head><body></body></html>"#)
      .expected_dir(None)
      .run();
  }
}
