use super::*;

static REGEX_BYLINE: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"(?i)byline|author|dateline|writtenby|p-author").unwrap()
});

static SELECTOR_ITEMPROP_NAME: LazyLock<Selector> =
  LazyLock::new(|| Selector::parse("[itemprop*=\"name\"]").unwrap());

static REGEX_BASIC_HTML_ENTITIES: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"&(?P<name>quot|amp|apos|lt|gt);").unwrap());

static REGEX_NUMERIC_HTML_ENTITIES: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"&#(?:x([0-9a-fA-F]+)|([0-9]+));").unwrap());

#[derive(Default)]
struct JsonLdMetadata {
  site_name: Option<String>,
  published_time: Option<String>,
}

impl JsonLdMetadata {
  fn merge_site_name(&mut self, value: Option<String>) {
    if self.site_name.is_none() {
      self.site_name = value;
    }
  }

  fn merge_published_time(&mut self, value: Option<String>) {
    if self.published_time.is_none() {
      self.published_time = value;
    }
  }

  fn is_complete(&self) -> bool {
    self.site_name.is_some() && self.published_time.is_some()
  }
}

pub struct MetadataStage;

impl Stage for MetadataStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    context.set_metadata(Self::collect_metadata(
      context.document(),
      context.options(),
    ));

    Ok(())
  }
}

impl MetadataStage {
  const JSON_LD_ARTICLE_TYPES: [&'static str; 19] = [
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

  const BYLINE_KEYS: [&'static str; 6] = [
    "dc:creator",
    "dcterm:creator",
    "dcterms:creator",
    "dc:author",
    "author",
    "parsely:author",
  ];

  const EXCERPT_KEYS: [&'static str; 6] = [
    "dc:description",
    "dcterm:description",
    "dcterms:description",
    "description",
    "og:description",
    "twitter:description",
  ];

  const PUBLISHED_TIME_KEYS: [&'static str; 4] = [
    "article:published_time",
    "parsely:pub-date",
    "parsely:publish_date",
    "publish_date",
  ];

  const REPLACEMENT_CHAR: char = '\u{FFFD}';
  const REPLACEMENT_CODEPOINT: u32 = 0xFFFD;

  const SITE_NAME_KEYS: [&'static str; 3] =
    ["og:site_name", "parsely:site_name", "parsely:site"];

  const TITLE_KEYS: [&'static str; 6] = [
    "dc:title",
    "dcterm:title",
    "dcterms:title",
    "title",
    "og:title",
    "twitter:title",
  ];

  fn collect_metadata(
    document: Document<'_>,
    options: &ReadabilityOptions,
  ) -> Metadata {
    let json_ld = if options.disable_json_ld {
      JsonLdMetadata::default()
    } else {
      Self::collect_json_ld_metadata(document)
    };

    let values = Self::collect_values(document);

    Metadata {
      title: Self::pick_meta_value(&values, &Self::TITLE_KEYS)
        .filter(|value| !value.trim().is_empty())
        .or_else(|| document.title()),
      byline: Self::pick_meta_value(&values, &Self::BYLINE_KEYS)
        .filter(|value| !value.trim().is_empty())
        .or_else(|| Self::find_byline(document)),
      excerpt: Self::pick_meta_value(&values, &Self::EXCERPT_KEYS),
      site_name: json_ld
        .site_name
        .or_else(|| Self::pick_meta_value(&values, &Self::SITE_NAME_KEYS)),
      published_time: json_ld
        .published_time
        .or_else(|| Self::pick_meta_value(&values, &Self::PUBLISHED_TIME_KEYS)),
    }
  }

  fn collect_values(document: Document<'_>) -> HashMap<String, String> {
    let mut values = HashMap::new();

    let head = document
      .html_element()
      .and_then(|html| {
        html.children()
          .find(|child| matches!(child.value(), Node::Element(el) if el.name() == "head"))
      });

    if let Some(head) = head {
      for meta in head.children() {
        let Some(element) = ElementRef::wrap(meta) else {
          continue;
        };

        if element.value().name() != "meta" {
          continue;
        }

        let content =
          element.value().attr("content").unwrap_or_default().trim();

        if content.is_empty() {
          continue;
        }

        if let Some(name) = element.value().attr("name") {
          Self::insert_meta_keys(&mut values, name, content);
        }

        if let Some(property) = element.value().attr("property") {
          Self::insert_meta_keys(&mut values, property, content);
        }
      }
    }

    values
  }

  fn collect_json_ld_metadata(document: Document<'_>) -> JsonLdMetadata {
    let mut metadata = JsonLdMetadata::default();

    for node in document.root().descendants() {
      let Some(element) = ElementRef::wrap(node) else {
        continue;
      };

      if element.value().name() != "script" {
        continue;
      }

      let Some(script_type) = element.value().attr("type") else {
        continue;
      };

      if !Self::is_json_ld_script_type(script_type) {
        continue;
      }

      let raw = element.text().collect::<String>();

      let Some(json_source) = Self::clean_json_ld_source(&raw) else {
        continue;
      };

      Self::parse_json_ld_payload(&json_source, &mut metadata);

      if metadata.is_complete() {
        break;
      }
    }

    metadata
  }

  fn decode_html_entities(input: &str) -> String {
    if !input.contains('&') {
      return input.to_string();
    }

    let named_decoded = REGEX_BASIC_HTML_ENTITIES.replace_all(
      input,
      |captures: &regex::Captures<'_>| -> String {
        match &captures["name"] {
          "quot" => "\"".to_string(),
          "amp" => "&".to_string(),
          "apos" => "'".to_string(),
          "lt" => "<".to_string(),
          "gt" => ">".to_string(),
          _ => captures
            .get(0)
            .map_or(String::new(), |m| m.as_str().to_string()),
        }
      },
    );

    REGEX_NUMERIC_HTML_ENTITIES
      .replace_all(&named_decoded, |captures: &regex::Captures<'_>| {
        let (value, radix) = if let Some(hex) = captures.get(1) {
          (hex.as_str(), 16)
        } else if let Some(num) = captures.get(2) {
          (num.as_str(), 10)
        } else {
          return captures.get(0).map_or(String::new(), |m| m.as_str().into());
        };

        let parsed = u32::from_str_radix(value, radix)
          .unwrap_or(Self::REPLACEMENT_CODEPOINT);

        Self::decode_numeric_codepoint(parsed).to_string()
      })
      .into_owned()
  }

  fn decode_numeric_codepoint(value: u32) -> char {
    const SURROGATE_START: u32 = 0xD800;
    const SURROGATE_END: u32 = 0xDFFF;
    const MAX_CODEPOINT: u32 = 0x0010_FFFF;

    if value == 0
      || value > MAX_CODEPOINT
      || (SURROGATE_START..=SURROGATE_END).contains(&value)
    {
      Self::REPLACEMENT_CHAR
    } else {
      char::from_u32(value).unwrap_or(Self::REPLACEMENT_CHAR)
    }
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
          return Some(Self::decode_html_entities(trimmed_name));
        }
      }

      return Some(Self::decode_html_entities(trimmed));
    }

    None
  }

  fn is_article_type(value: &Value) -> bool {
    match value {
      Value::String(typ) => Self::JSON_LD_ARTICLE_TYPES
        .iter()
        .any(|candidate| candidate == typ),
      Value::Array(items) => items.iter().any(Self::is_article_type),
      _ => false,
    }
  }

  fn is_json_ld_script_type(value: &str) -> bool {
    value
      .split(';')
      .next()
      .map(|prefix| prefix.trim().eq_ignore_ascii_case("application/ld+json"))
      .unwrap_or(false)
  }

  fn clean_json_ld_source(raw: &str) -> Option<String> {
    let mut trimmed = raw.trim();

    if trimmed.is_empty() {
      return None;
    }

    if let Some(stripped) = trimmed.strip_prefix("<![CDATA[") {
      trimmed = stripped;
    }

    if let Some(stripped) = trimmed.strip_suffix("]]>") {
      trimmed = stripped;
    }

    let final_trimmed = trimmed.trim();

    if final_trimmed.is_empty() {
      None
    } else {
      Some(final_trimmed.to_string())
    }
  }

  fn parse_json_ld_payload(source: &str, metadata: &mut JsonLdMetadata) {
    let stream = Deserializer::from_str(source).into_iter::<Value>();

    for value in stream {
      let Ok(value) = value else {
        break;
      };

      Self::update_json_ld_metadata(&value, metadata);

      if metadata.is_complete() {
        break;
      }
    }
  }

  fn update_json_ld_metadata(value: &Value, metadata: &mut JsonLdMetadata) {
    match value {
      Value::Object(map) => {
        if let Some(graph) = map.get("@graph") {
          Self::update_json_ld_metadata(graph, metadata);
        }

        let is_article = map.get("@type").is_some_and(Self::is_article_type);

        if is_article {
          if let Some(site_name) = map
            .get("siteName")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
          {
            metadata
              .merge_site_name(Some(Self::decode_html_entities(site_name)));
          } else if let Some(publisher) = map.get("publisher") {
            if let Some(site_name) = Self::extract_publisher_name(publisher) {
              metadata
                .merge_site_name(Some(Self::decode_html_entities(&site_name)));
            }
          }

          if let Some(published) = map
            .get("datePublished")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
          {
            metadata.merge_published_time(Some(Self::decode_html_entities(
              published,
            )));
          }
        }

        if metadata.is_complete() {
          return;
        }

        if let Some(main_entity) = map.get("mainEntity") {
          Self::update_json_ld_metadata(main_entity, metadata);
        }

        if metadata.is_complete() {
          return;
        }

        if let Some(main_entity_page) = map.get("mainEntityOfPage") {
          Self::update_json_ld_metadata(main_entity_page, metadata);
        }

        if metadata.is_complete() {
          return;
        }

        for nested_value in map.values() {
          Self::update_json_ld_metadata(nested_value, metadata);

          if metadata.is_complete() {
            break;
          }
        }
      }
      Value::Array(items) => {
        for item in items {
          Self::update_json_ld_metadata(item, metadata);

          if metadata.is_complete() {
            break;
          }
        }
      }
      _ => {}
    }
  }

  fn extract_publisher_name(value: &Value) -> Option<String> {
    match value {
      Value::Object(map) => map
        .get("name")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(|name| name.to_owned())
        .or_else(|| map.values().find_map(Self::extract_publisher_name)),
      Value::Array(items) => {
        items.iter().find_map(Self::extract_publisher_name)
      }
      _ => None,
    }
  }

  fn insert_meta_keys(
    values: &mut HashMap<String, String>,
    raw_keys: &str,
    content: &str,
  ) {
    let content = content.to_string();

    for raw_key in raw_keys.split_whitespace() {
      let key = Self::normalize_meta_key(raw_key);

      if key.is_empty() {
        continue;
      }

      values.insert(key, content.clone());
    }
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
        return Some(Self::decode_html_entities(value));
      }
    }

    None
  }
}
