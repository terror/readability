use super::*;

pub(crate) struct Context<'a> {
  html: &'a mut dom_query::Document,
  metadata: Metadata,
  options: &'a ReadabilityOptions,
}

impl<'a> Context<'a> {
  pub(crate) fn document(&mut self) -> Document<'_> {
    Document::new(&mut *self.html)
  }

  pub(crate) fn metadata(&mut self) -> Metadata {
    mem::take(&mut self.metadata)
  }

  pub(crate) fn new(
    html: &'a mut dom_query::Document,
    options: &'a ReadabilityOptions,
  ) -> Self {
    Self {
      html,
      metadata: Metadata::default(),
      options,
    }
  }

  pub(crate) fn options(&self) -> &ReadabilityOptions {
    self.options
  }
}
