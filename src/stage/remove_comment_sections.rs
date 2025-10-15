use super::*;

static REGEX_COMMENTS: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r"(?i)comment|comments|discussion|discuss|respond|reply|talkback")
    .unwrap()
});

const TARGET_TAGS: &[&str] = &["div", "section", "aside", "ul", "ol"];

/// Removes obvious comment or discussion sections from the article fragment.
pub struct RemoveCommentSectionsStage;

impl Stage for RemoveCommentSectionsStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return Ok(());
    };

    let mut to_remove = Vec::new();

    for node in root.descendants() {
      let Node::Element(element) = node.value() else {
        continue;
      };

      if !TARGET_TAGS.contains(&element.name()) {
        continue;
      }

      let matches_comment = element
        .attr("id")
        .into_iter()
        .chain(element.attr("class"))
        .any(|value| REGEX_COMMENTS.is_match(value));

      if matches_comment {
        to_remove.push(node.id());
        continue;
      }

      if let Some(role) = element.attr("role")
        && matches!(role, "complementary" | "feed" | "navigation")
      {
        if element
          .attr("aria-label")
          .into_iter()
          .any(|value| REGEX_COMMENTS.is_match(value))
        {
          to_remove.push(node.id());
        }
      }
    }

    for node_id in to_remove {
      if let Some(mut node) = fragment.html.tree.get_mut(node_id) {
        node.detach();
      }
    }

    Ok(())
  }
}
