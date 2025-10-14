use super::*;

const WRAPPER_CLASS: &str = "page";
const WRAPPER_ID: &str = "readability-page-1";

#[derive(Debug)]
pub(crate) struct ArticleFragment {
  pub(crate) html: Html,
  pub(crate) root_id: NodeId,
}

impl ArticleFragment {
  pub(crate) fn from_markup(markup: &str) -> Self {
    let wrapped = format!(
      "<div id=\"{WRAPPER_ID}\" class=\"{WRAPPER_CLASS}\">{markup}</div>"
    );

    let html = Html::parse_fragment(&wrapped);

    let root_id = html
      .tree
      .root()
      .descendants()
      .find(|node| {
        matches!(
          node.value(),
          Node::Element(element) if element.id() == Some(WRAPPER_ID)
        )
      })
      .map_or_else(|| html.tree.root().id(), |node| node.id());

    Self::new(html, root_id)
  }

  pub(crate) fn into_markup(self) -> Option<String> {
    self
      .html
      .tree
      .get(self.root_id)
      .map(Self::serialize_children)
      .map(|markup| {
        format!(
          "<div id=\"{WRAPPER_ID}\" class=\"{WRAPPER_CLASS}\">{markup}</div>"
        )
      })
  }

  pub(crate) fn new(html: Html, root_id: NodeId) -> Self {
    Self { html, root_id }
  }

  fn serialize_children(node: NodeRef<'_, Node>) -> String {
    let opts = SerializeOpts {
      scripting_enabled: false,
      traversal_scope: TraversalScope::ChildrenOnly(None),
      create_missing_parent: false,
    };

    let mut buffer = Vec::new();

    let serializer = SerializableNode { node };

    if serialize(&mut buffer, &serializer, opts).is_ok() {
      String::from_utf8(buffer).unwrap_or_default()
    } else {
      String::new()
    }
  }
}
