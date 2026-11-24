use super::*;

const EMPTY_CONTAINER_TAGS: &[&str] = &[
  "DIV", "SECTION", "HEADER", "H1", "H2", "H3", "H4", "H5", "H6",
];

/// Removes `div`, `section`, `header`, and heading elements that have no
/// content.
///
/// A node is considered without content when its text is empty and it has no
/// element children, or exactly as many element children as `<br>` and `<hr>`
/// descendants.
pub(crate) struct RemoveEmptyContainers;

impl Rule for RemoveEmptyContainers {
  fn apply(&mut self, context: &mut Context<'_>, node: NodeId) -> Action {
    let node = context.node(node);

    if node.has_any_tag(EMPTY_CONTAINER_TAGS)
      && node.is_element_without_content()
    {
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
  fn keeps_div_with_non_br_hr_child() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><div><img src=\"foo.jpg\"></div></body></html>")
      .expected_html(
        "<html><head></head><body><div><img src=\"foo.jpg\"></div></body></html>",
      )
      .run();
  }

  #[test]
  fn keeps_div_with_text() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><div>foo</div></body></html>")
      .expected_html("<html><head></head><body><div>foo</div></body></html>")
      .run();
  }

  #[test]
  fn keeps_non_target_empty_tag() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><span></span><p>foo</p></body></html>")
      .expected_html(
        "<html><head></head><body><span></span><p>foo</p></body></html>",
      )
      .run();
  }

  #[test]
  fn removes_div_with_only_br() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><div><br></div><p>foo</p></body></html>")
      .expected_html("<html><head></head><body><p>foo</p></body></html>")
      .run();
  }

  #[test]
  fn removes_div_with_only_hr() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><div><hr></div><p>foo</p></body></html>")
      .expected_html("<html><head></head><body><p>foo</p></body></html>")
      .run();
  }

  #[test]
  fn removes_empty_div() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><div></div><p>foo</p></body></html>")
      .expected_html("<html><head></head><body><p>foo</p></body></html>")
      .run();
  }

  #[test]
  fn removes_empty_header() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><header></header><p>foo</p></body></html>")
      .expected_html("<html><head></head><body><p>foo</p></body></html>")
      .run();
  }

  #[test]
  fn removes_empty_headings() {
    #[track_caller]
    fn case(tag: &str) {
      Test::new()
        .stage(RemoveEmptyContainers)
        .document(&format!(
          "<html><body><{tag}></{tag}><p>foo</p></body></html>"
        ))
        .expected_html("<html><head></head><body><p>foo</p></body></html>")
        .run();
    }

    case("h1");
    case("h2");
    case("h3");
    case("h4");
    case("h5");
    case("h6");
  }

  #[test]
  fn removes_empty_section() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><section></section><p>foo</p></body></html>")
      .expected_html("<html><head></head><body><p>foo</p></body></html>")
      .run();
  }
}
