use super::*;

pub(crate) struct SimplifyNestedElements;

impl SimplifyNestedElements {
  fn simplify(node: NodeRef) {
    let is_container = node.has_any_tag(&["DIV", "SECTION"])
      && !node
        .attr("id")
        .is_some_and(|id| id.starts_with("readability"));

    if is_container {
      if node.is_element_without_content() {
        node.remove_from_parent();
        return;
      }

      if node.has_single_tag_inside_element("DIV")
        || node.has_single_tag_inside_element("SECTION")
      {
        let child = node.element_children()[0];

        for attribute in node.attrs() {
          child.set_attr(&attribute.name.local, &attribute.value);
        }

        node.replace_with(&child);

        Self::simplify(child);

        return;
      }
    }

    for child in node.element_children() {
      Self::simplify(child);
    }
  }
}

impl Stage for SimplifyNestedElements {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    if let Some(article) = context.extraction.article {
      for child in context.node(article).element_children() {
        Self::simplify(child);
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simplifies_nested_elements() {
    Test::new()
      .stage(SimplifyNestedElements)
      .article()
      .document(
        r#"<html><body><div id="readability-page-1"><div class="foo"><section id="bar"><p>baz</p></section></div><div> <br> </div></div></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><div id="readability-page-1"><section id="bar" class="foo"><p>baz</p></section></div></body></html>"#,
      )
      .run();
  }
}
