use super::*;

static REGEX_NORMALIZE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\s{2,}").unwrap());

pub struct Readability {
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

    let fragment = Html::parse_fragment(&markup);

    let text_content = REGEX_NORMALIZE
      .replace_all(
        &fragment
          .tree
          .root()
          .descendants()
          .filter_map(|node| match node.value() {
            Node::Text(value) => Some(value.trim()),
            _ => None,
          })
          .collect::<Vec<_>>()
          .join(" "),
        " ",
      )
      .trim()
      .to_string();

    let first_paragraph = fragment
      .select(
        &Selector::parse("p")
          .map_err(|error| Error::InvalidSelector(error.to_string()))?,
      )
      .map(|element| {
        element
          .text()
          .collect::<Vec<_>>()
          .join(" ")
          .trim()
          .to_string()
      })
      .find(|text| !text.is_empty());

    Ok(Article {
      title: context.metadata().title.clone().unwrap_or(String::new()),
      byline: context.metadata().byline.clone(),
      dir: context.article_dir().cloned(),
      lang: context
        .body_lang()
        .cloned()
        .or(context.document_lang().cloned()),
      content: markup,
      text_content: text_content.clone(),
      length: text_content.chars().count(),
      excerpt: context.metadata().excerpt.clone().or(first_paragraph),
      site_name: context.metadata().site_name.clone(),
      published_time: context.metadata().published_time.clone(),
    })
  }
}
