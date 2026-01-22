use super::*;

pub struct RewriteFontTags;

impl Stage for RewriteFontTags {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    context.document.select("font").rename("span");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  test! {
    name: converts_font_to_span,
    stage: RewriteFontTags,
    content: "<html><body><font>Hello</font></body></html>",
    expected: "<html><head></head><body><span>Hello</span></body></html>",
  }

  test! {
    name: preserves_font_tag_content,
    stage: RewriteFontTags,
    content: "<html><body><font>Hello <b>world</b></font></body></html>",
    expected: "<html><head></head><body><span>Hello <b>world</b></span></body></html>",
  }

  test! {
    name: converts_multiple_font_tags,
    stage: RewriteFontTags,
    content: "<html><body><font>One</font><p>Middle</p><font>Two</font></body></html>",
    expected: "<html><head></head><body><span>One</span><p>Middle</p><span>Two</span></body></html>",
  }

  test! {
    name: handles_nested_font_tags,
    stage: RewriteFontTags,
    content: "<html><body><font>Outer <font>Inner</font></font></body></html>",
    expected: "<html><head></head><body><span>Outer <span>Inner</span></span></body></html>",
  }
}
