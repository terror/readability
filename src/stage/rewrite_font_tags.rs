use super::*;

pub struct RewriteFontTags;

impl Stage for RewriteFontTags {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    context.document().rename_elements("font", "span");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn converts_font_to_span() {
    let mut document =
      dom_query::Document::from("<html><body><font>Hello</font></body></html>");

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    RewriteFontTags.run(&mut context).unwrap();

    assert_eq!(document.select("font").length(), 0);
    assert_eq!(document.select("span").length(), 1);
  }

  #[test]
  fn preserves_font_tag_content() {
    let mut document = dom_query::Document::from(
      "<html><body><font>Hello <b>world</b></font></body></html>",
    );

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    RewriteFontTags.run(&mut context).unwrap();

    assert_eq!(
      document.select("span").html().to_string(),
      "<span>Hello <b>world</b></span>"
    );
  }

  #[test]
  fn converts_multiple_font_tags() {
    let mut document = dom_query::Document::from(
      "<html><body><font>One</font><p>Middle</p><font>Two</font></body></html>",
    );

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    RewriteFontTags.run(&mut context).unwrap();

    assert_eq!(document.select("font").length(), 0);
    assert_eq!(document.select("p").length(), 1);
    assert_eq!(document.select("span").length(), 2);
  }

  #[test]
  fn handles_nested_font_tags() {
    let mut document = dom_query::Document::from(
      "<html><body><font>Outer <font>Inner</font></font></body></html>",
    );

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    RewriteFontTags.run(&mut context).unwrap();

    assert_eq!(document.select("font").length(), 0);
    assert_eq!(document.select("span").length(), 2);
  }
}
