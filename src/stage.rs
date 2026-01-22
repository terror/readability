use super::*;

mod element_limit;
mod remove_disallowed_nodes;

pub use {
  element_limit::ElementLimitStage,
  remove_disallowed_nodes::RemoveDisallowedNodesStage,
};

pub(crate) trait Stage {
  fn run(&mut self, context: &mut Context<'_>) -> Result;
}
