use super::*;

pub(crate) struct CleanClasses;

impl Stage for CleanClasses {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    if context.options.keep_classes {
      return Ok(());
    }

    if let Some(article) = context.article {
      Self::clean(&context.node(article), &context.options.preserved_classes);
    }

    Ok(())
  }
}

impl CleanClasses {
  fn clean(node: &NodeRef, preserved: &[String]) {
    let class = node
      .attr("class")
      .unwrap_or_default()
      .split_whitespace()
      .filter(|class| preserved.iter().any(|preserved| preserved == class))
      .collect::<Vec<_>>()
      .join(" ");

    if class.is_empty() {
      node.remove_attr("class");
    } else {
      node.set_attr("class", &class);
    }

    for child in node.element_children() {
      Self::clean(&child, preserved);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_unpreserved_classes() {
    Test::new()
      .stage(CleanClasses)
      .article()
      .document(r#"<html><body class="foo"><div class="bar page baz"><p class="qux">quux</p></div></body></html>"#)
      .expected_html(r#"<html><head></head><body><div class="page"><p>quux</p></div></body></html>"#)
      .run();
  }
}
