use super::*;

const PRESENTATIONAL_ATTRIBUTES: &[&str] = &[
  "align",
  "background",
  "bgcolor",
  "border",
  "cellpadding",
  "cellspacing",
  "frame",
  "hspace",
  "rules",
  "style",
  "valign",
  "vspace",
];

const SIZE_ATTRIBUTE_ELEMENTS: &[&str] = &["table", "th", "td", "hr", "pre"];

/// Removes inline presentational attributes from the extracted article.
pub struct StripPresentationalAttributesStage;

impl Stage for StripPresentationalAttributesStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return Ok(());
    };

    let nodes = root
      .descendants()
      .filter(|node| matches!(node.value(), Node::Element(_)))
      .map(|node| node.id())
      .collect::<Vec<NodeId>>();

    for node_id in nodes {
      let Some(mut node) = fragment.html.tree.get_mut(node_id) else {
        continue;
      };

      let Node::Element(element) = node.value() else {
        continue;
      };

      element.attrs.retain(|(name, _)| {
        !PRESENTATIONAL_ATTRIBUTES.contains(&name.local.as_ref())
      });

      if SIZE_ATTRIBUTE_ELEMENTS.contains(&element.name()) {
        element.attrs.retain(|(name, _)| {
          !matches!(name.local.as_ref(), "width" | "height")
        });
      }
    }

    Ok(())
  }
}
