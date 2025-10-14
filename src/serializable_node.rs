use super::*;

pub(crate) struct SerializableNode<'a> {
  pub(crate) node: NodeRef<'a, Node>,
}

impl html5ever::serialize::Serialize for SerializableNode<'_> {
  fn serialize<S: Serializer>(
    &self,
    serializer: &mut S,
    traversal_scope: TraversalScope,
  ) -> std::io::Result<()> {
    serialize_node(self.node, serializer, traversal_scope)
  }
}

fn serialize_node<S: Serializer>(
  root: NodeRef<'_, Node>,
  serializer: &mut S,
  traversal_scope: TraversalScope,
) -> io::Result<()> {
  for edge in root.traverse() {
    match edge {
      Edge::Open(node) => {
        if node == root && traversal_scope == TraversalScope::ChildrenOnly(None)
        {
          continue;
        }

        match node.value() {
          Node::Doctype(doctype) => serializer.write_doctype(doctype.name())?,
          Node::Comment(comment) => serializer.write_comment(comment)?,
          Node::Text(text) => serializer.write_text(text)?,
          Node::Element(element) => {
            let attrs =
              element.attrs.iter().map(|(name, value)| (name, &value[..]));
            serializer.start_elem(element.name.clone(), attrs)?;
          }
          _ => {}
        }
      }
      Edge::Close(node) => {
        if node == root && traversal_scope == TraversalScope::ChildrenOnly(None)
        {
          continue;
        }

        if let Some(element) = node.value().as_element() {
          serializer.end_elem(element.name.clone())?;
        }
      }
    }
  }

  Ok(())
}
