use super::*;

pub(crate) struct Document<'a> {
  document: &'a mut dom_query::Document,
}

impl<'a> Document<'a> {
  pub(crate) fn attribute(&self, selector: &str, name: &str) -> Option<String> {
    self
      .document
      .select(selector)
      .nodes()
      .first()
      .and_then(|node| node.attr(name))
      .map(|value| value.to_string())
  }

  pub(crate) fn element_count(&self) -> usize {
    self
      .document
      .root()
      .descendants()
      .into_iter()
      .filter(NodeRef::is_element)
      .count()
  }

  pub(crate) fn new(document: &'a mut dom_query::Document) -> Self {
    Document { document }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn attribute_returns_value() {
    let mut document = dom_query::Document::from(
      r#"<html lang=" en "><head></head><body></body></html>"#,
    );

    let document = Document::new(&mut document);

    assert_eq!(document.attribute("html", "lang").as_deref(), Some(" en "));
  }

  #[test]
  fn attribute_returns_none_when_missing() {
    let mut document =
      dom_query::Document::from("<html><head></head><body></body></html>");

    let document = Document::new(&mut document);

    assert_eq!(document.attribute("html", "lang"), None);
  }

  #[test]
  fn counts_element_nodes_only_once() {
    let mut document = dom_query::Document::from(
      r#"
      <html>
        <head><meta charset="utf-8" /></head>
        <body>
          <div>
            <p>One</p>
            <span>Two</span>
          </div>
          <img src="image.png" />
        </body>
      </html>
      "#,
    );

    let document = Document::new(&mut document);

    assert_eq!(document.element_count(), 8);
  }
}
