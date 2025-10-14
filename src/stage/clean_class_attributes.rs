use super::*;

const CLASSES_TO_PRESERVE: &[&str] = &["page"];

pub struct CleanClassAttributesStage;

impl Stage for CleanClassAttributesStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    Self::clean_classes(fragment);

    Ok(())
  }
}

impl CleanClassAttributesStage {
  fn clean_classes(fragment: &mut ArticleFragment) {
    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return;
    };

    let node_ids: Vec<NodeId> = root
      .descendants()
      .filter(|node| matches!(node.value(), Node::Element(_)))
      .map(|node| node.id())
      .collect();

    for node_id in node_ids {
      let Some(mut node) = fragment.html.tree.get_mut(node_id) else {
        continue;
      };

      let Node::Element(element) = node.value() else {
        continue;
      };

      let class_index = element
        .attrs
        .iter()
        .position(|(name, _)| name.local.as_ref() == "class");

      let Some(index) = class_index else {
        continue;
      };

      let class_value = element.attrs[index].1.to_string();

      let preserved: Vec<&str> = class_value
        .split_whitespace()
        .filter(|class_name| CLASSES_TO_PRESERVE.contains(class_name))
        .collect();

      if preserved.is_empty() {
        element.attrs.remove(index);
      } else {
        let new_value = preserved.join(" ");
        element.attrs[index].1.clear();
        element.attrs[index].1.push_slice(&new_value);
      }
    }
  }
}
