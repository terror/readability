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

impl Stage for RemoveHiddenNodes {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let nodes = context.document.select("*").nodes().to_vec();

    for node in nodes {
      if node.parent().is_none() {
        continue;
      }

      if Self::is_hidden(&node) {
        node.remove_from_parent();
      }
    }

    Ok(())
  }
}

impl RemoveHiddenNodes {
  fn is_hidden(node: &NodeRef) -> bool {
    if node.attr("hidden").is_some() {
      return true;
    }

    let style = node
      .attr("style")
      .map(|style| style.to_lowercase())
      .unwrap_or_default();

    if style.contains("display:none") || style.contains("display: none") {
      return true;
    }

    if style.contains("visibility:hidden")
      || style.contains("visibility: hidden")
    {
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

    if node
      .attr("aria-modal")
      .is_some_and(|value| value.as_ref() == "true")
      && node
        .attr("role")
        .is_some_and(|value| value.as_ref() == "dialog")
    {
      return true;
    }

    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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

  #[test]
  fn removes_hidden_attribute() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(r"<html><body><div hidden>foo</div><p>bar</p></body></html>")
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
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
  fn removes_children_with_parent() {
    Test::new()
      .stage(RemoveHiddenNodes)
      .document(
        r"<html><body><div hidden><p>nested</p></div><p>bar</p></body></html>",
      )
      .expected_html("<html><head></head><body><p>bar</p></body></html>")
      .run();
  }
}
