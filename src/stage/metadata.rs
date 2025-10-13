use super::*;

static REGEX_BYLINE: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"(?i)byline|author|dateline|writtenby|p-author").unwrap()
});

static SELECTOR_ITEMPROP_NAME: LazyLock<Selector> =
  LazyLock::new(|| Selector::parse("[itemprop*=\"name\"]").unwrap());

pub struct MetadataStage;

impl Stage for MetadataStage {
  fn run(&mut self, ctx: &mut Context<'_>) -> Result<()> {
    let mut metadata = Self::collect_metadata(ctx.document());

    if metadata.title.as_ref().is_none_or(String::is_empty) {
      metadata.title = ctx.document().document_title();
    }

    ctx.set_metadata(metadata);

    Ok(())
  }
}

impl MetadataStage {
  fn collect_metadata(document: Document<'_>) -> CollectedMetadata {
    let mut metadata = CollectedMetadata::default();

    let values = Self::collect_values(document);

    metadata.title = Self::pick_meta_value(
      &values,
      &[
        "dc:title",
        "dcterm:title",
        "dcterms:title",
        "title",
        "og:title",
        "twitter:title",
      ],
    );

    metadata.byline = Self::pick_meta_value(
      &values,
      &[
        "dc:creator",
        "dcterm:creator",
        "dcterms:creator",
        "dc:author",
        "author",
        "parsely:author",
      ],
    );

    if metadata
      .byline
      .as_ref()
      .is_none_or(|value| value.trim().is_empty())
    {
      metadata.byline = Self::find_byline(document);
    }

    metadata.excerpt = Self::pick_meta_value(
      &values,
      &[
        "dc:description",
        "dcterm:description",
        "dcterms:description",
        "description",
        "og:description",
        "twitter:description",
      ],
    );

    metadata.site_name = Self::pick_meta_value(
      &values,
      &["og:site_name", "parsely:site_name", "parsely:site"],
    );

    metadata.published_time = Self::pick_meta_value(
      &values,
      &[
        "article:published_time",
        "parsely:pub-date",
        "parsely:publish_date",
        "publish_date",
      ],
    );

    metadata
  }

  fn collect_values(document: Document<'_>) -> HashMap<String, String> {
    let mut values = HashMap::new();

    if let Some(head) = document
      .html_element()
      .and_then(|html| {
        html.children()
          .find(|child| matches!(child.value(), Node::Element(el) if el.name() == "head"))
      })
    {
      for meta in head.children() {
        let Some(element) = ElementRef::wrap(meta) else {
          continue;
        };

        if element.value().name() != "meta" {
          continue;
        }

        let content = element.value().attr("content").unwrap_or_default().trim();

        if content.is_empty() {
          continue;
        }

        if let Some(name) = element.value().attr("name") {
          let key = Self::normalize_meta_key(name);

          if !key.is_empty() {
            values.insert(key, content.to_string());
          }
        }

        if let Some(property) = element.value().attr("property") {
          let key = Self::normalize_meta_key(property);

          if !key.is_empty() {
            values.insert(key, content.to_string());
          }
        }
      }
    }

    values
  }

  fn find_byline(document: Document<'_>) -> Option<String> {
    for node in document.root().descendants() {
      let Some(element) = ElementRef::wrap(node) else {
        continue;
      };

      let text = document.collect_text(node.id(), true);
      let trimmed = text.trim();

      if trimmed.is_empty() || trimmed.chars().count() >= 100 {
        continue;
      }

      let rel_author = element.value().attr("rel").is_some_and(|value| {
        value
          .split_whitespace()
          .any(|token| token.eq_ignore_ascii_case("author"))
      });

      let itemprop_author = element
        .value()
        .attr("itemprop")
        .is_some_and(|value| value.to_ascii_lowercase().contains("author"));

      let mut match_parts = Vec::new();

      if let Some(class_name) = element.value().attr("class") {
        match_parts.push(class_name);
      }

      if let Some(id) = element.value().attr("id") {
        match_parts.push(id);
      }

      let match_string = match_parts.join(" ");

      let class_match =
        !match_string.is_empty() && REGEX_BYLINE.is_match(&match_string);

      if !(rel_author || itemprop_author || class_match) {
        continue;
      }

      if let Some(name_el) = element.select(&SELECTOR_ITEMPROP_NAME).next() {
        let name = document.collect_text(name_el.id(), true);
        let trimmed_name = name.trim();

        if !trimmed_name.is_empty() && trimmed_name.chars().count() < 100 {
          return Some(trimmed_name.to_string());
        }
      }

      return Some(trimmed.to_string());
    }

    None
  }

  fn normalize_meta_key(raw: &str) -> String {
    raw
      .trim()
      .chars()
      .filter(|ch| !ch.is_whitespace())
      .map(|ch| {
        if ch == '.' {
          ':'
        } else {
          ch.to_ascii_lowercase()
        }
      })
      .collect()
  }

  fn pick_meta_value(
    values: &HashMap<String, String>,
    keys: &[&str],
  ) -> Option<String> {
    for key in keys {
      let normalized = Self::normalize_meta_key(key);

      if let Some(value) = values.get(&normalized) {
        return Some(value.clone());
      }
    }

    None
  }
}
