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
