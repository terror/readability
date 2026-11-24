use super::*;

pub struct Parser {
  base_url: Option<Url>,
  html: dom_query::Document,
  options: ParserOptions,
}

impl Parser {
  #[must_use]
  pub fn is_readable(&self) -> bool {
    readable::is_readable(&self.html)
  }

  /// Creates a new parser instance.
  ///
  /// # Errors
  ///
  /// Returns an error when the optional `base_url` cannot be parsed.
  pub fn new(
    html: &str,
    base_url: Option<&str>,
    options: ParserOptions,
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
  /// Returns an error when the document has more elements than
  /// `max_elements` allows, or when no article content can be found.
  pub fn parse(mut self) -> Result<Article> {
    let mut context = Context {
      url: self.base_url.clone(),
      ..Context::new(&mut self.html, &self.options)
    };

    Pipeline::default().run(&mut context)?;

    let article = context
      .extraction
      .article
      .map(|id| context.document.tree.get_unchecked(&id))
      .ok_or(Error::MissingArticleContent)?;

    let content = article.inner_html().to_string();

    let text_content = article.text().to_string();

    let Metadata {
      byline,
      excerpt,
      published_time,
      site_name,
      title,
    } = context.metadata;

    Ok(Article {
      byline,
      content,
      dir: context.dir,
      excerpt,
      lang: context.lang,
      length: text_length(&text_content),
      published_time,
      site_name,
      text_content,
      title: title.unwrap_or_default(),
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn deeply_nested_elements() {
    let depth = 10_000;

    let html = format!(
      "<div>{}{}{}</div>",
      "<span>".repeat(depth),
      "foo ".repeat(200),
      "</span>".repeat(depth),
    );

    Parser::new(&html, None, ParserOptions::default())
      .unwrap()
      .parse()
      .unwrap();
  }
}
