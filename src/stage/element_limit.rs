use super::*;

pub struct ElementLimit;

impl Stage for ElementLimit {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(limit) = context.options().max_elements else {
      return Ok(());
    };

    let count = context.document().element_count();

    if count > limit {
      return Err(Error::ElementLimitExceeded {
        found: count,
        limit,
      });
    }

    Ok(())
  }
}
