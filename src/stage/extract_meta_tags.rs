use super::*;
use regex::Regex;

pub(crate) struct ExtractMetaTags;

impl Stage for ExtractMetaTags {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let values = Self::collect_meta_values(context.document);

    let metadata = &mut context.metadata;

    if metadata.title.is_none() {
      metadata.title = Self::extract_title(&values, context.document);
    }

    if metadata.byline.is_none() {
      metadata.byline = Self::extract_byline(&values);
    }

    if metadata.excerpt.is_none() {
      metadata.excerpt = Self::extract_excerpt(&values);
    }

    if metadata.site_name.is_none() {
      metadata.site_name = values.get("og:site_name").cloned();
    }

    if metadata.published_time.is_none() {
      metadata.published_time = Self::extract_published_time(&values);
    }

    Ok(())
  }
}

impl ExtractMetaTags {
  fn collect_meta_values(
    document: &dom_query::Document,
  ) -> std::collections::HashMap<String, String> {
    let mut values = std::collections::HashMap::new();

    for meta in document.select("meta").nodes().to_vec() {
      let content = match meta.attr("content") {
        Some(content) if !content.trim().is_empty() => {
          content.trim().to_string()
        }
        _ => continue,
      };

      if let Some(property) = meta.attr("property") {
        for key in Self::match_property(property.as_ref()) {
          values.entry(key).or_insert_with(|| content.clone());
        }
      }

      if let Some(name) = meta.attr("name")
        && let Some(key) = Self::match_name(name.as_ref())
      {
        values.entry(key).or_insert_with(|| content.clone());
      }
    }

    values
  }

  fn extract_article_title(document: &dom_query::Document) -> Option<String> {
    TitleExtractor::new(document)
      .extract(document.select("title").first().text().trim())
  }

  fn extract_byline(
    values: &std::collections::HashMap<String, String>,
  ) -> Option<String> {
    let article_author = values
      .get("article:author")
      .filter(|v| !Self::is_url(v))
      .cloned();

    values
      .get("dc:creator")
      .or_else(|| values.get("dcterm:creator"))
      .or_else(|| values.get("author"))
      .or_else(|| values.get("parsely-author"))
      .cloned()
      .or(article_author)
  }

  fn extract_excerpt(
    values: &std::collections::HashMap<String, String>,
  ) -> Option<String> {
    values
      .get("dc:description")
      .or_else(|| values.get("dcterm:description"))
      .or_else(|| values.get("og:description"))
      .or_else(|| values.get("weibo:article:description"))
      .or_else(|| values.get("weibo:webpage:description"))
      .or_else(|| values.get("description"))
      .or_else(|| values.get("twitter:description"))
      .cloned()
  }

  fn extract_published_time(
    values: &std::collections::HashMap<String, String>,
  ) -> Option<String> {
    values
      .get("article:published_time")
      .or_else(|| values.get("parsely-pub-date"))
      .cloned()
  }

  fn extract_title(
    values: &std::collections::HashMap<String, String>,
    document: &dom_query::Document,
  ) -> Option<String> {
    let title = values
      .get("dc:title")
      .or_else(|| values.get("dcterm:title"))
      .or_else(|| values.get("og:title"))
      .or_else(|| values.get("weibo:article:title"))
      .or_else(|| values.get("weibo:webpage:title"))
      .or_else(|| values.get("title"))
      .or_else(|| values.get("twitter:title"))
      .or_else(|| values.get("parsely-title"))
      .cloned();

    title.or_else(|| Self::extract_article_title(document))
  }

  fn is_url(s: &str) -> bool {
    Url::parse(s).is_ok()
  }

  fn match_name(name: &str) -> Option<String> {
    let pattern = Regex::new(
      r"(?ix)
      ^\s*
      (?:(dc|dcterm|og|twitter|parsely|weibo:(?:article|webpage))\s*[-\.:]\s*)?
      (author|creator|pub-date|description|title|site_name)
      \s*$",
    )
    .unwrap();

    if pattern.is_match(name) {
      Some(
        name
          .to_lowercase()
          .chars()
          .filter(|c| !c.is_whitespace())
          .collect::<String>()
          .replace('.', ":"),
      )
    } else {
      None
    }
  }

  fn match_property(property: &str) -> Vec<String> {
    let pattern =
      Regex::new(r"(?i)\s*(article|dc|dcterm|og|twitter)\s*:\s*(author|creator|description|published_time|title|site_name)\s*")
        .unwrap();

    pattern
      .find_iter(property)
      .map(|m| {
        m.as_str()
          .to_lowercase()
          .chars()
          .filter(|c: &char| !c.is_whitespace())
          .collect::<String>()
      })
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn run(content: &str) -> Metadata {
    let mut document = dom_query::Document::from(content);
    let options = ReadabilityOptions::default();
    let mut context = Context::new(&mut document, &options);
    ExtractJsonLd.run(&mut context).unwrap();
    ExtractMetaTags.run(&mut context).unwrap();
    context.metadata
  }

  #[test]
  fn og_title() {
    assert_eq!(
      run(
        r#"<html><head><meta property="og:title" content="foo"/></head><body></body></html>"#
      ),
      Metadata {
        title: Some("foo".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn og_description() {
    assert_eq!(
      run(
        r#"<html><head><meta property="og:description" content="foo"/></head><body></body></html>"#
      ),
      Metadata {
        excerpt: Some("foo".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn og_site_name() {
    assert_eq!(
      run(
        r#"<html><head><meta property="og:site_name" content="foo"/></head><body></body></html>"#
      ),
      Metadata {
        site_name: Some("foo".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn article_published_time() {
    assert_eq!(
      run(
        r#"<html><head><meta property="article:published_time" content="2024-01-01"/></head><body></body></html>"#,
      ),
      Metadata {
        published_time: Some("2024-01-01".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn article_author_url_ignored() {
    assert_eq!(
      run(
        r#"<html><head><meta property="article:author" content="https://example.com/author"/></head><body></body></html>"#,
      ),
      Metadata::default()
    );
  }

  #[test]
  fn article_author_non_url_used() {
    assert_eq!(
      run(
        r#"<html><head><meta property="article:author" content="foo bar"/></head><body></body></html>"#,
      ),
      Metadata {
        byline: Some("foo bar".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn json_ld_title_takes_priority() {
    assert_eq!(
      run(
        r#"<html><head>
          <script type="application/ld+json">{"@context":"https://schema.org","@type":"Article","name":"foo"}</script>
          <meta property="og:title" content="bar"/>
        </head><body></body></html>"#,
      ),
      Metadata {
        title: Some("foo".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn meta_fills_gap_when_no_json_ld() {
    assert_eq!(
      run(
        r#"<html><head>
          <script type="application/ld+json">{"@context":"https://schema.org","@type":"Article","name":"foo"}</script>
          <meta property="og:description" content="bar"/>
        </head><body></body></html>"#,
      ),
      Metadata {
        title: Some("foo".into()),
        excerpt: Some("bar".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn author_meta() {
    assert_eq!(
      run(
        r#"<html><head><meta name="author" content="foo"/></head><body></body></html>"#
      ),
      Metadata {
        byline: Some("foo".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn dc_creator() {
    assert_eq!(
      run(
        r#"<html><head><meta name="dc.creator" content="foo"/></head><body></body></html>"#
      ),
      Metadata {
        byline: Some("foo".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn title_strips_site_name_suffix() {
    assert_eq!(
      run(
        r#"<html><head><title>foo bar baz qux quux | site name</title></head><body></body></html>"#,
      ),
      Metadata {
        title: Some("foo bar baz qux quux".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn title_strips_colon_suffix() {
    assert_eq!(
      run(
        r#"<html><head><title>site: foo bar baz qux</title></head><body></body></html>"#,
      ),
      Metadata {
        title: Some("foo bar baz qux".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn title_uses_h1_when_too_short() {
    assert_eq!(
      run(
        r#"<html><head><title>hi</title></head><body><h1>foo bar</h1></body></html>"#,
      ),
      Metadata {
        title: Some("foo bar".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn twitter_title_fallback() {
    assert_eq!(
      run(
        r#"<html><head><meta name="twitter:title" content="foo"/></head><body></body></html>"#,
      ),
      Metadata {
        title: Some("foo".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn parsely_pub_date() {
    assert_eq!(
      run(
        r#"<html><head><meta name="parsely-pub-date" content="2024-06-01"/></head><body></body></html>"#,
      ),
      Metadata {
        published_time: Some("2024-06-01".into()),
        ..Metadata::default()
      }
    );
  }

  #[test]
  fn empty_content_ignored() {
    assert_eq!(
      run(
        r#"<html><head><meta property="og:title" content=""/></head><body></body></html>"#,
      ),
      Metadata::default()
    );
  }

  #[test]
  fn dc_title_preferred_over_og_title() {
    assert_eq!(
      run(
        r#"<html><head>
          <meta name="dc.title" content="foo"/>
          <meta property="og:title" content="bar"/>
        </head><body></body></html>"#,
      ),
      Metadata {
        title: Some("foo".into()),
        ..Metadata::default()
      }
    );
  }
}
