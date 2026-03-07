use super::*;

pub(crate) struct ExtractDir;

impl Stage for ExtractDir {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    context.dir = ["body", "html"].iter().find_map(|selector| {
      context
        .document
        .select(selector)
        .nodes()
        .first()
        .and_then(|node| node.attr("dir"))
        .map(|dir| dir.trim().to_string())
        .filter(|dir| !dir.is_empty())
    });

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn run(content: &str) -> Option<String> {
    let mut document = dom_query::Document::from(content);
    let options = ReadabilityOptions::default();
    let mut context = Context::new(&mut document, &options);
    ExtractDir.run(&mut context).unwrap();
    context.dir
  }

  #[test]
  fn extracts_dir_from_html_element() {
    assert_eq!(
      run(r#"<html dir="rtl"><head></head><body></body></html>"#),
      Some("rtl".into())
    );
  }

  #[test]
  fn extracts_dir_from_body_before_html() {
    assert_eq!(
      run(r#"<html dir="ltr"><head></head><body dir="rtl"></body></html>"#),
      Some("rtl".into())
    );
  }

  #[test]
  fn returns_none_when_no_dir() {
    assert_eq!(run(r"<html><head></head><body></body></html>"), None);
  }

  #[test]
  fn returns_none_when_dir_empty() {
    assert_eq!(
      run(r#"<html dir=""><head></head><body></body></html>"#),
      None
    );
  }
}
