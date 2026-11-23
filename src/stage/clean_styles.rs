use super::*;

const DEPRECATED_SIZE_ATTRIBUTE_ELEMS: &[&str] =
  &["TABLE", "TH", "TD", "HR", "PRE"];

const PRESENTATIONAL_ATTRIBUTES: &[&str] = &[
  "align",
  "background",
  "bgcolor",
  "border",
  "cellpadding",
  "cellspacing",
  "frame",
  "hspace",
  "rules",
  "style",
  "valign",
  "vspace",
];

pub(crate) struct CleanStyles;

impl Stage for CleanStyles {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    if let Some(article) = context.article {
      Self::clean(&context.node(article));
    }

    Ok(())
  }
}

impl CleanStyles {
  fn clean(node: &NodeRef) {
    let tag = node.tag();

    if tag == "SVG" {
      return;
    }

    node.remove_attrs(PRESENTATIONAL_ATTRIBUTES);

    if DEPRECATED_SIZE_ATTRIBUTE_ELEMS.contains(&tag.as_str()) {
      node.remove_attrs(&["width", "height"]);
    }

    for child in node.element_children() {
      Self::clean(&child);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_presentational_attributes() {
    Test::new()
      .stage(CleanStyles)
      .article()
      .document(
        r#"<html><body><p style="foo" align="bar" class="baz">qux</p><table width="1"><tbody><tr><td height="1"></td></tr></tbody></table><img width="1"><svg style="foo"><g style="bar"></g></svg></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><p class="baz">qux</p><table><tbody><tr><td></td></tr></tbody></table><img width="1"><svg style="foo"><g style="bar"></g></svg></body></html>"#,
      )
      .run();
  }
}
