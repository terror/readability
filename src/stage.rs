use super::*;

mod element_limit;
mod remove_disallowed_nodes;
mod rewrite_font_tags;

pub use {
  element_limit::ElementLimit, remove_disallowed_nodes::RemoveDisallowedNodes,
  rewrite_font_tags::RewriteFontTags,
};

pub(crate) trait Stage {
  fn run(&mut self, context: &mut Context<'_>) -> Result;
}

#[cfg(test)]
macro_rules! test {
  {
    name: $name:ident,
    stage: $stage:expr,
    content: $content:expr,
    expected: $expected:expr $(,)?
  } => {
    #[test]
    fn $name() {
      let mut document = dom_query::Document::from($content);

      let options = ReadabilityOptions::default();

      let mut context = Context::new(&mut document, &options);

      $stage.run(&mut context).unwrap();

      assert_eq!(document.html().to_string(), $expected);
    }
  };
}

#[cfg(test)]
pub(crate) use test;
