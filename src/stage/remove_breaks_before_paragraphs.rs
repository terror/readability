use super::*;

pub(crate) struct RemoveBreaksBeforeParagraphs;

impl Stage for RemoveBreaksBeforeParagraphs {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(article) = context.extraction.article else {
      return Ok(());
    };

    for br in context.node(article).elements_with_tags(&["BR"]) {
      let mut next = br.next_sibling();

      while let Some(node) = next
        .clone()
        .filter(|node| !node.is_element() && node.text().trim().is_empty())
      {
        next = node.next_sibling();
      }

      if next.is_some_and(|node| node.tag() == "P") {
        br.remove_from_parent();
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_breaks_before_paragraphs() {
    Test::new()
      .stage(RemoveBreaksBeforeParagraphs)
      .article()
      .document("<html><body><div>foo<br> <p>bar</p><br><span>baz</span></div></body></html>")
      .expected_html(
        "<html><head></head><body><div>foo <p>bar</p><br><span>baz</span></div></body></html>",
      )
      .run();
  }
}
