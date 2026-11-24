use super::*;

/// Removes nodes that are not visible to the user before scoring.
///
/// A node is removed when any of the following conditions hold:
/// - its `style` attribute sets `display: none`
/// - its `style` attribute sets `visibility: hidden`
/// - it carries the `hidden` attribute
/// - it carries `aria-hidden="true"` and its `class` does not contain
///   `"fallback-image"` (Wikimedia math images use this class)
/// - it carries both `aria-modal="true"` and `role="dialog"`
pub(crate) struct RemoveHiddenNodes;

impl RemoveHiddenNodes {
  fn is_hidden(node: &NodeRef) -> bool {
    if node.attr("hidden").is_some() {
      return true;
    }

    if node.attr("style").is_some_and(|style| {
      DISPLAY_NONE.is_match(&style) || VISIBILITY_HIDDEN.is_match(&style)
    }) {
      return true;
    }

    if node
      .attr("aria-hidden")
      .is_some_and(|value| value.as_ref() == "true")
    {
      let class = node.attr("class").unwrap_or_default();

      if !class.contains("fallback-image") {
        return true;
      }
    }

    node
      .attr("aria-modal")
      .is_some_and(|value| value.as_ref() == "true")
      && node
        .attr("role")
        .is_some_and(|value| value.as_ref() == "dialog")
  }
}

impl Rule for RemoveHiddenNodes {
  fn apply(&mut self, context: &mut Context<'_>, node: NodeId) -> Action {
    if Self::is_hidden(&context.node(node)) {
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
  fn hidden_styles() {
    #[track_caller]
    fn case(style: &str, expected: bool) {
      let document = dom_query::Document::from(
        format!(r#"<div style="{style}"></div>"#).as_str(),
      );

      assert_eq!(
        RemoveHiddenNodes::is_hidden(&document.select("div").nodes()[0]),
        expected,
      );
    }

    case("display : none", true);
    case("color: red;display:\tnone !important", true);
    case("visibility :hidden", true);
    case("--foo-display:none", false);
  }

  #[test]
  fn keeps_aria_hidden_false() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(
        r#"<html><body><div aria-hidden="false">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><div aria-hidden="false">foo</div><p>bar</p></body></html>"#,
      )
      .run();
  }

  #[test]
  fn keeps_aria_modal_without_dialog_role() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(
        r#"<html><body><div aria-modal="true" role="alertdialog">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><div aria-modal="true" role="alertdialog">foo</div><p>bar</p></body></html>"#,
      )
      .run();
  }

  #[test]
  fn keeps_fallback_image_despite_aria_hidden() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(
        r#"<html><body><img aria-hidden="true" class="fallback-image" src="x.png"/><p>bar</p></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><img aria-hidden="true" class="fallback-image" src="x.png"><p>bar</p></body></html>"#,
      )
      .run();
  }

  #[test]
  fn removes_aria_hidden_true() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(
        r#"<html><body><div aria-hidden="true">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn removes_aria_modal_dialog() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(
        r#"<html><body><div aria-modal="true" role="dialog">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn removes_children_with_parent() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(
        r"<html><body><div hidden><p>nested</p></div><p>bar</p></body></html>",
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn removes_display_none() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(
        r#"<html><body><div style="display:none">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn removes_display_none_with_space() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(
        r#"<html><body><div style="display: none">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn removes_hidden_attribute() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(r"<html><body><div hidden>foo</div><p>bar</p></body></html>")
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn removes_visibility_hidden() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(
        r#"<html><body><div style="visibility:hidden">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }

  #[test]
  fn removes_visibility_hidden_with_space() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(
        r#"<html><body><div style="visibility: hidden">foo</div><p>bar</p></body></html>"#,
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }
}
