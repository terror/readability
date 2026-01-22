use super::*;

pub(crate) struct Document<'a> {
  document: &'a mut dom_query::Document,
}

impl<'a> Document<'a> {
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

  pub(crate) fn remove_elements(&mut self, selector: &str) {
    self.document.select(selector).remove();
  }

  pub(crate) fn rename_elements(&mut self, selector: &str, new_name: &str) {
    self.document.select(selector).rename(new_name);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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
