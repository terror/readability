use super::*;

/// Scans the document for a byline element and stores its text in the context
/// metadata. Only runs when no byline has already been extracted from metadata.
///
/// An element is a byline candidate if:
/// - its `rel` attribute equals `"author"`, or
/// - its `itemprop` attribute contains `"author"`, or
/// - its `class` and `id` attributes match `/byline|author|dateline|writtenby|p-author/i`
///
/// and its trimmed text is non-empty and fewer than 100 characters.
///
/// When a candidate is found, a descendant with `itemprop` containing `"name"`
/// and non-empty text is preferred for the byline text.
///
/// Candidates are skipped if any ancestor would be removed as an unlikely
/// candidate by the JS readability algorithm.
pub(crate) struct ExtractByline;

const UNLIKELY_ROLES: &[&str] = &[
  "menu",
  "menubar",
  "complementary",
  "navigation",
  "alert",
  "alertdialog",
  "dialog",
];

impl Stage for ExtractByline {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    if context.metadata.byline.is_some() {
      return Ok(());
    }

    let nodes = context.document.select("*").nodes().to_vec();

    for node in nodes {
      let rel = node.attr("rel").unwrap_or_default();
      let itemprop = node.attr("itemprop").unwrap_or_default();

      let class = node.attr("class").unwrap_or_default();
      let id = node.attr("id").unwrap_or_default();

      let is_byline_candidate = rel.as_ref() == "author"
        || itemprop.contains("author")
        || BYLINE.is_match(&format!("{class} {id}"));

      if !is_byline_candidate {
        continue;
      }

      let should_skip = node.is_hidden()
        || node.ancestors(None).iter().any(|ancestor| {
          if ancestor.is_hidden() {
            return true;
          }

          let ancestor_class = ancestor.attr("class").unwrap_or_default();
          let ancestor_id = ancestor.attr("id").unwrap_or_default();
          let ancestor_match = format!("{ancestor_class} {ancestor_id}");
          let ancestor_role = ancestor.attr("role").unwrap_or_default();

          let tag = ancestor
            .node_name()
            .map(|n| n.to_uppercase())
            .unwrap_or_default();

          let is_unlikely_by_class = UNLIKELY_CANDIDATE
            .is_match(&ancestor_match)
            && !MAYBE_CANDIDATE.is_match(&ancestor_match)
            && tag != "BODY"
            && tag != "A";

          let is_unlikely_by_role = UNLIKELY_ROLES
            .iter()
            .any(|&role| ancestor_role.as_ref() == role);

          is_unlikely_by_class || is_unlikely_by_role
        });

      if should_skip {
        continue;
      }

      let text = node.text();
      let text = text.trim();

      if text.is_empty() || text.len() >= 100 {
        continue;
      }

      let byline = node
        .descendants_it()
        .find(|node| {
          node
            .attr("itemprop")
            .is_some_and(|value| value.contains("name"))
            && !node.text().trim().is_empty()
        })
        .map_or_else(
          || text.to_string(),
          |node| node.text().trim().to_string(),
        );

      context.metadata.byline = Some(byline);

      break;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn class_author() {
    Test::new()
      .stage(ExtractByline)
      .document(
        r#"<html><body><div class="article-author">foo</div></body></html>"#,
      )
      .expected_metadata(Metadata {
        byline: Some("foo".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn class_byline() {
    Test::new()
      .stage(ExtractByline)
      .document(r#"<html><body><p class="byline">foo</p></body></html>"#)
      .expected_metadata(Metadata {
        byline: Some("foo".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn empty_text_skipped() {
    Test::new()
      .stage(ExtractByline)
      .document(r#"<html><body><p class="byline">   </p></body></html>"#)
      .expected_metadata(Metadata::default())
      .run();
  }

  #[test]
  fn existing_byline_not_overwritten() {
    Test::new()
      .stage(ExtractByline)
      .document(r#"<html><body><p class="byline">bar</p></body></html>"#)
      .metadata(Metadata {
        byline: Some("foo".into()),
        ..Metadata::default()
      })
      .expected_metadata(Metadata {
        byline: Some("foo".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn id_author() {
    Test::new()
      .stage(ExtractByline)
      .document(r#"<html><body><div id="author">foo</div></body></html>"#)
      .expected_metadata(Metadata {
        byline: Some("foo".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn itemprop_author() {
    Test::new()
      .stage(ExtractByline)
      .document(
        r#"<html><body><span itemprop="author">foo</span></body></html>"#,
      )
      .expected_metadata(Metadata {
        byline: Some("foo".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn prefers_itemprop_name_descendant() {
    Test::new()
      .stage(ExtractByline)
      .document(
        r#"<html><body><span itemprop="author"><span itemprop="name">foo</span> extra</span></body></html>"#,
      )
      .expected_metadata(Metadata {
        byline: Some("foo".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn rel_author() {
    Test::new()
      .stage(ExtractByline)
      .document(r#"<html><body><a rel="author">foo</a></body></html>"#)
      .expected_metadata(Metadata {
        byline: Some("foo".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn too_long_skipped() {
    Test::new()
      .stage(ExtractByline)
      .document(&format!(
        r#"<html><body><p class="byline">{}</p></body></html>"#,
        "a".repeat(100)
      ))
      .expected_metadata(Metadata::default())
      .run();
  }
}
