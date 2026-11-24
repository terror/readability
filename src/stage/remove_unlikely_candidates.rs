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
/// - one of the node's four nearest ancestors is a `<table>` or `<code>`
///   element
/// - the node is `<body>` or `<a>`
///
/// Additionally, nodes whose `role` attribute is in `UNLIKELY_ROLES` are
/// removed regardless of class or id.
///
/// Nothing is removed once a retry has disabled the `strip_unlikelys` flag.
pub(crate) struct RemoveUnlikelyCandidates;

impl RemoveUnlikelyCandidates {
  fn is_unlikely(node: &NodeRef) -> bool {
    let match_string = node.match_string();

    let tag = node.tag();

    if UNLIKELY_CANDIDATE.is_match(&match_string)
      && !MAYBE_CANDIDATE.is_match(&match_string)
      && !node.has_ancestor_tag("TABLE", Some(3), |_| true)
      && !node.has_ancestor_tag("CODE", Some(3), |_| true)
      && tag != "BODY"
      && tag != "A"
    {
      return true;
    }

    node
      .attr("role")
      .is_some_and(|role| UNLIKELY_ROLES.contains(&role.as_ref()))
  }
}

impl Rule for RemoveUnlikelyCandidates {
  fn apply(&mut self, context: &mut Context<'_>, node: NodeId) -> Action {
    if context.flags.strip_unlikelys && Self::is_unlikely(&context.node(node)) {
      Action::Remove
    } else {
      Action::Continue
    }
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
  fn skips_when_flag_disabled() {
    Test::new()
      .stage(RemoveUnlikelyCandidates)
      .document(r#"<html><body><div class="comment">foo</div></body></html>"#)
      .flags(Flags {
        strip_unlikelys: false,
        ..Flags::default()
      })
      .expected_html(
        r#"<html><head></head><body><div class="comment">foo</div></body></html>"#,
      )
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
