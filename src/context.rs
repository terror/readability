use super::*;

pub(crate) struct Context<'a> {
  pub(crate) dir: Option<String>,
  pub(crate) document: &'a mut dom_query::Document,
  pub(crate) extraction: Extraction,
  pub(crate) flags: Flags,
  pub(crate) lang: Option<String>,
  pub(crate) metadata: Metadata,
  pub(crate) options: &'a ReadabilityOptions,
  pub(crate) url: Option<Url>,
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
      dir: None,
      document: html,
      extraction: Extraction::default(),
      flags: Flags::default(),
      lang: None,
      metadata: Metadata::default(),
      options,
      url: None,
    }
  }

  pub(crate) fn node(&self, id: NodeId) -> NodeRef<'_> {
    self.document.tree.get_unchecked(&id)
  }

  pub(crate) fn options(&self) -> &ReadabilityOptions {
    self.options
  }
}
