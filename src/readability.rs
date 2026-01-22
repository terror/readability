use super::*;

pub struct Readability {
  base_url: Option<Url>,
  html: dom_query::Document,
  options: ReadabilityOptions,
}

impl Readability {
  /// Creates a new readability parser instance.
  ///
  /// # Errors
  ///
  /// Returns an error when the optional `base_url` cannot be parsed.
  pub fn new(
    html: &str,
    base_url: Option<&str>,
    options: ReadabilityOptions,
  ) -> Result<Self> {
    let base_url = base_url
      .map(|value| Url::parse(value).map_err(Error::from))
      .transpose()?;

    Ok(Self {
      base_url,
      html: dom_query::Document::from(html),
      options,
    })
  }

  /// Extracts the article contents using the configured pipeline.
  ///
  /// # Errors
  ///
  /// Returns an error when the pipeline cannot resolve article content.
  pub fn parse(&mut self) -> Result<Article> {
    let context = Pipeline::with_default_stages(
      Context::new(&mut self.html, &self.options),
      self.base_url.as_ref(),
    )
    .run()?;

    let Metadata {
      byline,
      excerpt,
      published_time,
      site_name,
      title,
    } = context.metadata;

    Ok(Article {
      byline,
      content: context.document.html().to_string(),
      dir: None,
      excerpt,
      lang: None,
      length: context.document.text().to_string().len(),
      published_time,
      site_name,
      text_content: context.document.text().to_string(),
      title: title.unwrap_or_default(),
    })
  }
}
