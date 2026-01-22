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
    let mut context = Pipeline::with_default_stages(
      Context::new(&mut self.html, &self.options),
      self.base_url.as_ref(),
    )
    .run()?;

    let Metadata {
      title,
      byline,
      excerpt,
      site_name,
      published_time,
    } = context.metadata();

    Ok(Article {
      title: title.unwrap_or_default(),
      byline,
      dir: None,
      lang: None,
      content: String::new(),
      text_content: String::new(),
      length: 0,
      excerpt,
      site_name,
      published_time,
    })
  }
}
