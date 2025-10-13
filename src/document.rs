use super::*;

static REGEX_NORMALIZE: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"\s{2,}").unwrap());

static REGEX_HASH_URL: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#.+").unwrap());

#[derive(Clone, Copy)]
pub(crate) struct Document<'a> {
  html: &'a Html,
}

impl<'a> Document<'a> {
  pub(crate) fn new(html: &'a Html) -> Self {
    Self { html }
  }

  pub(crate) fn count_elements(&self) -> usize {
    self
      .html
      .tree
      .root()
      .descendants()
      .filter(|node| node.value().is_element())
      .count()
  }

  pub(crate) fn root(&self) -> NodeRef<'a, Node> {
    self.html.tree.root()
  }

  pub(crate) fn html_element(&self) -> Option<NodeRef<'a, Node>> {
    self.root().children().find(
      |child| matches!(child.value(), Node::Element(el) if el.name() == "html"),
    )
  }

  pub(crate) fn body_element(&self) -> Option<NodeRef<'a, Node>> {
    self.html_element()?.children().find(
      |child| matches!(child.value(), Node::Element(el) if el.name() == "body"),
    )
  }

  pub(crate) fn node(&self, id: NodeId) -> Option<NodeRef<'a, Node>> {
    self.html.tree.get(id)
  }

  pub(crate) fn collect_text(
    &self,
    node_id: NodeId,
    normalize: bool,
  ) -> String {
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

  pub(crate) fn link_density(&self, node_id: NodeId) -> f64 {
    let text_length = self.collect_text(node_id, true).len() as f64;

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

          link_length += text.trim().len() as f64 * weight;
        }
      }
    }

    link_length / text_length
  }

  pub(crate) fn document_title(&self) -> Option<String> {
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
}
