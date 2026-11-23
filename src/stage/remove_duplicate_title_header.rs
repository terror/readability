use super::*;

const TITLE_SIMILARITY_THRESHOLD: f64 = 0.75;

pub(crate) struct RemoveDuplicateTitleHeader;

impl Stage for RemoveDuplicateTitleHeader {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let title = context.metadata.title.clone().unwrap_or_default();

    let header = context
      .document
      .select("h1, h2")
      .nodes()
      .iter()
      .find(|node| {
        text_similarity(&title, node.text().trim()) > TITLE_SIMILARITY_THRESHOLD
      })
      .cloned();

    if let Some(header) = header {
      header.remove_from_parent();
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_first_matching_header() {
    Test::new()
      .stage(RemoveDuplicateTitleHeader)
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

  #[test]
  fn keeps_non_matching_header() {
    Test::new()
      .stage(RemoveDuplicateTitleHeader)
      .document("<html><body><h1>baz</h1></body></html>")
      .metadata(Metadata {
        title: Some("foo bar".into()),
        ..Metadata::default()
      })
      .expected_html("<html><head></head><body><h1>baz</h1></body></html>")
      .run();
  }
}
