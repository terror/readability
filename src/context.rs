use super::*;

#[derive(Debug)]
pub(crate) struct Context<'a> {
  article_dir: Option<String>,
  article_fragment: Option<ArticleFragment>,
  article_markup: Option<String>,
  body_lang: Option<String>,
  document_lang: Option<String>,
  html: &'a mut Html,
  metadata: Metadata,
  options: &'a ReadabilityOptions,
}

impl<'a> Context<'a> {
  pub(crate) fn article_dir(&self) -> Option<&String> {
    self.article_dir.as_ref()
  }

  pub(crate) fn article_fragment_mut(
    &mut self,
  ) -> Option<&mut ArticleFragment> {
    self.article_fragment.as_mut()
  }

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

  pub(crate) fn metadata(&self) -> &Metadata {
    &self.metadata
  }

  pub(crate) fn new(
    html: &'a mut Html,
    options: &'a ReadabilityOptions,
  ) -> Self {
    Self {
      article_fragment: None,
      article_dir: None,
      html,
      options,
      metadata: Metadata::default(),
      document_lang: None,
      body_lang: None,
      article_markup: None,
    }
  }

  pub(crate) fn options(&self) -> &ReadabilityOptions {
    self.options
  }

  pub(crate) fn set_article_dir(&mut self, dir: Option<String>) {
    self.article_dir = dir;
  }

  pub(crate) fn set_article_fragment(&mut self, fragment: ArticleFragment) {
    self.article_fragment = Some(fragment);
    self.article_markup = None;
  }

  pub(crate) fn set_article_markup(&mut self, markup: String) {
    self.article_markup = Some(markup);
    self.article_fragment = None;
  }

  pub(crate) fn set_body_lang(&mut self, lang: Option<String>) {
    self.body_lang = lang;
  }

  pub(crate) fn set_document_lang(&mut self, lang: Option<String>) {
    self.document_lang = lang;
  }

  pub(crate) fn set_metadata(&mut self, metadata: Metadata) {
    self.metadata = metadata;
  }

  pub(crate) fn take_article_markup(&mut self) -> Option<String> {
    match self.article_markup.take() {
      Some(markup) => Some(markup),
      None => self
        .article_fragment
        .take()
        .and_then(ArticleFragment::into_markup),
    }
  }
}
