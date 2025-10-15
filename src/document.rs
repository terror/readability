use super::*;

static REGEX_NORMALIZE: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\s{2,}").unwrap());

static REGEX_HASH_URL: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^#.+").unwrap());

static REGEX_TITLE_SEPARATORS: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\s[\|\-–—\\\/>»]\s").unwrap());

static REGEX_TITLE_FIRST_SEPARATOR: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^[^\|\-–—\\\/>»]*[\|\-–—\\\/>»]").unwrap());

static REGEX_TITLE_HIERARCHICAL_SEPARATORS: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\s[\\\/>»]\s").unwrap());

#[derive(Clone, Copy)]
pub(crate) struct Document<'a> {
  html: &'a Html,
}

impl<'a> Document<'a> {
  pub(crate) fn body_element(self) -> Option<NodeRef<'a, Node>> {
    self.html_element()?.children().find(
      |child| matches!(child.value(), Node::Element(el) if el.name() == "body"),
    )
  }

  pub(crate) fn collect_text(self, node_id: NodeId, normalize: bool) -> String {
    let Some(node) = self.node(node_id) else {
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

  pub(crate) fn count_elements(self) -> usize {
    self
      .html
      .tree
      .root()
      .descendants()
      .filter(|node| node.value().is_element())
      .count()
  }

  pub(crate) fn html_element(self) -> Option<NodeRef<'a, Node>> {
    self.root().children().find(
      |child| matches!(child.value(), Node::Element(el) if el.name() == "html"),
    )
  }

  pub(crate) fn link_density(self, node_id: NodeId) -> f64 {
    let text_length = u32::try_from(self.collect_text(node_id, true).len())
      .map_or(0.0, f64::from);

    if text_length == 0.0 {
      return 0.0;
    }

    let mut link_length = 0.0;

    if let Some(node) = self.node(node_id) {
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

          link_length +=
            u32::try_from(text.trim().len()).map_or(0.0, f64::from) * weight;
        }
      }
    }

    link_length / text_length
  }

  pub(crate) fn new(html: &'a Html) -> Self {
    Self { html }
  }

  pub(crate) fn node(self, id: NodeId) -> Option<NodeRef<'a, Node>> {
    self.html.tree.get(id)
  }

  pub(crate) fn root(self) -> NodeRef<'a, Node> {
    self.html.tree.root()
  }

  pub(crate) fn title(self) -> Option<String> {
    let title = self
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
      .filter(|title| !title.is_empty());

    let mut cur_title = title?;

    let orig_title = cur_title.clone();

    let mut title_had_hierarchical_separators = false;

    if REGEX_TITLE_SEPARATORS.is_match(&cur_title) {
      title_had_hierarchical_separators =
        REGEX_TITLE_HIERARCHICAL_SEPARATORS.is_match(&cur_title);

      if let Some(last_match) =
        REGEX_TITLE_SEPARATORS.find_iter(&orig_title).last()
      {
        cur_title = orig_title[..last_match.start()].to_string();
      }

      if Self::word_count(&cur_title) < 3 {
        cur_title = REGEX_TITLE_FIRST_SEPARATOR
          .replace(&orig_title, "")
          .to_string();
      }
    } else if cur_title.contains(": ") {
      let trimmed_title = cur_title.trim();

      let has_matching_heading = self
        .root()
        .descendants()
        .filter_map(ElementRef::wrap)
        .filter(|element| matches!(element.value().name(), "h1" | "h2"))
        .map(|element| self.collect_text(element.id(), true))
        .any(|heading| heading.trim() == trimmed_title);

      if !has_matching_heading
        && let Some((_, after)) = orig_title.rsplit_once(':')
        && !has_matching_heading
      {
        cur_title = after.to_string();

        if let Some((before, after)) = orig_title.split_once(':')
          && Self::word_count(&cur_title) < 3
        {
          cur_title = after.to_string();

          if Self::word_count(before) > 5 {
            cur_title.clone_from(&orig_title);
          }
        }
      }
    } else if cur_title.len() > 150 || cur_title.len() < 15 {
      let mut h1_nodes = Vec::new();

      for node in self.root().descendants() {
        if let Some(element) = ElementRef::wrap(node)
          && element.value().name() == "h1"
        {
          h1_nodes.push(node.id());

          if h1_nodes.len() > 1 {
            break;
          }
        }
      }

      if h1_nodes.len() == 1 {
        cur_title = self.collect_text(h1_nodes[0], true);
      }
    }

    cur_title = REGEX_NORMALIZE
      .replace_all(cur_title.trim(), " ")
      .into_owned();

    let cur_title_word_count = Self::word_count(&cur_title);

    let normalized_orig = REGEX_TITLE_SEPARATORS
      .replace_all(&orig_title, "")
      .into_owned();

    let normalized_word_count =
      Self::word_count(&normalized_orig).saturating_sub(1);

    if cur_title_word_count <= 4
      && (!title_had_hierarchical_separators
        || cur_title_word_count != normalized_word_count)
    {
      cur_title = orig_title;
    }

    if cur_title.is_empty() {
      None
    } else {
      Some(cur_title)
    }
  }

  fn word_count(value: &str) -> usize {
    value
      .split_whitespace()
      .filter(|token| !token.is_empty())
      .count()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn finds_html_and_body_elements() {
    let html = Html::parse_document(
      "
      <html>
        <head></head>
        <body><p>content</p></body>
      </html>
      ",
    );

    let document = Document::new(&html);

    let html_element = document.html_element().expect("missing <html> element");

    assert!(matches!(
      html_element.value(),
      Node::Element(element) if element.name() == "html"
    ));

    let body_element = document.body_element().expect("missing <body> element");

    assert!(matches!(
      body_element.value(),
      Node::Element(element) if element.name() == "body"
    ));

    assert!(document.node(body_element.id()).is_some());
  }

  #[test]
  fn collect_text_respects_normalization_flag() {
    let html = Html::parse_document(
      "
      <html>
        <body>
          Hi   there <span>General
          Kenobi</span>
        </body>
      </html>
      ",
    );
    let document = Document::new(&html);

    let body = document.body_element().expect("missing <body>");

    let raw = document.collect_text(body.id(), false);
    let normalized = document.collect_text(body.id(), true);

    assert!(raw.contains('\n'));
    assert!(raw.contains("   "));

    assert_eq!(normalized, "Hi there General Kenobi");
  }

  #[test]
  fn counts_element_nodes_only_once() {
    let html = Html::parse_document(
      r#"
      <html>
        <head><meta charset="utf-8" /></head>
        <body>
          <div>
            <p>One</p>
            <span>Two</span>
          </div>
          <img src="image.png" />
        </body>
      </html>
      "#,
    );

    let document = Document::new(&html);

    assert_eq!(document.count_elements(), 8);
  }

  #[test]
  fn link_density_weights_hash_links_less() {
    let html = Html::parse_document(
      r##"
      <html>
        <body>
          Intro text
          <a href="/link">Link text</a>
          filler
          <a href="#section">Hash link</a>
          tail
        </body>
      </html>
      "##,
    );

    let document = Document::new(&html);

    let body = document.body_element().unwrap();

    let normalized_body = document.collect_text(body.id(), true);

    let text_length = normalized_body.len() as f64;
    let expected_link_length = 9.0 + (9.0 * 0.3);

    let (expected_density, actual_density) = (
      expected_link_length / text_length,
      document.link_density(body.id()),
    );

    assert!((actual_density - expected_density).abs() < 1e-9);
  }

  #[test]
  fn link_density_returns_zero_when_no_text() {
    let html = Html::parse_document(
      "
      <html>
        <head></head>
        <body></body>
      </html>
      ",
    );

    let document = Document::new(&html);

    let html_element = document.html_element().unwrap();

    let head = html_element
      .children()
      .find(|child| {
        matches!(child.value(), Node::Element(el) if el.name() == "head")
      })
      .unwrap();

    assert_eq!(document.link_density(head.id()), 0.0);
  }

  #[test]
  fn title_prefers_single_h1_when_title_length_is_extreme() {
    let html = Html::parse_document(
      "
      <html>
        <head><title>Hi</title></head>
        <body><h1>Descriptive Heading For Article Content</h1></body>
      </html>
      ",
    );
    let document = Document::new(&html);

    assert_eq!(
      document.title(),
      Some("Descriptive Heading For Article Content".to_string())
    );
  }

  #[test]
  fn title_prefers_portion_after_colon_without_matching_heading() {
    let html = Html::parse_document(
      "
      <html>
        <head><title>Category: One Two Three Four Five</title></head>
        <body><h1>Completely Different Heading</h1></body>
      </html>
      ",
    );

    let document = Document::new(&html);

    assert_eq!(
      document.title(),
      Some("One Two Three Four Five".to_string())
    );
  }

  #[test]
  fn title_trims_after_common_separators() {
    let html = Html::parse_document(
      "
      <html>
        <head><title>An Extra Wordy Article Title - Site Name</title></head>
        <body></body>
      </html>
      ",
    );

    let document = Document::new(&html);

    assert_eq!(
      document.title(),
      Some("An Extra Wordy Article Title".to_string())
    );
  }

  #[test]
  fn title_retains_original_when_separator_leaves_too_few_words() {
    let html = Html::parse_document(
      "
      <html>
        <head><title>Hi - Site Name</title></head>
        <body></body>
      </html>
      ",
    );

    let document = Document::new(&html);

    assert_eq!(document.title(), Some("Hi - Site Name".to_string()));
  }

  #[test]
  fn title_with_matching_heading_keeps_colon_separator() {
    let html = Html::parse_document(
      "
      <html>
        <head><title>News: Breaking Story</title></head>
        <body><h1>News: Breaking Story</h1></body>
      </html>
      ",
    );

    let document = Document::new(&html);

    assert_eq!(document.title(), Some("News: Breaking Story".to_string()));
  }

  #[test]
  fn title_reverts_to_original_when_colon_suffix_is_too_short() {
    let html = Html::parse_document(
      "
      <html>
        <head>
          <title>
            This category name is quite long for the site: Teaser
          </title>
        </head>
        <body><h1>Other heading</h1></body>
      </html>
      ",
    );

    let document = Document::new(&html);

    assert_eq!(
      document.title(),
      Some("This category name is quite long for the site: Teaser".to_string())
    );
  }

  #[test]
  fn title_returns_none_when_missing() {
    let html = Html::parse_document(
      "
      <html>
        <head></head>
        <body></body>
      </html>
      ",
    );

    let document = Document::new(&html);

    assert_eq!(document.title(), None);
  }

  #[test]
  fn word_count_ignores_extra_whitespace() {
    assert_eq!(Document::word_count("  Hello   world  test "), 3);
    assert_eq!(Document::word_count(""), 0);
  }
}
