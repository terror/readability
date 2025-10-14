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
