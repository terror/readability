use super::*;

const TITLE_SIMILARITY_THRESHOLD: f64 = 0.75;

#[derive(Default)]
pub(crate) struct RemoveDuplicateTitleHeader {
  removed: bool,
}

impl Rule for RemoveDuplicateTitleHeader {
  fn apply(&mut self, context: &mut Context<'_>, node: NodeId) -> Action {
    if self.removed {
      return Action::Continue;
    }

    let node = context.node(node);

    if !node.has_any_tag(&["H1", "H2"]) {
      return Action::Continue;
    }

    let title = context.metadata.title.as_deref().unwrap_or_default();

    if text_similarity(title, node.text().trim()) > TITLE_SIMILARITY_THRESHOLD {
      self.removed = true;
      Action::Remove
    } else {
      Action::Continue
    }
  }

  fn reset(&mut self) {
    self.removed = false;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn keeps_non_matching_header() {
    Test::new()
      .stage(RemoveDuplicateTitleHeader::default())
      .document("<html><body><h1>baz</h1></body></html>")
      .metadata(Metadata {
        title: Some("foo bar".into()),
        ..Metadata::default()
      })
      .expected_html("<html><head></head><body><h1>baz</h1></body></html>")
      .run();
  }

  #[test]
  fn removes_first_matching_header() {
    Test::new()
      .stage(RemoveDuplicateTitleHeader::default())
      .document("<html><body><h2>baz</h2><h1>foo bar</h1><h2>foo bar</h2></body></html>")
      .metadata(Metadata {
        title: Some("foo bar".into()),
        ..Metadata::default()
      })
      .expected_html(
        "<html><head></head><body><h2>baz</h2><h2>foo bar</h2></body></html>",
      )
      .run();
  }
}
