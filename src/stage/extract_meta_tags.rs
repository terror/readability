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

    let metadata = mem::take(&mut context.metadata);

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
          if let Some(property) = META_PROPERTY.find(token) {
            let key = property
              .as_str()
              .to_lowercase()
              .replace(|c: char| c.is_whitespace(), "");

            values.insert(key, content.clone());
          }
        }
      }

      if let Some(name) = meta.attr("name") {
        values.insert(
          Self::normalize_key(name.as_ref()).replace('.', ":"),
          content.clone(),
        );
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

  #[test]
  fn og_title() {
    Test::new()
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractMetaTags)
      .document(
        r#"<html><head><meta property="article:author" content="https://example.com/author"/></head><body></body></html>"#,
      )
      .expected_metadata(Metadata::default())
      .run();
  }

  #[test]
  fn article_author_non_url_used() {
    Test::new()
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractJsonLd)
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractJsonLd)
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractJsonLd)
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractJsonLd)
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractJsonLd)
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractJsonLd)
      .stage(ExtractMetaTags)
      .document(
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
    Test::new()
      .stage(ExtractJsonLd)
      .stage(ExtractMetaTags)
      .document(
        r#"<html><head><meta property="og:title" content=""/></head><body></body></html>"#,
      )
      .expected_metadata(Metadata::default())
      .run();
  }

  #[test]
  fn dc_title_preferred_over_og_title() {
    Test::new()
      .stage(ExtractJsonLd)
      .stage(ExtractMetaTags)
      .document(
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
