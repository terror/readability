use super::*;

const SHARE_ELEMENT_THRESHOLD: usize = 500;

pub(crate) struct CleanShareElements;

impl CleanShareElements {
  fn clean(node: &NodeRef) {
    for child in node.element_children() {
      if SHARE_ELEMENTS.is_match(&child.match_string())
        && text_length(&child.text()) < SHARE_ELEMENT_THRESHOLD
      {
        child.remove_from_parent();
      } else {
        Self::clean(&child);
      }
    }
  }
}

impl Stage for CleanShareElements {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(article) = context.extraction.article else {
      return Ok(());
    };

    for child in context.node(article).element_children() {
      Self::clean(&child);
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_share_elements() {
    Test::new()
      .stage(CleanShareElements)
      .article()
      .document(
        r#"<html><body><div class="share"><p>foo</p><div class="share">bar</div><div class="shared">baz</div></div></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><div class="share"><p>foo</p><div class="shared">baz</div></div></body></html>"#,
      )
      .run();
  }
}
