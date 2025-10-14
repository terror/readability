use super::*;

pub struct RewriteFontTagsStage;

impl Stage for RewriteFontTagsStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let html = context.html_mut();
    Self::rewrite_font_tags(html);
    Ok(())
  }
}

impl RewriteFontTagsStage {
  fn rewrite_font_tags(html: &mut Html) {
    let mut to_rewrite = Vec::new();

    for node in html.tree.root().descendants() {
      if let Node::Element(element) = node.value()
        && element.name() == "font"
      {
        to_rewrite.push(node.id());
      }
    }

    for id in to_rewrite {
      if let Some(mut node) = html.tree.get_mut(id)
        && let Node::Element(element) = node.value()
      {
        Self::set_element_tag(element, "span");
      }
    }
  }

  fn set_element_tag(element: &mut Element, tag: &str) {
    element.name = QualName::new(None, ns!(html), LocalName::from(tag));
  }
}
