use super::*;

mod element_limit;

pub use element_limit::ElementLimitStage;

pub(crate) trait Stage {
  fn run(&mut self, context: &mut Context<'_>) -> Result;
}
