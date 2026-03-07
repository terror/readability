use super::*;

pub(crate) struct ExtractLang;

impl Stage for ExtractLang {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let lang = context
      .document
      .select("html")
      .nodes()
      .first()
      .and_then(|node| node.attr("lang"))
      .map(|lang| lang.trim().to_string())
      .filter(|lang| !lang.is_empty());

    context.lang = lang;

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
    ExtractLang.run(&mut context).unwrap();
    context.lang
  }

  #[test]
  fn extracts_lang_from_html_element() {
    assert_eq!(
      run(r#"<html lang="en"><head></head><body></body></html>"#),
      Some("en".into())
    );
  }

  #[test]
  fn returns_none_when_no_lang() {
    assert_eq!(run(r#"<html><head></head><body></body></html>"#), None);
  }

  #[test]
  fn returns_none_when_lang_empty() {
    assert_eq!(
      run(r#"<html lang=""><head></head><body></body></html>"#),
      None
    );
  }

  #[test]
  fn extracts_xml_lang() {
    assert_eq!(
      run(r#"<html xml:lang="fr"><head></head><body></body></html>"#),
      None
    );
  }

  #[test]
  fn extracts_lang_with_xmlns() {
    assert_eq!(
      run(
        r#"<html lang="en" xmlns="http://www.w3.org/1999/xhtml" xml:lang="en"><head></head><body></body></html>"#
      ),
      Some("en".into())
    );
  }
}
