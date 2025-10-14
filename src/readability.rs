use super::*;

static REGEX_NORMALIZE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\s{2,}").unwrap());

pub struct Readability {
  article_dir: Option<String>,
  base_url: Option<Url>,
  html: Html,
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
      article_dir: None,
      base_url,
      html: Html::parse_document(html),
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

    let markup = context
      .take_article_markup()
      .ok_or(Error::MissingArticleContent)?;

    let title = context
      .metadata()
      .title
      .clone()
      .filter(|value| !value.is_empty())
      .or(context.document().document_title())
      .unwrap_or(String::new());

    let lang = context
      .body_lang()
      .cloned()
      .or(context.document_lang().cloned());

    let fragment = Html::parse_fragment(&markup);

    let mut text = String::new();

    for node in fragment.tree.root().descendants() {
      if let Node::Text(value) = node.value() {
        if !text.is_empty() {
          text.push(' ');
        }

        text.push_str(value.trim());
      }
    }

    let text_content = REGEX_NORMALIZE
      .replace_all(&text, " ")
      .into_owned()
      .trim()
      .to_string();

    let selector = Selector::parse("p")
      .map_err(|error| Error::InvalidSelector(error.to_string()))?;

    let fragment = Html::parse_fragment(&markup);

    let first_paragraph = fragment
      .select(&selector)
      .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
      .find(|text| !text.is_empty());

    let excerpt = context.metadata().excerpt.clone().or(first_paragraph);

    Ok(Article {
      title,
      byline: context.metadata().byline.clone(),
      dir: self.article_dir.clone(),
      lang,
      content: markup,
      text_content: text_content.clone(),
      length: text_content.chars().count(),
      excerpt,
      site_name: context.metadata().site_name.clone(),
      published_time: context.metadata().published_time.clone(),
    })
  }
}
