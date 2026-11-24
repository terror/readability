use super::*;

pub(crate) struct CleanClasses;

impl Stage for CleanClasses {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    if context.options.keep_classes {
      return Ok(());
    }

    let Some(article) = context.extraction.article else {
      return Ok(());
    };

    let preserved = &context.options.classes_to_preserve;

    let mut stack = vec![context.node(article)];

    while let Some(node) = stack.pop() {
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

      stack.extend(node.element_children());
    }

    Ok(())
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
