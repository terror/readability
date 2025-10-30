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

static REGEX_TOKENIZE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\W+").unwrap());

#[derive(Default)]
struct JsonLdMetadata {
  byline: Option<String>,
  excerpt: Option<String>,
  published_time: Option<String>,
  site_name: Option<String>,
  title: Option<String>,
}

impl JsonLdMetadata {
  fn is_complete(&self) -> bool {
    self.title.is_some()
      && self.byline.is_some()
      && self.excerpt.is_some()
      && self.site_name.is_some()
      && self.published_time.is_some()
  }

  fn merge_byline(&mut self, value: Option<String>) {
    if self.byline.is_none() {
      self.byline = value;
    }
  }

  fn merge_excerpt(&mut self, value: Option<String>) {
    if self.excerpt.is_none() {
      self.excerpt = value;
    }
  }

  fn merge_published_time(&mut self, value: Option<String>) {
    if self.published_time.is_none() {
      self.published_time = value;
    }
  }

  fn merge_site_name(&mut self, value: Option<String>) {
    if self.site_name.is_none() {
      self.site_name = value;
    }
  }

  fn merge_title(&mut self, value: Option<String>) {
    if self.title.is_none() {
      self.title = value;
    }
  }
}

struct JsonLd<'a> {
  document: Document<'a>,
}

impl<'a> JsonLd<'a> {
  const ARTICLE_TYPES: [&'static str; 19] = [
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

  fn clean_source(raw: &str) -> Option<String> {
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

  fn collect_metadata(&self) -> JsonLdMetadata {
    let document_title = self.document.title();
    let mut metadata = JsonLdMetadata::default();

    for node in self.document.root().descendants() {
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

      let Some(json_source) = Self::clean_source(&raw) else {
        continue;
      };

      Self::parse_payload(
        &json_source,
        document_title.as_deref(),
        &mut metadata,
      );

      if metadata.is_complete() {
        break;
      }
    }

    metadata
  }

  fn extract_article_metadata(
    map: &serde_json::Map<String, Value>,
    document_title: Option<&str>,
    metadata: &mut JsonLdMetadata,
  ) {
    let name = Self::extract_string_field(map, "name");

    let headline = Self::extract_string_field(map, "headline");

    metadata.merge_title(Self::sanitize_value(Self::select_title(
      name,
      headline,
      document_title,
    )));

    metadata.merge_byline(Self::extract_author(map.get("author")));

    metadata.merge_excerpt(Self::sanitize_value(Self::extract_string_field(
      map,
      "description",
    )));

    if let Some(site_name) = Self::extract_string_field(map, "siteName") {
      metadata.merge_site_name(Self::sanitize_value(Some(site_name)));
    } else if let Some(publisher) = map.get("publisher") {
      metadata.merge_site_name(Self::extract_publisher_name(publisher));
    }

    metadata.merge_published_time(Self::sanitize_value(
      Self::extract_string_field(map, "datePublished"),
    ));
  }

  fn extract_author(value: Option<&Value>) -> Option<String> {
    value.and_then(Self::extract_author_value)
  }

  fn extract_author_value(value: &Value) -> Option<String> {
    match value {
      Value::Object(map) => {
        if let Some(name) = map.get("name").and_then(Value::as_str) {
          return Self::sanitize_value(Some(name));
        }

        None
      }
      Value::Array(items) => {
        let mut names = Vec::new();

        for item in items {
          if let Some(name) = Self::extract_author_value(item) {
            names.push(name);
          }
        }

        if names.is_empty() {
          None
        } else {
          Some(names.join(", "))
        }
      }
      Value::String(raw) => Self::sanitize_value(Some(raw)),
      _ => None,
    }
  }

  fn extract_publisher_name(value: &Value) -> Option<String> {
    match value {
      Value::Object(map) => {
        if let Some(name) = map.get("name").and_then(Value::as_str)
          && let Some(sanitized) = Self::sanitize_value(Some(name))
        {
          return Some(sanitized);
        }

        for nested in map.values() {
          if let Some(result) = Self::extract_publisher_name(nested) {
            return Some(result);
          }
        }

        None
      }
      Value::Array(items) => {
        items.iter().find_map(Self::extract_publisher_name)
      }
      _ => None,
    }
  }

  fn extract_string_field<'b>(
    map: &'b serde_json::Map<String, Value>,
    key: &str,
  ) -> Option<&'b str> {
    map
      .get(key)
      .and_then(Value::as_str)
      .map(str::trim)
      .filter(|value| !value.is_empty())
  }

  fn is_article_type(value: &Value) -> bool {
    match value {
      Value::String(typ) => {
        Self::ARTICLE_TYPES.iter().any(|candidate| candidate == typ)
      }
      Value::Array(items) => items.iter().any(Self::is_article_type),
      _ => false,
    }
  }

  fn is_json_ld_script_type(value: &str) -> bool {
    value.split(';').next().is_some_and(|prefix| {
      prefix.trim().eq_ignore_ascii_case("application/ld+json")
    })
  }

  fn new(document: Document<'a>) -> Self {
    Self { document }
  }

  fn parse_payload(
    source: &str,
    document_title: Option<&str>,
    metadata: &mut JsonLdMetadata,
  ) {
    let stream = Deserializer::from_str(source).into_iter::<Value>();

    for value in stream {
      let Ok(value) = value else {
        break;
      };

      Self::update_metadata(&value, document_title, metadata);

      if metadata.is_complete() {
        break;
      }
    }
  }

  fn sanitize_value(value: Option<&str>) -> Option<String> {
    let value = value?;

    let trimmed = value.trim();

    if trimmed.is_empty() {
      None
    } else {
      Some(MetadataStage::decode_html_entities(trimmed))
    }
  }

  fn select_title<'b>(
    name: Option<&'b str>,
    headline: Option<&'b str>,
    document_title: Option<&str>,
  ) -> Option<&'b str> {
    match (name, headline) {
      (Some(name), Some(headline)) if name != headline => {
        if let Some(doc_title) = document_title {
          let name_matches = Self::text_similarity(name, doc_title) > 0.75;

          let headline_matches =
            Self::text_similarity(headline, doc_title) > 0.75;

          if headline_matches && !name_matches {
            Some(headline)
          } else {
            Some(name)
          }
        } else {
          Some(name)
        }
      }
      (Some(name), _) => Some(name),
      (None, Some(headline)) => Some(headline),
      _ => None,
    }
  }

  fn text_similarity(text_a: &str, text_b: &str) -> f64 {
    let (lower_a, lower_b) = (text_a.to_lowercase(), text_b.to_lowercase());

    let tokens_a: Vec<&str> = REGEX_TOKENIZE
      .split(&lower_a)
      .filter(|token| !token.is_empty())
      .collect();

    let tokens_b: Vec<&str> = REGEX_TOKENIZE
      .split(&lower_b)
      .filter(|token| !token.is_empty())
      .collect();

    if tokens_a.is_empty() || tokens_b.is_empty() {
      return 0.0;
    }

    let tokens_a_set = tokens_a.iter().copied().collect::<HashSet<&str>>();

    let uniq_tokens_b = tokens_b
      .iter()
      .copied()
      .filter(|token| !tokens_a_set.contains(token))
      .collect::<Vec<&str>>();

    if uniq_tokens_b.is_empty() {
      return 1.0;
    }

    let (uniq_str, tokens_b_str) =
      (uniq_tokens_b.join(" "), tokens_b.join(" "));

    if tokens_b_str.is_empty() {
      0.0
    } else {
      let uniq_len_f64 =
        f64::from(u32::try_from(uniq_str.len()).unwrap_or(u32::MAX));

      let tokens_len_f64 =
        f64::from(u32::try_from(tokens_b_str.len()).unwrap_or(u32::MAX));

      1.0 - (uniq_len_f64 / tokens_len_f64)
    }
  }

  fn update_metadata(
    value: &Value,
    document_title: Option<&str>,
    metadata: &mut JsonLdMetadata,
  ) {
    match value {
      Value::Object(map) => {
        if let Some(graph) = map.get("@graph") {
          Self::update_metadata(graph, document_title, metadata);
        }

        let is_article = map.get("@type").is_some_and(Self::is_article_type);

        if is_article {
          Self::extract_article_metadata(map, document_title, metadata);
        }

        if metadata.is_complete() {
          return;
        }

        if let Some(main_entity) = map.get("mainEntity") {
          Self::update_metadata(main_entity, document_title, metadata);
        }

        if metadata.is_complete() {
          return;
        }

        if let Some(main_entity_page) = map.get("mainEntityOfPage") {
          Self::update_metadata(main_entity_page, document_title, metadata);
        }

        if metadata.is_complete() {
          return;
        }

        for nested_value in map.values() {
          Self::update_metadata(nested_value, document_title, metadata);

          if metadata.is_complete() {
            break;
          }
        }
      }
      Value::Array(items) => {
        for item in items {
          Self::update_metadata(item, document_title, metadata);

          if metadata.is_complete() {
            break;
          }
        }
      }
      _ => {}
    }
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
  const BYLINE_KEYS: [&'static str; 7] = [
    "dc:creator",
    "dcterm:creator",
    "dcterms:creator",
    "dc:author",
    "author",
    "parsely:author",
    "og:article:author",
  ];
  const EXCERPT_KEYS: [&'static str; 6] = [
    "dc:description",
    "dcterm:description",
    "dcterms:description",
    "og:description",
    "description",
    "twitter:description",
  ];

  const PUBLISHED_TIME_KEYS: [&'static str; 2] =
    ["article:published_time", "parsely:pub-date"];

  const REPLACEMENT_CHAR: char = '\u{FFFD}';
  const REPLACEMENT_CODEPOINT: u32 = 0xFFFD;

  const SITE_NAME_KEYS: [&'static str; 3] =
    ["og:site_name", "parsely:site_name", "parsely:site"];

  const TITLE_KEYS: [&'static str; 6] = [
    "dc:title",
    "dcterm:title",
    "dcterms:title",
    "og:title",
    "twitter:title",
    "title",
  ];

  fn collect_metadata(
    document: Document<'_>,
    options: &ReadabilityOptions,
  ) -> Metadata {
    let json_ld = if options.disable_json_ld {
      JsonLdMetadata::default()
    } else {
      JsonLd::new(document).collect_metadata()
    };

    let values = Self::collect_values(document);

    Metadata {
      title: json_ld
        .title
        .clone()
        .or_else(|| {
          Self::pick_meta_value(&values, &Self::TITLE_KEYS)
            .filter(|value| !value.trim().is_empty())
        })
        .or_else(|| document.title()),
      byline: json_ld
        .byline
        .clone()
        .or_else(|| {
          Self::pick_meta_value(&values, &Self::BYLINE_KEYS)
            .filter(|value| !value.trim().is_empty())
        })
        .or_else(|| Self::find_byline(document)),
      excerpt: json_ld
        .excerpt
        .clone()
        .or_else(|| Self::pick_meta_value(&values, &Self::EXCERPT_KEYS)),
      site_name: json_ld
        .site_name
        .clone()
        .or_else(|| Self::pick_meta_value(&values, &Self::SITE_NAME_KEYS)),
      published_time: json_ld
        .published_time
        .clone()
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
      for meta in head.descendants() {
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

    if values.is_empty() {
      for node in document.root().descendants() {
        let Some(element) = ElementRef::wrap(node) else {
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

      let mut match_parts = Vec::new();

      if let Some(class_name) = element.value().attr("class") {
        match_parts.push(class_name);
      }

      if let Some(id) = element.value().attr("id") {
        match_parts.push(id);
      }

      let match_string = match_parts.join(" ");

      let rel_author = element.value().attr("rel").is_some_and(|value| {
        value
          .split_whitespace()
          .any(|token| token.eq_ignore_ascii_case("author"))
      });

      let itemprop_author = element
        .value()
        .attr("itemprop")
        .is_some_and(|value| value.to_ascii_lowercase().contains("author"));

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

      let parent_has_byline_keyword =
        match_string.to_ascii_lowercase().contains("byline");

      if let Some(child_byline) = Self::find_descendant_byline(
        document,
        element,
        parent_has_byline_keyword,
      ) {
        return Some(child_byline);
      }

      return Some(Self::decode_html_entities(trimmed));
    }

    None
  }

  fn find_descendant_byline(
    document: Document<'_>,
    element: ElementRef<'_>,
    parent_has_byline_keyword: bool,
  ) -> Option<String> {
    if parent_has_byline_keyword {
      return None;
    }

    let mut best: Option<(usize, String)> = None;

    for descendant in element.descendants() {
      let Some(child) = ElementRef::wrap(descendant) else {
        continue;
      };

      if child.id() == element.id() {
        continue;
      }

      let text = document.collect_text(child.id(), true);

      let trimmed = text.trim();

      if trimmed.is_empty() || trimmed.chars().count() >= 100 {
        continue;
      }

      let mut match_parts = Vec::new();

      if let Some(class_name) = child.value().attr("class") {
        match_parts.push(class_name);
      }

      if let Some(id) = child.value().attr("id") {
        match_parts.push(id);
      }

      let match_string = match_parts.join(" ");

      let rel_author = child.value().attr("rel").is_some_and(|value| {
        value
          .split_whitespace()
          .any(|token| token.eq_ignore_ascii_case("author"))
      });

      let itemprop_author = child
        .value()
        .attr("itemprop")
        .is_some_and(|value| value.to_ascii_lowercase().contains("author"));

      let class_match =
        !match_string.is_empty() && REGEX_BYLINE.is_match(&match_string);

      if !(rel_author || itemprop_author || class_match) {
        continue;
      }

      let decoded = Self::decode_html_entities(trimmed);

      let length = decoded.chars().count();

      match best {
        Some((best_length, _)) if length <= best_length => continue,
        _ => best = Some((length, decoded)),
      };
    }

    best.map(|(_, value)| value)
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
