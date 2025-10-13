use super::*;

pub type ReadabilityError = anyhow::Error;

const DEFAULT_TAGS_TO_SCORE: &[&str] =
  &["section", "h2", "h3", "h4", "h5", "h6", "p", "td", "pre"];

static REGEX_NORMALIZE: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"\s{2,}").unwrap());

static REGEX_COMMAS: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"[,،﹐︐﹑⹀⸲，]").unwrap());

static REGEX_HASH_URL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#.+").unwrap());

#[derive(Debug, Clone)]
struct Candidate {
  node: NodeId,
  score: f64,
}

#[derive(Debug, Clone, Default)]
struct CollectedMetadata {
  title: Option<String>,
  byline: Option<String>,
  excerpt: Option<String>,
  site_name: Option<String>,
  published_time: Option<String>,
}

pub struct Readability {
  html: Html,
  options: ReadabilityOptions,
  _base_url: Option<Url>,
  metadata: CollectedMetadata,
  article_dir: Option<String>,
  article_lang: Option<String>,
}

impl Readability {
  pub fn new(
    html: &str,
    base_url: Option<&str>,
    options: ReadabilityOptions,
  ) -> Result<Self> {
    let base_url = base_url
      .map(Url::parse)
      .transpose()
      .context("invalid base url")?;

    Ok(Self {
      html: Html::parse_document(html),
      options,
      _base_url: base_url,
      metadata: CollectedMetadata::default(),
      article_dir: None,
      article_lang: None,
    })
  }

  pub fn parse(&mut self) -> Result<Article> {
    if let Some(limit) = self.options.max_elems_to_parse {
      let elements = self.count_elements();

      if elements > limit {
        return Err(anyhow!(
          "Aborting parsing document; {elements} elements found (limit: {limit})"
        ));
      }
    }

    self.remove_scripts_and_styles();

    self.article_lang = self.infer_lang();

    self.metadata = self.collect_metadata();

    let title = self
      .metadata
      .title
      .clone()
      .unwrap_or_else(|| self.get_article_title());

    let article_html = self
      .grab_article()
      .context("failed to identify article content")?;

    let text_content = Self::text_from_html(&article_html);

    let excerpt = self
      .metadata
      .excerpt
      .clone()
      .or_else(|| Self::first_paragraph(&article_html));

    let length = text_content.chars().count();

    Ok(Article {
      title,
      byline: self.metadata.byline.clone(),
      dir: self.article_dir.clone(),
      lang: self.article_lang.clone(),
      content: article_html,
      text_content,
      length,
      excerpt,
      site_name: self.metadata.site_name.clone(),
      published_time: self.metadata.published_time.clone(),
    })
  }

  fn count_elements(&self) -> usize {
    self
      .html
      .tree
      .root()
      .descendants()
      .filter(|node| node.value().is_element())
      .count()
  }

  fn html_element(&self) -> Option<NodeRef<'_, Node>> {
    self.html.tree.root().children().find(
      |child| matches!(child.value(), Node::Element(el) if el.name() == "html"),
    )
  }

  fn body_element(&self) -> Option<NodeRef<'_, Node>> {
    self.html_element()?.children().find(
      |child| matches!(child.value(), Node::Element(el) if el.name() == "body"),
    )
  }

  fn remove_scripts_and_styles(&mut self) {
    let mut to_remove = Vec::new();

    for node in self.html.tree.root().descendants() {
      if let Node::Element(element) = node.value() {
        match element.name() {
          "script" | "noscript" | "style" => to_remove.push(node.id()),
          _ => {}
        }
      }
    }

    for id in to_remove {
      if let Some(mut node) = self.html.tree.get_mut(id) {
        node.detach();
      }
    }
  }

  fn infer_lang(&self) -> Option<String> {
    self
      .html_element()
      .and_then(ElementRef::wrap)
      .and_then(|el| el.value().attr("lang"))
      .map(|value| value.to_string())
  }

  fn collect_metadata(&self) -> CollectedMetadata {
    let mut metadata = CollectedMetadata::default();

    let document_title = self.document_title();

    let mut values: HashMap<String, String> = HashMap::new();

    if let Some(head) = self
      .html_element()
      .and_then(|html| html.children().find(|child| matches!(child.value(), Node::Element(el) if el.name() == "head")))
    {
      for meta in head.children() {
        let element = match ElementRef::wrap(meta) {
          Some(element) if element.value().name() == "meta" => element,
          _ => continue,
        };

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

    if metadata.title.as_ref().is_none_or(|value| value.is_empty()) {
      metadata.title = document_title;
    }

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

  fn document_title(&self) -> Option<String> {
    self
      .html_element()
      .and_then(|html| {
        html.children()
          .find(|child| matches!(child.value(), Node::Element(el) if el.name() == "head"))
      })
      .and_then(|head| {
        head.children()
          .find(|child| matches!(child.value(), Node::Element(el) if el.name() == "title"))
      })
      .map(|title_node| self.collect_text(title_node.id(), true))
      .filter(|title| !title.is_empty())
  }

  fn get_article_title(&self) -> String {
    self
      .metadata
      .title
      .clone()
      .or_else(|| self.document_title())
      .unwrap_or_else(|| String::from("Untitled"))
  }

  fn grab_article(&mut self) -> Option<String> {
    let body_id = self.body_element()?.id();

    if let Some(body_lang) = self
      .html
      .tree
      .get(body_id)
      .and_then(ElementRef::wrap)
      .and_then(|el| el.value().attr("lang"))
    {
      self.article_lang = Some(body_lang.to_string());
    }

    let body = self.html.tree.get(body_id)?;

    let mut elements_to_score = Vec::new();

    for node in body.descendants() {
      let element = match ElementRef::wrap(node) {
        Some(el) => el,
        None => continue,
      };

      if DEFAULT_TAGS_TO_SCORE.contains(&element.value().name()) {
        elements_to_score.push(element);
      }
    }

    let mut candidates: HashMap<NodeId, Candidate> = HashMap::new();

    for element in elements_to_score {
      let text = element.text().collect::<Vec<_>>().join(" ");

      let text = text.trim();

      if text.len() < 25 {
        continue;
      }

      let mut score = 1.0;
      score += REGEX_COMMAS.find_iter(text).count() as f64;
      score += (text.len() / 100).min(3) as f64;

      let mut node = element.deref().parent();
      let mut level = 0;

      while let Some(parent) = node {
        let entry = candidates.entry(parent.id()).or_insert(Candidate {
          node: parent.id(),
          score: 0.0,
        });

        let divider = match level {
          0 => 1.0,
          1 => 2.0,
          _ => (level as f64 + 1.0) * 3.0,
        };

        entry.score += score / divider;

        level += 1;

        if level >= 5 {
          break;
        }

        node = parent.parent();
      }
    }

    if candidates.is_empty() {
      return None;
    }

    let mut top_candidates: Vec<Candidate> =
      candidates.values().cloned().collect();

    top_candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    let top_score = top_candidates.first().map(|c| c.score).unwrap_or(0.0);
    let sibling_score_threshold = (top_score * 0.2).max(10.0);
    let top_candidate = top_candidates.first()?.node;

    let mut article_parts = Vec::new();

    let top_node = self.html.tree.get(top_candidate)?;
    let parent = top_node.parent();

    if let Some(parent) = parent {
      for child in parent.children() {
        let Some(element) = ElementRef::wrap(child) else {
          continue;
        };

        let append = if child.id() == top_candidate {
          true
        } else {
          let candidate_score =
            candidates.get(&child.id()).map(|c| c.score).unwrap_or(0.0);

          if candidate_score >= sibling_score_threshold {
            true
          } else if element.value().name() == "p" {
            let text = self.collect_text(child.id(), true);

            let link_density = self.link_density(child.id());

            let len = text.len();

            (len > 80 && link_density < 0.25)
              || (len > 0
                && len <= 80
                && link_density == 0.0
                && text.contains('.'))
          } else {
            false
          }
        };

        if append {
          article_parts.push(element.html());
        }
      }
    } else if let Some(element) = ElementRef::wrap(top_node) {
      article_parts.push(element.html());
    }

    if article_parts.is_empty() {
      return None;
    }

    let content = format!(
      "<div id=\"readability-content\"><div id=\"readability-page-1\" class=\"page\">{}</div></div>",
      article_parts.join("")
    );

    Some(content)
  }

  fn collect_text(&self, node_id: NodeId, normalize: bool) -> String {
    let Some(node) = self.html.tree.get(node_id) else {
      return String::new();
    };

    let mut text = String::new();

    for descendant in node.descendants() {
      if let Node::Text(value) = descendant.value() {
        text.push_str(value);
      }
    }

    let text = text.trim();

    if normalize {
      REGEX_NORMALIZE.replace_all(text, " ").into_owned()
    } else {
      text.to_string()
    }
  }

  fn link_density(&self, node_id: NodeId) -> f64 {
    let text_length = self.collect_text(node_id, true).len() as f64;

    if text_length == 0.0 {
      return 0.0;
    }

    let mut link_length = 0.0;

    if let Some(node) = self.html.tree.get(node_id) {
      for descendant in node.descendants() {
        if let Some(element) = ElementRef::wrap(descendant)
          && element.value().name() == "a"
        {
          let text = element.text().collect::<Vec<_>>().join(" ");

          let href = element.value().attr("href").unwrap_or_default();

          let weight = if REGEX_HASH_URL.is_match(href) {
            0.3
          } else {
            1.0
          };

          link_length += text.trim().len() as f64 * weight;
        }
      }
    }

    link_length / text_length
  }

  fn text_from_html(html: &str) -> String {
    let fragment = Html::parse_fragment(html);

    let mut text = String::new();

    for node in fragment.tree.root().descendants() {
      if let Node::Text(value) = node.value() {
        if !text.is_empty() {
          text.push(' ');
        }

        text.push_str(value.trim());
      }
    }

    REGEX_NORMALIZE
      .replace_all(&text, " ")
      .into_owned()
      .trim()
      .to_string()
  }

  fn first_paragraph(html: &str) -> Option<String> {
    let selector = Selector::parse("p").unwrap();

    let fragment = Html::parse_fragment(html);

    fragment
      .select(&selector)
      .map(|el| el.text().collect::<Vec<_>>().join(" ").trim().to_string())
      .find(|text| !text.is_empty())
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
