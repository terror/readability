use super::*;

pub struct NormalizeArticleWhitespaceStage;

impl Stage for NormalizeArticleWhitespaceStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    Self::normalize_whitespace_nodes(fragment);

    Ok(())
  }
}

impl NormalizeArticleWhitespaceStage {
  fn is_preserved_whitespace_context(mut node: NodeRef<'_, Node>) -> bool {
    while let Some(parent) = node.parent() {
      if let Node::Element(element) = parent.value() {
        match element.name() {
          "pre" | "code" | "textarea" | "script" | "style" => return true,
          _ => {}
        }
      }

      node = parent;
    }

    false
  }

  fn normalize_whitespace_nodes(fragment: &mut ArticleFragment) {
    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return;
    };

    let text_nodes = root
      .descendants()
      .filter(|node| matches!(node.value(), Node::Text(_)))
      .map(|node| node.id())
      .collect::<Vec<NodeId>>();

    for node_id in text_nodes {
      let Some(node_ref) = fragment.html.tree.get(node_id) else {
        continue;
      };

      if Self::is_preserved_whitespace_context(node_ref) {
        continue;
      }

      let Some(mut node_mut) = fragment.html.tree.get_mut(node_id) else {
        continue;
      };

      let Node::Text(text_node) = node_mut.value() else {
        continue;
      };

      let original = text_node.to_string();

      if original.trim().is_empty() {
        continue;
      }

      let mut normalized = String::with_capacity(original.len());
      let mut last_was_space = false;

      for ch in original.chars() {
        match ch {
          '\n' | '\r' | '\t' | ' ' => {
            if !last_was_space {
              normalized.push(' ');
              last_was_space = true;
            }
          }
          ch => {
            normalized.push(ch);
            last_was_space = ch == ' ';
          }
        }
      }

      if normalized != original {
        text_node.text.clear();
        text_node.text.push_slice(&normalized);
      }
    }
  }
}
