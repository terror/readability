use super::*;

const EMBED_TAGS: &[&str] = &["OBJECT", "EMBED", "IFRAME"];

pub(crate) struct Clean(pub(crate) &'static str);

impl Stage for Clean {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(article) = context.extraction.article else {
      return Ok(());
    };

    let is_embed = EMBED_TAGS.contains(&self.0);

    for element in context
      .node(article)
      .elements_with_tags(&[self.0])
      .iter()
      .rev()
    {
      if is_embed && is_allowed_video(element) {
        continue;
      }

      element.remove_from_parent();
    }

    Ok(())
  }
}

pub(crate) fn is_allowed_video(element: &NodeRef) -> bool {
  element
    .attrs()
    .iter()
    .any(|attribute| VIDEOS.is_match(&attribute.value))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_elements() {
    Test::new()
      .stage(Clean("ASIDE"))
      .article()
      .document("<html><body><aside>foo</aside><p>bar</p></body></html>")
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn keeps_allowed_videos() {
    Test::new()
      .stage(Clean("IFRAME"))
      .article()
      .document(
        r#"<html><body><iframe src="https://www.youtube.com/embed/foo"></iframe><iframe src="https://bar.com"></iframe></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><iframe src="https://www.youtube.com/embed/foo"></iframe></body></html>"#,
      )
      .run();
  }
}
