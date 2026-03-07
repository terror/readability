use super::*;

const BYLINE_KEYS: &[&str] =
  &["dc:creator", "dcterm:creator", "author", "parsely-author"];

const EXCERPT_KEYS: &[&str] = &[
  "dc:description",
  "dcterm:description",
  "og:description",
  "weibo:article:description",
  "weibo:webpage:description",
  "description",
  "twitter:description",
];

const PUBLISHED_TIME_KEYS: &[&str] =
  &["article:published_time", "parsely-pub-date"];

const SITE_NAME_KEYS: &[&str] = &["og:site_name"];

const TITLE_KEYS: &[&str] = &[
  "dc:title",
  "dcterm:title",
  "og:title",
  "weibo:article:title",
  "weibo:webpage:title",
  "title",
  "twitter:title",
  "parsely-title",
];

pub(crate) struct ExtractMetaTags;

impl Stage for ExtractMetaTags {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let values = Self::collect_meta_values(context.document);

    let title = Self::extract_title(&values, context.document);

    let metadata = std::mem::take(&mut context.metadata);

    context.metadata = Metadata {
      title: metadata.title.or(title),
      byline: metadata.byline.or_else(|| Self::extract_byline(&values)),
      excerpt: metadata
        .excerpt
        .or_else(|| Self::first_value(&values, EXCERPT_KEYS)),
      site_name: metadata
        .site_name
        .or_else(|| Self::first_value(&values, SITE_NAME_KEYS)),
      published_time: metadata
        .published_time
        .or_else(|| Self::first_value(&values, PUBLISHED_TIME_KEYS)),
    };

    Ok(())
  }
}

impl ExtractMetaTags {
  fn collect_meta_values(
    document: &dom_query::Document,
  ) -> HashMap<String, String> {
    let mut values = HashMap::new();

    for meta in document.select("meta").nodes().to_vec() {
      let content = match meta.attr("content") {
        Some(content) if !content.trim().is_empty() => {
          content.trim().to_string()
        }
        _ => continue,
      };

      if let Some(property) = meta.attr("property") {
        for token in property.split_whitespace() {
          values
            .entry(Self::normalize_key(token))
            .or_insert_with(|| content.clone());
        }
      }

      if let Some(name) = meta.attr("name") {
        values
          .entry(Self::normalize_key(name.as_ref()).replace('.', ":"))
          .or_insert_with(|| content.clone());
      }
    }

    values
  }

  fn extract_article_title(document: &dom_query::Document) -> Option<String> {
    TitleExtractor::new(document)
      .extract(document.select("title").first().text().trim())
  }

  fn extract_byline(values: &HashMap<String, String>) -> Option<String> {
    let article_author = values
      .get("article:author")
      .filter(|value| !Self::is_url(value))
      .cloned();

    Self::first_value(values, BYLINE_KEYS).or(article_author)
  }

  fn extract_title(
    values: &HashMap<String, String>,
    document: &dom_query::Document,
  ) -> Option<String> {
    Self::first_value(values, TITLE_KEYS)
      .or_else(|| Self::extract_article_title(document))
  }

  fn first_value(
    values: &HashMap<String, String>,
    keys: &[&str],
  ) -> Option<String> {
    keys.iter().find_map(|key| values.get(*key).cloned())
  }

  fn is_url(s: &str) -> bool {
    Url::parse(s).is_ok()
  }

  fn normalize_key(s: &str) -> String {
    s.to_lowercase()
      .chars()
      .filter(|c| !c.is_whitespace())
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test(content: &str) -> Test {
    Test::new()
      .stage(ExtractJsonLd)
      .stage(ExtractMetaTags)
      .document(content)
  }

  #[test]
  fn og_title() {
    test(
      r#"<html><head><meta property="og:title" content="foo"/></head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      title: Some("foo".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn og_description() {
    test(
      r#"<html><head><meta property="og:description" content="foo"/></head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      excerpt: Some("foo".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn og_site_name() {
    test(
      r#"<html><head><meta property="og:site_name" content="foo"/></head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      site_name: Some("foo".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn article_published_time() {
    test(
      r#"<html><head><meta property="article:published_time" content="2024-01-01"/></head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      published_time: Some("2024-01-01".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn article_author_url_ignored() {
    test(
      r#"<html><head><meta property="article:author" content="https://example.com/author"/></head><body></body></html>"#,
    )
    .expected_metadata(Metadata::default())
    .run();
  }

  #[test]
  fn article_author_non_url_used() {
    test(
      r#"<html><head><meta property="article:author" content="foo bar"/></head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      byline: Some("foo bar".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn json_ld_title_takes_priority() {
    test(
      r#"<html><head>
          <script type="application/ld+json">{"@context":"https://schema.org","@type":"Article","name":"foo"}</script>
          <meta property="og:title" content="bar"/>
        </head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      title: Some("foo".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn meta_fills_gap_when_no_json_ld() {
    test(
      r#"<html><head>
          <script type="application/ld+json">{"@context":"https://schema.org","@type":"Article","name":"foo"}</script>
          <meta property="og:description" content="bar"/>
        </head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      title: Some("foo".into()),
      excerpt: Some("bar".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn author_meta() {
    test(
      r#"<html><head><meta name="author" content="foo"/></head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      byline: Some("foo".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn dc_creator() {
    test(
      r#"<html><head><meta name="dc.creator" content="foo"/></head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      byline: Some("foo".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn title_strips_site_name_suffix() {
    test(
      r"<html><head><title>foo bar baz qux quux | site name</title></head><body></body></html>",
    )
    .expected_metadata(Metadata {
      title: Some("foo bar baz qux quux".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn title_strips_colon_suffix() {
    test(
      r"<html><head><title>site: foo bar baz qux</title></head><body></body></html>",
    )
    .expected_metadata(Metadata {
      title: Some("foo bar baz qux".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn title_uses_h1_when_too_short() {
    test(
      r"<html><head><title>hi</title></head><body><h1>foo bar</h1></body></html>",
    )
    .expected_metadata(Metadata {
      title: Some("foo bar".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn twitter_title_fallback() {
    test(
      r#"<html><head><meta name="twitter:title" content="foo"/></head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      title: Some("foo".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn parsely_pub_date() {
    test(
      r#"<html><head><meta name="parsely-pub-date" content="2024-06-01"/></head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      published_time: Some("2024-06-01".into()),
      ..Metadata::default()
    })
    .run();
  }

  #[test]
  fn empty_content_ignored() {
    test(
      r#"<html><head><meta property="og:title" content=""/></head><body></body></html>"#,
    )
    .expected_metadata(Metadata::default())
    .run();
  }

  #[test]
  fn dc_title_preferred_over_og_title() {
    test(
      r#"<html><head>
          <meta name="dc.title" content="foo"/>
          <meta property="og:title" content="bar"/>
        </head><body></body></html>"#,
    )
    .expected_metadata(Metadata {
      title: Some("foo".into()),
      ..Metadata::default()
    })
    .run();
  }
}
