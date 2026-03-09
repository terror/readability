use super::*;

const UNLIKELY_ROLES: &[&str] = &[
  "menu",
  "menubar",
  "complementary",
  "navigation",
  "alert",
  "alertdialog",
  "dialog",
];

/// Removes nodes that are unlikely to contain article content.
///
/// A node is removed when its combined `class` and `id` match the
/// `UNLIKELY_CANDIDATE` pattern and none of the following exceptions apply:
/// - the `MAYBE_CANDIDATE` pattern also matches
/// - the node has an ancestor `<table>` or `<code>` element
/// - the node is `<body>` or `<a>`
///
/// Additionally, nodes whose `role` attribute is in `UNLIKELY_ROLES` are
/// removed regardless of class or id.
pub(crate) struct RemoveUnlikelyCandidates;

impl Stage for RemoveUnlikelyCandidates {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let nodes = context.document.select("*").nodes().to_vec();

    for node in nodes {
      if node.parent().is_none() {
        continue;
      }

      let tag = node
        .node_name()
        .map(|node| node.to_uppercase())
        .unwrap_or_default();

      if tag == "BODY" || tag == "A" {
        continue;
      }

      let role = node.attr("role").unwrap_or_default();

      if UNLIKELY_ROLES.iter().any(|&r| role.as_ref() == r) {
        node.remove_from_parent();
        continue;
      }

      let class = node.attr("class").unwrap_or_default();
      let id = node.attr("id").unwrap_or_default();
      let match_string = format!("{class} {id}");

      if !UNLIKELY_CANDIDATE.is_match(&match_string) {
        continue;
      }

      if MAYBE_CANDIDATE.is_match(&match_string) {
        continue;
      }

      let has_table_or_code_ancestor = node.ancestors(None).iter().any(|a| {
        a.node_name().is_some_and(|n| {
          let upper = n.to_uppercase();
          upper == "TABLE" || upper == "CODE"
        })
      });

      if has_table_or_code_ancestor {
        continue;
      }

      node.remove_from_parent();
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_comment_class() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(
        r#"<html><body><div class="comment">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn removes_sidebar_id() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(
        r#"<html><body><div id="sidebar">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn keeps_article_class() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(
        r#"<html><body><div class="comment article">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><div class="comment article">foo</div><p>bar</p></body></html>"#,
      )
      .run();
  }

  #[test]
  fn keeps_body() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(r#"<html><body class="comment"><p>bar</p></body></html>"#)
      .expected_html(
        r#"<html><head></head><body class="comment"><p>bar</p></body></html>"#,
      )
      .run();
  }

  #[test]
  fn keeps_anchor() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(
        "<html><body><a class=\"comment\" href=\"#\">foo</a></body></html>",
      )
      .expected_html(
        "<html><head></head><body><a class=\"comment\" href=\"#\">foo</a></body></html>",
      )
      .run();
  }

  #[test]
  fn keeps_inside_table() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(
        r#"<html><body><table><tr><td class="comment">foo</td></tr></table></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><table><tbody><tr><td class="comment">foo</td></tr></tbody></table></body></html>"#,
      )
      .run();
  }

  #[test]
  fn keeps_inside_code() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(
        r#"<html><body><code><span class="comment">foo</span></code></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><code><span class="comment">foo</span></code></body></html>"#,
      )
      .run();
  }

  #[test]
  fn removes_unlikely_role() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(
        r#"<html><body><nav role="navigation">foo</nav><p>bar</p></body></html>"#,
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn removes_menu_role() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(
        r#"<html><body><div role="menu">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn keeps_unmatched_role() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(
        r#"<html><body><div role="main">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><div role="main">foo</div><p>bar</p></body></html>"#,
      )
      .run();
  }

  #[test]
  fn removes_children_with_parent() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(
        r#"<html><body><div class="sidebar"><p>nested</p></div><p>bar</p></body></html>"#,
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }
}
