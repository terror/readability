use {super::*, html5ever::serialize::Serialize};

pub(crate) struct SerializableNode<'a> {
  pub(crate) node: NodeRef<'a, Node>,
}

impl Serialize for SerializableNode<'_> {
  fn serialize<S: Serializer>(
    &self,
    serializer: &mut S,
    traversal_scope: TraversalScope,
  ) -> io::Result<()> {
    for edge in self.node.traverse() {
      match edge {
        Edge::Open(node) => {
          if node == self.node
            && traversal_scope == TraversalScope::ChildrenOnly(None)
          {
            continue;
          }

          match node.value() {
            Node::Doctype(doctype) => {
              serializer.write_doctype(doctype.name())?
            }
            Node::Comment(comment) => serializer.write_comment(comment)?,
            Node::Text(text) => serializer.write_text(text)?,
            Node::Element(element) => {
              serializer.start_elem(
                element.name.clone(),
                element.attrs.iter().map(|(name, value)| (name, &value[..])),
              )?;
            }
            _ => {}
          }
        }
        Edge::Close(node) => {
          if node == self.node
            && traversal_scope == TraversalScope::ChildrenOnly(None)
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
}
