use super::*;

pub struct RemoveDisallowedNodes;

impl Stage for RemoveDisallowedNodes {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    context.document.select("script, style, noscript").remove();

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  test! {
    name: removes_script_tags,
    stage: RemoveDisallowedNodes,
    content: "<html><body><script>alert('hi');</script><p>Content</p></body></html>",
    expected: "<html><head></head><body><p>Content</p></body></html>",
  }

  test! {
    name: removes_style_tags,
    stage: RemoveDisallowedNodes,
    content: "<html><head><style>body { color: red; }</style></head><body><p>Content</p></body></html>",
    expected: "<html><head></head><body><p>Content</p></body></html>",
  }

  test! {
    name: removes_noscript_tags,
    stage: RemoveDisallowedNodes,
    content: "<html><body><noscript>Enable JS</noscript><p>Content</p></body></html>",
    expected: "<html><head></head><body><p>Content</p></body></html>",
  }
}
