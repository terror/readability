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

pub struct SanitizationStage;

impl Stage for SanitizationStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let html = context.html_mut();

    Self::remove_disallowed_nodes(html);
    Self::rewrite_font_tags(html);
    Self::remove_unlikely_candidates(html);
    Self::replace_break_sequences(html);
    Self::normalize_containers(html);

    Ok(())
  }
}

impl SanitizationStage {
  /// Replaces a `<br>` run with a new `<p>` and moves phrasing siblings inside.
  fn convert_br_chain(html: &mut Html, br_id: NodeId) {
    let Some(br_ref) = html.tree.get(br_id) else {
      return;
    };

    let mut removal_ids = vec![br_id];
    let mut next = br_ref.next_sibling();

    while let Some(node) = next {
      if Self::is_whitespace_text(&node) || Self::is_br_element(&node) {
        removal_ids.push(node.id());
        next = node.next_sibling();
        continue;
      }

      break;
    }

    let mut nodes_to_move = Vec::new();
    let mut cursor = next;

    while let Some(node) = cursor {
      if Self::is_br_element(&node) {
        break;
      }

      if Self::is_phrasing_node(&node) || Self::is_whitespace_text(&node) {
        nodes_to_move.push(node.id());
        cursor = node.next_sibling();
      } else {
        break;
      }
    }

    if nodes_to_move.is_empty() && removal_ids.len() <= 1 {
      return;
    }

    let paragraph_id = {
      let Some(mut br_node) = html.tree.get_mut(br_id) else {
        return;
      };

      let new_id = {
        let paragraph = br_node.insert_before(Self::create_element("p"));
        paragraph.id()
      };

      br_node.detach();

      new_id
    };

    for removal_id in removal_ids.into_iter().skip(1) {
      if let Some(mut node) = html.tree.get_mut(removal_id) {
        node.detach();
      }
    }

    let valid_nodes: Vec<NodeId> = nodes_to_move
      .into_iter()
      .filter(|node_id| html.tree.get(*node_id).is_some())
      .collect();

    if let Some(mut paragraph) = html.tree.get_mut(paragraph_id) {
      for node_id in valid_nodes {
        paragraph.append_id(node_id);
      }
    }

    Self::trim_whitespace(html, paragraph_id);

    if let Some(parent_id) = html
      .tree
      .get(paragraph_id)
      .and_then(|node| node.parent().map(|parent| parent.id()))
      && let Some(mut parent) = html.tree.get_mut(parent_id)
      && let Node::Element(element) = parent.value()
      && element.name() == "p"
    {
      Self::set_element_tag(element, "div");
    }
  }

  /// Constructs a new element node in the HTML namespace.
  fn create_element(tag: &str) -> Node {
    Node::Element(Element::new(
      QualName::new(None, ns!(html), LocalName::from(tag)),
      Vec::new(),
    ))
  }

  /// Returns true if any ancestor tag name matches the supplied list.
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

  /// Detects whether a node contains (or nests) any block-level descendants.
  fn has_block_child(node: NodeRef<'_, Node>) -> bool {
    let mut child = node.first_child();

    while let Some(current) = child {
      let has_block = match current.value() {
        Node::Element(element) => {
          Self::is_block_element(element.name())
            || Self::has_block_child(current)
        }
        _ => false,
      };

      if has_block {
        return true;
      }

      child = current.next_sibling();
    }

    false
  }

  /// Returns true when the tag is treated as a block-level element here.
  fn is_block_element(tag: &str) -> bool {
    matches!(
      tag,
      "blockquote" | "dl" | "div" | "img" | "ol" | "p" | "pre" | "table" | "ul"
    )
  }

  /// Returns true when the supplied node is a `<br>` element.
  fn is_br_element(node: &NodeRef<'_, Node>) -> bool {
    matches!(node.value(), Node::Element(element) if element.name() == "br")
  }

  /// Checks if a `<br>` begins a sequence that should be wrapped into `<p>`.
  fn is_break_chain_start(node: NodeRef<'_, Node>) -> bool {
    let mut next = node.next_sibling();

    while let Some(sibling) = next {
      if Self::is_whitespace_text(&sibling) {
        next = sibling.next_sibling();
        continue;
      }

      return Self::is_br_element(&sibling);
    }

    false
  }

  /// Identifies empty structural wrappers that only hold whitespace.
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

  /// Returns true when the tag name is one of the known phrasing elements.
  fn is_phrasing_element(tag: &str) -> bool {
    matches!(
      tag,
      "abbr"
        | "audio"
        | "b"
        | "bdi"
        | "bdo"
        | "br"
        | "button"
        | "cite"
        | "code"
        | "data"
        | "datalist"
        | "dfn"
        | "em"
        | "embed"
        | "i"
        | "img"
        | "input"
        | "kbd"
        | "label"
        | "mark"
        | "math"
        | "meter"
        | "noscript"
        | "object"
        | "output"
        | "progress"
        | "q"
        | "ruby"
        | "samp"
        | "script"
        | "select"
        | "small"
        | "span"
        | "strong"
        | "sub"
        | "sup"
        | "textarea"
        | "time"
        | "var"
        | "wbr"
    )
  }

  /// Returns true when a node qualifies as phrasing content.
  fn is_phrasing_node(node: &NodeRef<'_, Node>) -> bool {
    match node.value() {
      Node::Text(_) => true,
      Node::Element(element) => Self::is_phrasing_element(element.name()),
      _ => false,
    }
  }

  /// Returns true if the node is a whitespace-only text node.
  fn is_whitespace_text(node: &NodeRef<'_, Node>) -> bool {
    matches!(
      node.value(),
      Node::Text(text) if text.trim().is_empty()
    )
  }

  /// Returns true if the mutable node is a whitespace-only text node.
  fn node_mut_is_whitespace_text(
    node: &mut ego_tree::NodeMut<'_, Node>,
  ) -> bool {
    matches!(
      node.value(),
      Node::Text(text) if text.trim().is_empty()
    )
  }

  /// Converts lightweight wrappers into more canonical block elements.
  fn normalize_containers(html: &mut Html) {
    let mut to_convert = Vec::new();

    for node in html.tree.root().descendants() {
      let Node::Element(element) = node.value() else {
        continue;
      };

      match element.name() {
        "div" if !Self::has_block_child(node) => {
          to_convert.push((node.id(), "p"));
        }
        "section" | "header" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
          if Self::is_empty_container(node) =>
        {
          to_convert.push((node.id(), "div"));
        }
        _ => {}
      }
    }

    for (id, tag) in to_convert {
      if let Some(mut node) = html.tree.get_mut(id)
        && let Node::Element(element) = node.value()
      {
        Self::set_element_tag(element, tag);
      }
    }
  }

  /// Removes script, noscript, and style elements from the tree.
  fn remove_disallowed_nodes(html: &mut Html) {
    let mut to_remove = Vec::new();

    for node in html.tree.root().descendants() {
      if matches!(
        node.value(),
        Node::Element(element)
          if matches!(element.name(), "script" | "noscript" | "style")
      ) {
        to_remove.push(node.id());
      }
    }

    for id in to_remove {
      if let Some(mut node) = html.tree.get_mut(id) {
        node.detach();
      }
    }
  }

  /// Drops nodes matching Mozilla’s "unlikely candidate" heuristics.
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

      if !match_string.is_empty()
        && REGEX_UNLIKELY_CANDIDATES.is_match(&match_string)
        && !REGEX_OK_MAYBE_CANDIDATE.is_match(&match_string)
        && !Self::has_ancestor_tag(node, &["table", "code"])
      {
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

  /// Collapses `<br>` chains into paragraphs so inline content becomes blocks.
  fn replace_break_sequences(html: &mut Html) {
    let mut chain_starts = Vec::new();

    for node in html.tree.root().descendants() {
      if Self::is_br_element(&node) && Self::is_break_chain_start(node) {
        chain_starts.push(node.id());
      }
    }

    for id in chain_starts {
      Self::convert_br_chain(html, id);
    }
  }

  /// Re-tags legacy `<font>` elements as `<span>` nodes to avoid styling noise.
  fn rewrite_font_tags(html: &mut Html) {
    let mut to_rewrite = Vec::new();

    for node in html.tree.root().descendants() {
      if let Node::Element(element) = node.value()
        && element.name() == "font"
      {
        to_rewrite.push(node.id());
      }
    }

    for id in to_rewrite {
      if let Some(mut node) = html.tree.get_mut(id)
        && let Node::Element(element) = node.value()
      {
        Self::set_element_tag(element, "span");
      }
    }
  }

  /// Retags an existing element while leaving its attributes untouched.
  fn set_element_tag(element: &mut Element, tag: &str) {
    element.name = QualName::new(None, ns!(html), LocalName::from(tag));
  }

  /// Prunes leading and trailing whitespace-only children from a paragraph.
  fn trim_whitespace(html: &mut Html, para_id: NodeId) {
    if let Some(mut paragraph) = html.tree.get_mut(para_id) {
      while let Some(mut child) = paragraph.first_child() {
        if Self::node_mut_is_whitespace_text(&mut child) {
          child.detach();
        } else {
          break;
        }
      }

      while let Some(mut child) = paragraph.last_child() {
        if Self::node_mut_is_whitespace_text(&mut child) {
          child.detach();
        } else {
          break;
        }
      }
    }
  }
}
