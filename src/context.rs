use super::*;

pub(crate) struct Context<'a> {
  pub(crate) document: &'a mut dom_query::Document,
  pub(crate) metadata: Metadata,
  pub(crate) options: &'a ReadabilityOptions,
}

impl<'a> Context<'a> {
  pub(crate) fn document(&mut self) -> Document<'_> {
    Document::new(&mut *self.document)
  }

  pub(crate) fn new(
    html: &'a mut dom_query::Document,
    options: &'a ReadabilityOptions,
  ) -> Self {
    Self {
      document: html,
      metadata: Metadata::default(),
      options,
    }
  }

  pub(crate) fn options(&self) -> &ReadabilityOptions {
    self.options
  }
}
