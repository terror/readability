use super::*;

/// Converts layout tables that only wrap a single cell into semantic containers.
pub struct FlattenSimpleTablesStage;

impl Stage for FlattenSimpleTablesStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    Self::flatten(fragment);

    Ok(())
  }
}

impl FlattenSimpleTablesStage {
  const WRAPPER_TAGS: &'static [&'static str] =
    &["tbody", "thead", "tfoot", "tr", "td"];

  fn collect_tables(fragment: &ArticleFragment) -> Vec<NodeId> {
    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return Vec::new();
    };

    root
      .descendants()
      .filter(|node| matches!(node.value(), Node::Element(el) if el.name() == "table"))
      .map(|node| node.id())
      .collect()
  }

  fn flatten(fragment: &mut ArticleFragment) {
    let tables = Self::collect_tables(fragment);

    for table_id in tables {
      Self::flatten_table(fragment, table_id);
    }
  }

  fn flatten_table(fragment: &mut ArticleFragment, table_id: NodeId) {
    let should_flatten = {
      let Some(node) = fragment.html.tree.get(table_id) else {
        return;
      };

      let Node::Element(element) = node.value() else {
        return;
      };

      if element.name() != "table" {
        return;
      }

      Self::is_simple_table(node)
    };

    if !should_flatten {
      return;
    }

    Self::unwrap_wrappers(fragment, table_id);
    Self::lift_children_to_parent(fragment, table_id);
  }

  fn is_simple_table(node: ego_tree::NodeRef<'_, Node>) -> bool {
    let mut row_count = 0;
    let mut cell_count = 0;

    for descendant in node.descendants() {
      let Node::Element(element) = descendant.value() else {
        continue;
      };

      match element.name() {
        "tr" => row_count += 1,
        "td" => cell_count += 1,
        "th" | "col" | "colgroup" | "caption" => return false,
        _ => {}
      }
    }

    row_count <= 1 && cell_count <= 1 && cell_count > 0
  }

  fn is_whitespace(node: &NodeRef<'_, Node>) -> bool {
    match node.value() {
      Node::Text(text) => text.trim().is_empty(),
      _ => false,
    }
  }

  fn lift_children_to_parent(fragment: &mut ArticleFragment, node_id: NodeId) {
    let Some(children_owner) = fragment.html.tree.get(node_id) else {
      return;
    };

    if children_owner.parent().is_none() {
      return;
    }

    let children: Vec<NodeId> =
      children_owner.children().map(|child| child.id()).collect();

    for child_id in children {
      if let Some(mut current) = fragment.html.tree.get_mut(node_id) {
        current.insert_id_before(child_id);
      }
    }

    if let Some(mut current) = fragment.html.tree.get_mut(node_id) {
      current.detach();
    }
  }

  fn move_children(
    fragment: &mut ArticleFragment,
    parent_id: NodeId,
    child_id: NodeId,
  ) {
    let child_nodes: Vec<NodeId> = fragment
      .html
      .tree
      .get(child_id)
      .into_iter()
      .flat_map(|child| child.children().map(|node| node.id()))
      .collect();

    for node_id in child_nodes {
      if let Some(mut parent) = fragment.html.tree.get_mut(parent_id) {
        parent.append_id(node_id);
      }
    }

    if let Some(mut child) = fragment.html.tree.get_mut(child_id) {
      child.detach();
    }
  }

  fn single_wrapper_child(
    node: NodeRef<'_, Node>,
  ) -> Option<NodeRef<'_, Node>> {
    let mut element_child: Option<NodeRef<'_, Node>> = None;

    for child in node.children() {
      match child.value() {
        Node::Element(element) => {
          if !Self::WRAPPER_TAGS.contains(&element.name()) {
            return None;
          }

          if element_child.is_some() {
            return None;
          }

          element_child = Some(child);
        }
        _ => {
          if !Self::is_whitespace(&child) {
            return None;
          }
        }
      }
    }

    element_child
  }

  fn unwrap_wrappers(fragment: &mut ArticleFragment, table_id: NodeId) {
    loop {
      let child_id = {
        let Some(node) = fragment.html.tree.get(table_id) else {
          return;
        };

        let Some(wrapper) = Self::single_wrapper_child(node) else {
          break;
        };

        wrapper.id()
      };

      Self::move_children(fragment, table_id, child_id);
    }
  }
}
