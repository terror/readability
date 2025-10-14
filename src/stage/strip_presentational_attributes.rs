use super::*;

/// Removes inline presentational attributes from the extracted article.
pub struct StripPresentationalAttributesStage;

impl Stage for StripPresentationalAttributesStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    Self::strip_attributes(fragment);

    Ok(())
  }
}

impl StripPresentationalAttributesStage {
  const PRESENTATIONAL_ATTRIBUTES: &'static [&'static str] = &[
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

  const SIZE_ATTRIBUTE_ELEMENTS: &'static [&'static str] =
    &["table", "th", "td", "hr", "pre"];

  fn strip_attributes(fragment: &mut ArticleFragment) {
    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return;
    };

    let nodes: Vec<NodeId> = root
      .descendants()
      .filter(|node| matches!(node.value(), Node::Element(_)))
      .map(|node| node.id())
      .collect();

    for node_id in nodes {
      let Some(mut node) = fragment.html.tree.get_mut(node_id) else {
        continue;
      };

      let Node::Element(element) = node.value() else {
        continue;
      };

      element.attrs.retain(|(name, _)| {
        !Self::PRESENTATIONAL_ATTRIBUTES.contains(&name.local.as_ref())
      });

      if Self::SIZE_ATTRIBUTE_ELEMENTS.contains(&element.name()) {
        element.attrs.retain(|(name, _)| {
          !matches!(name.local.as_ref(), "width" | "height")
        });
      }
    }
  }
}
