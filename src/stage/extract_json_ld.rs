use super::*;

const SCHEMA_ORG: &str = "schema.org";

const ARTICLE_TYPES: &[&str] = &[
  "Article",
  "AdvertiserContentArticle",
  "NewsArticle",
  "AnalysisNewsArticle",
  "AskPublicNewsArticle",
  "BackgroundNewsArticle",
  "OpinionNewsArticle",
  "ReportageNewsArticle",
  "ReviewNewsArticle",
  "Report",
  "SatiricalArticle",
  "ScholarlyArticle",
  "MedicalScholarlyArticle",
  "SocialMediaPosting",
  "BlogPosting",
  "LiveBlogPosting",
  "DiscussionForumPosting",
  "TechArticle",
  "APIReference",
];

pub(crate) struct ExtractJsonLd;

impl Stage for ExtractJsonLd {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let scripts = context
      .document
      .select("script[type='application/ld+json']")
      .nodes()
      .to_vec();

    for script in scripts {
      let text = script.text();

      let text = text.trim();

      let text = text
        .trim_start_matches("<![CDATA[")
        .trim_end_matches("]]>")
        .trim();

      let Ok(value) = serde_json::from_str::<serde_json::Value>(text) else {
        continue;
      };

      let Some(article) = Self::find_article(&value) else {
        continue;
      };

      context.metadata = Self::extract_metadata(article);

      break;
    }

    Ok(())
  }
}

impl ExtractJsonLd {
  fn extract_byline(article: &serde_json::Value) -> Option<String> {
    let author = article.get("author")?;

    if let Some(name) = author.get("name").and_then(|value| value.as_str()) {
      return Some(name.trim().to_owned());
    }

    if let Some(authors) = author.as_array() {
      let names = authors
        .iter()
        .filter_map(|author| {
          author.get("name").and_then(|value| value.as_str())
        })
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(", ");

      if !names.is_empty() {
        return Some(names);
      }
    }

    None
  }

  fn extract_metadata(article: &serde_json::Value) -> Metadata {
    let title = Self::extract_title(article);

    let byline = Self::extract_byline(article);

    let excerpt = article
      .get("description")
      .and_then(|value| value.as_str())
      .map(|string| string.trim().to_owned());

    let site_name = article
      .get("publisher")
      .and_then(|publish| publish.get("name"))
      .and_then(|value| value.as_str())
      .map(|string| string.trim().to_owned());

    let published_time = article
      .get("datePublished")
      .and_then(|value| value.as_str())
      .map(|string| string.trim().to_owned());

    Metadata {
      byline,
      excerpt,
      published_time,
      site_name,
      title,
    }
  }

  fn extract_title(article: &serde_json::Value) -> Option<String> {
    let name = article.get("name").and_then(|value| value.as_str());

    let headline = article.get("headline").and_then(|value| value.as_str());

    match (name, headline) {
      (Some(name), Some(headline)) if name != headline => {
        Some(name.trim().to_owned())
      }
      (Some(name), _) => Some(name.trim().to_owned()),
      (None, Some(headline)) => Some(headline.trim().to_owned()),
      (None, None) => None,
    }
  }

  fn find_article(value: &serde_json::Value) -> Option<&serde_json::Value> {
    let value = if let Some(array) = value.as_array() {
      array
        .iter()
        .find(|item| item.get("@type").is_some_and(Self::is_article_type))?
    } else {
      value
    };

    if !value.get("@context").is_some_and(Self::is_schema_org) {
      return None;
    }

    if let Some(graph) = value.get("@graph").and_then(|graph| graph.as_array())
      && value.get("@type").is_none()
    {
      return graph
        .iter()
        .find(|item| item.get("@type").is_some_and(Self::is_article_type));
    }

    if !value.get("@type").is_some_and(Self::is_article_type) {
      return None;
    }

    Some(value)
  }

  fn is_article_type(value: &serde_json::Value) -> bool {
    value
      .as_str()
      .is_some_and(|string| ARTICLE_TYPES.contains(&string))
  }

  fn is_schema_org(context: &serde_json::Value) -> bool {
    match context {
      serde_json::Value::String(string) => string.contains(SCHEMA_ORG),
      serde_json::Value::Object(object) => object
        .get("@vocab")
        .and_then(|value| value.as_str())
        .is_some_and(|string| string.contains(SCHEMA_ORG)),
      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn array_of_objects_picks_article() {
    Test::new()
      .stage(ExtractJsonLd)
      .document(
        r#"<html><head><script type="application/ld+json">
        [
          { "@context": "https://schema.org", "@type": "VideoObject", "name": "foo" },
          { "@context": "https://schema.org", "@type": "NewsArticle", "name": "bar" }
        ]
        </script></head><body></body></html>"#,
      )
      .expected_metadata(Metadata {
        title: Some("bar".to_string()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn cdata_stripped() {
    Test::new()
      .stage(ExtractJsonLd)
      .document(
        r#"<html><head><script type="application/ld+json">
        <![CDATA[
        {
          "@context": "https://schema.org",
          "@type": "Article",
          "name": "foo"
        }
        ]]>
        </script></head><body></body></html>"#,
      )
      .expected_metadata(Metadata {
        title: Some("foo".to_string()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn context_object_with_vocab() {
    Test::new()
      .stage(ExtractJsonLd)
      .document(
        r#"<html><head><script type="application/ld+json">
        {
          "@context": { "@vocab": "https://schema.org/" },
          "@type": "Article",
          "name": "foo"
        }
        </script></head><body></body></html>"#,
      )
      .expected_metadata(Metadata {
        title: Some("foo".to_string()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn extracts_article_fields() {
    Test::new()
      .stage(ExtractJsonLd)
      .document(
        r#"<html><head><script type="application/ld+json">
        {
          "@context": "https://schema.org",
          "@type": "NewsArticle",
          "name": "foo",
          "description": "bar",
          "publisher": { "name": "baz" },
          "datePublished": "2024-01-01",
          "author": { "name": "qux" }
        }
        </script></head><body></body></html>"#,
      )
      .expected_metadata(Metadata {
        title: Some("foo".to_string()),
        excerpt: Some("bar".to_string()),
        site_name: Some("baz".to_string()),
        published_time: Some("2024-01-01".to_string()),
        byline: Some("qux".to_string()),
      })
      .run();
  }

  #[test]
  fn graph_traversal() {
    Test::new()
      .stage(ExtractJsonLd)
      .document(
        r#"<html><head><script type="application/ld+json">
        {
          "@context": "https://schema.org",
          "@graph": [
            { "@type": "WebSite", "name": "foo" },
            { "@type": "Article", "name": "bar" }
          ]
        }
        </script></head><body></body></html>"#,
      )
      .expected_metadata(Metadata {
        title: Some("bar".to_string()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn headline_fallback() {
    Test::new()
      .stage(ExtractJsonLd)
      .document(
        r#"<html><head><script type="application/ld+json">
        {
          "@context": "https://schema.org",
          "@type": "Article",
          "headline": "foo"
        }
        </script></head><body></body></html>"#,
      )
      .expected_metadata(Metadata {
        title: Some("foo".to_string()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn ignores_non_article_type() {
    Test::new()
      .stage(ExtractJsonLd)
      .document(
        r#"<html><head><script type="application/ld+json">
        {
          "@context": "https://schema.org",
          "@type": "VideoObject",
          "name": "foo"
        }
        </script></head><body></body></html>"#,
      )
      .expected_metadata(Metadata::default())
      .run();
  }

  #[test]
  fn ignores_non_schema_org() {
    Test::new()
      .stage(ExtractJsonLd)
      .document(
        r#"<html><head><script type="application/ld+json">
        {
          "@context": "https://example.com",
          "@type": "Article",
          "name": "foo"
        }
        </script></head><body></body></html>"#,
      )
      .expected_metadata(Metadata::default())
      .run();
  }

  #[test]
  fn invalid_json_skipped() {
    Test::new()
      .stage(ExtractJsonLd)
      .document(
        r#"<html><head>
        <script type="application/ld+json">not json</script>
        <script type="application/ld+json">{"@context":"https://schema.org","@type":"Article","name":"foo"}</script>
        </head><body></body></html>"#,
      )
      .expected_metadata(Metadata {
        title: Some("foo".to_string()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn multiple_authors() {
    Test::new()
      .stage(ExtractJsonLd)
      .document(
        r#"<html><head><script type="application/ld+json">
        {
          "@context": "https://schema.org",
          "@type": "Article",
          "author": [{ "name": "foo" }, { "name": "bar" }]
        }
        </script></head><body></body></html>"#,
      )
      .expected_metadata(Metadata {
        byline: Some("foo, bar".to_string()),
        ..Metadata::default()
      })
      .run();
  }
}
