use super::*;

pub(crate) struct ExtractLang;

impl Stage for ExtractLang {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    context.lang = context
      .document()
      .attribute("html", "lang")
      .map(|lang| lang.trim().to_string())
      .filter(|lang| !lang.is_empty());

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn extracts_lang_from_html_element() {
    Test::new()
      .stage(ExtractLang)
      .document(r#"<html lang="en"><head></head><body></body></html>"#)
      .expected_lang(Some("en"))
      .run();
  }

  #[test]
  fn returns_none_when_no_lang() {
    Test::new()
      .stage(ExtractLang)
      .document(r"<html><head></head><body></body></html>")
      .expected_lang(None)
      .run();
  }

  #[test]
  fn returns_none_when_lang_empty() {
    Test::new()
      .stage(ExtractLang)
      .document(r#"<html lang=""><head></head><body></body></html>"#)
      .expected_lang(None)
      .run();
  }

  #[test]
  fn extracts_xml_lang() {
    Test::new()
      .stage(ExtractLang)
      .document(r#"<html xml:lang="fr"><head></head><body></body></html>"#)
      .expected_lang(None)
      .run();
  }

  #[test]
  fn extracts_lang_with_xmlns() {
    Test::new()
      .stage(ExtractLang)
      .document(
        r#"<html lang="en" xmlns="http://www.w3.org/1999/xhtml" xml:lang="en"><head></head><body></body></html>"#,
      )
      .expected_lang(Some("en"))
      .run();
  }
}
