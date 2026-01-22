use super::*;

pub struct RemoveDisallowedNodes;

impl Stage for RemoveDisallowedNodes {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    context
      .document()
      .remove_elements("script, style, noscript");

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn removes_script_tags() {
    let mut document = dom_query::Document::from(
      "<html><body><script>alert('hi');</script><p>Content</p></body></html>",
    );

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    RemoveDisallowedNodes.run(&mut context).unwrap();

    assert_eq!(document.select("p").length(), 1);
    assert_eq!(document.select("script").length(), 0);
  }

  #[test]
  fn removes_style_tags() {
    let mut document = dom_query::Document::from(
      "<html><head><style>body { color: red; }</style></head><body><p>Content</p></body></html>",
    );

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    RemoveDisallowedNodes.run(&mut context).unwrap();

    assert_eq!(document.select("p").length(), 1);
    assert_eq!(document.select("style").length(), 0);
  }

  #[test]
  fn removes_noscript_tags() {
    let mut document = dom_query::Document::from(
      "<html><body><noscript>Enable JS</noscript><p>Content</p></body></html>",
    );

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    RemoveDisallowedNodes.run(&mut context).unwrap();

    assert_eq!(document.select("noscript").length(), 0);
    assert_eq!(document.select("p").length(), 1);
  }

  #[test]
  fn removes_all_disallowed_tags() {
    let mut document = dom_query::Document::from(
      "
      <html>
        <head>
          <style>.foo { color: red; }</style>
          <script>var x = 1;</script>
        </head>
        <body>
          <script>alert('inline');</script>
          <noscript>Please enable JavaScript</noscript>
          <p>Actual content</p>
          <style>.bar { display: none; }</style>
        </body>
      </html>
      ",
    );

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    RemoveDisallowedNodes.run(&mut context).unwrap();

    assert_eq!(document.select("noscript").length(), 0);
    assert_eq!(document.select("p").length(), 1);
    assert_eq!(document.select("script").length(), 0);
    assert_eq!(document.select("style").length(), 0);
  }
}
