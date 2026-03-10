use super::*;

const EMPTY_CONTAINER_TAGS: &[&str] = &[
  "DIV", "SECTION", "HEADER", "H1", "H2", "H3", "H4", "H5", "H6",
];

/// Removes `div`, `section`, `header`, and heading elements that have no
/// content.
///
/// A node is considered without content when its text is empty and its only
/// children (if any) are `<br>` or `<hr>` elements.
pub(crate) struct RemoveEmptyContainers;

impl Stage for RemoveEmptyContainers {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let nodes = context.document.select("*").nodes().to_vec();

    for node in nodes {
      if node.parent().is_none() {
        continue;
      }

      let tag = node
        .node_name()
        .map(|node_name| node_name.to_uppercase())
        .unwrap_or_default();

      if !EMPTY_CONTAINER_TAGS.contains(&tag.as_str()) {
        continue;
      }

      if Self::is_without_content(&node) {
        node.remove_from_parent();
      }
    }

    Ok(())
  }
}

impl RemoveEmptyContainers {
  fn is_without_content(node: &dom_query::Node) -> bool {
    let text = node.text();

    if !text.trim().is_empty() {
      return false;
    }

    let children = node.children();

    let non_br_hr = children.iter().filter(|child| {
      child.node_name().is_some_and(|node_name| {
        let upper = node_name.to_uppercase();
        upper != "BR" && upper != "HR"
      })
    });

    non_br_hr.count() == 0
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_empty_div() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><div></div><p>foo</p></body></html>")
      .expected_html("<html><head></head><body><p>foo</p></body></html>")
      .run();
  }

  #[test]
  fn removes_empty_section() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><section></section><p>foo</p></body></html>")
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
  fn keeps_div_with_text() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><div>foo</div></body></html>")
      .expected_html("<html><head></head><body><div>foo</div></body></html>")
      .run();
  }

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
  fn keeps_non_target_empty_tag() {
    Test::new()
      .stage(RemoveEmptyContainers)
      .document("<html><body><span></span><p>foo</p></body></html>")
      .expected_html(
        "<html><head></head><body><span></span><p>foo</p></body></html>",
      )
      .run();
  }
}
