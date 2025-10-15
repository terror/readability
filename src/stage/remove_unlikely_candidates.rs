use super::*;

static REGEX_UNLIKELY_CANDIDATES: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(concat!(
    r"(?i)-ad-|ai2html|banner|breadcrumbs|combx|comment|community|cover-wrap|",
    r"disqus|extra|footer|gdpr|header|legends|menu|related|remark|replies|rss|",
    r"shoutbox|sidebar|skyscraper|social|sponsor|supplemental|ad-break|agegate|",
    r"pagination|pager|popup|yom-remote"
  ))
  .unwrap()
});

static REGEX_OK_MAYBE_CANDIDATE: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"(?i)and|article|body|column|content|main|mathjax|shadow")
    .unwrap()
});

const MIN_CONTENT_TEXT_LENGTH: usize = 200;
const MIN_PARAGRAPH_THRESHOLD: usize = 2;

pub struct RemoveUnlikelyCandidatesStage;

impl Stage for RemoveUnlikelyCandidatesStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let html = context.html_mut();
    Self::remove_unlikely_candidates(html);
    Ok(())
  }
}

impl RemoveUnlikelyCandidatesStage {
  fn has_ancestor_tag(node: NodeRef<'_, Node>, tags: &[&str]) -> bool {
    let mut parent = node.parent();

    while let Some(current) = parent {
      if let Node::Element(element) = current.value()
        && tags.contains(&element.name())
      {
        return true;
      }

      parent = current.parent();
    }

    false
  }

  fn is_empty_container(node: NodeRef<'_, Node>) -> bool {
    let Node::Element(element) = node.value() else {
      return false;
    };

    let mut has_text = false;
    let mut allowed_children_only = true;

    for descendant in node.children() {
      match descendant.value() {
        Node::Text(text) => {
          if !text.trim().is_empty() {
            has_text = true;
            break;
          }
        }
        Node::Element(child_element) => {
          if !matches!(child_element.name(), "br" | "hr") {
            allowed_children_only = false;
            break;
          }
        }
        _ => {}
      }
    }

    if has_text {
      return false;
    }

    if !allowed_children_only {
      return false;
    }

    matches!(
      element.name(),
      "div" | "section" | "header" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
    )
  }

  fn remove_unlikely_candidates(html: &mut Html) {
    let mut to_remove = Vec::new();

    for node in html.tree.root().descendants() {
      let Node::Element(element) = node.value() else {
        continue;
      };

      let tag = element.name();

      if matches!(tag, "body" | "a" | "html" | "head") {
        continue;
      }

      if let Some(role) = element.attr("role")
        && matches!(
          role,
          "menu"
            | "menubar"
            | "complementary"
            | "navigation"
            | "alert"
            | "alertdialog"
            | "dialog"
        )
      {
        to_remove.push(node.id());
        continue;
      }

      let mut match_parts = Vec::new();

      if let Some(class_attr) = element.attr("class") {
        match_parts.push(class_attr);
      }

      if let Some(id_attr) = element.attr("id") {
        match_parts.push(id_attr);
      }

      let match_string = match_parts.join(" ").trim().to_string();

      let matches_unlikely = !match_string.is_empty()
        && REGEX_UNLIKELY_CANDIDATES.is_match(&match_string);

      if matches_unlikely
        && !REGEX_OK_MAYBE_CANDIDATE.is_match(&match_string)
        && !Self::has_ancestor_tag(node, &["table", "code"])
      {
        if Self::contains_significant_content(node) {
          continue;
        }

        to_remove.push(node.id());
        continue;
      }

      if Self::is_empty_container(node) {
        to_remove.push(node.id());
      }
    }

    for id in to_remove {
      if let Some(mut node) = html.tree.get_mut(id) {
        node.detach();
      }
    }
  }

  fn contains_significant_content(node: NodeRef<'_, Node>) -> bool {
    let text_length = Self::node_text_length(node);

    if text_length >= MIN_CONTENT_TEXT_LENGTH {
      return true;
    }

    let paragraph_count = Self::count_descendants_with_tag(node, &["p"]);

    paragraph_count >= MIN_PARAGRAPH_THRESHOLD
  }

  fn count_descendants_with_tag(
    node: NodeRef<'_, Node>,
    tags: &[&str],
  ) -> usize {
    node
      .descendants()
      .filter(|descendant| {
        matches!(
          descendant.value(),
          Node::Element(element) if tags.contains(&element.name())
        )
      })
      .count()
  }

  fn node_text_length(node: NodeRef<'_, Node>) -> usize {
    node
      .descendants()
      .filter_map(|descendant| {
        if let Node::Text(text) = descendant.value() {
          Some(text.trim().len())
        } else {
          None
        }
      })
      .sum()
  }
}
