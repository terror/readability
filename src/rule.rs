use super::*;

pub(crate) enum Action {
  Continue,
  Remove,
  Replace(NodeId),
}

pub(crate) trait Rule {
  fn apply(&mut self, context: &mut Context<'_>, node: NodeId) -> Action;

  fn reset(&mut self) {}
}

impl<T: Rule> Stage for T {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    traverse(context, &mut [self]);
    Ok(())
  }
}

pub(crate) fn traverse(context: &mut Context<'_>, rules: &mut [&mut dyn Rule]) {
  for rule in rules.iter_mut() {
    rule.reset();
  }

  let mut current = context
    .document
    .root()
    .first_element_child()
    .map(|node| node.id);

  'nodes: while let Some(id) = current {
    for rule in rules.iter_mut() {
      match rule.apply(context, id) {
        Action::Continue => {}
        Action::Remove => {
          let node = context.node(id);
          current = next_element(&node, true).map(|next| next.id);
          node.remove_from_parent();
          continue 'nodes;
        }
        Action::Replace(replacement) => {
          current =
            next_element(&context.node(replacement), false).map(|next| next.id);
          continue 'nodes;
        }
      }
    }

    current = next_element(&context.node(id), false).map(|next| next.id);
  }
}

fn next_element<'a>(
  node: &NodeRef<'a>,
  skip_children: bool,
) -> Option<NodeRef<'a>> {
  if !skip_children && let Some(child) = node.first_element_child() {
    return Some(child);
  }

  iter::once(node.clone())
    .chain(node.ancestors(None))
    .find_map(|node| node.next_element_sibling())
}
