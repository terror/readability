use super::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct CollectedMetadata {
  pub(crate) byline: Option<String>,
  pub(crate) excerpt: Option<String>,
  pub(crate) published_time: Option<String>,
  pub(crate) site_name: Option<String>,
  pub(crate) title: Option<String>,
}

#[derive(Debug)]
pub(crate) struct Context<'a> {
  article_markup: Option<String>,
  body_lang: Option<String>,
  document_lang: Option<String>,
  html: &'a mut Html,
  metadata: CollectedMetadata,
  options: &'a ReadabilityOptions,
}

impl<'a> Context<'a> {
  pub(crate) fn body_lang(&self) -> Option<&String> {
    self.body_lang.as_ref()
  }

  pub(crate) fn document(&self) -> Document<'_> {
    Document::new(&*self.html)
  }

  pub(crate) fn document_lang(&self) -> Option<&String> {
    self.document_lang.as_ref()
  }

  pub(crate) fn html_mut(&mut self) -> &mut Html {
    self.html
  }

  pub(crate) fn metadata(&self) -> &CollectedMetadata {
    &self.metadata
  }

  pub(crate) fn new(
    html: &'a mut Html,
    options: &'a ReadabilityOptions,
  ) -> Self {
    Self {
      html,
      options,
      metadata: CollectedMetadata::default(),
      document_lang: None,
      body_lang: None,
      article_markup: None,
    }
  }

  pub(crate) fn options(&self) -> &ReadabilityOptions {
    self.options
  }

  pub(crate) fn set_article_markup(&mut self, markup: String) {
    self.article_markup = Some(markup);
  }

  pub(crate) fn set_body_lang(&mut self, lang: Option<String>) {
    self.body_lang = lang;
  }

  pub(crate) fn set_document_lang(&mut self, lang: Option<String>) {
    self.document_lang = lang;
  }

  pub(crate) fn set_metadata(&mut self, metadata: CollectedMetadata) {
    self.metadata = metadata;
  }

  pub(crate) fn take_article_markup(&mut self) -> Option<String> {
    self.article_markup.take()
  }
}
