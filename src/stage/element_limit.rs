use super::*;

pub struct ElementLimitStage;

impl Stage for ElementLimitStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    if let Some(limit) = context.options().max_elems_to_parse {
      let count = context.document().count_elements();

      if count > limit {
        return Err(Error::ElementLimitExceeded {
          found: count,
          limit,
        });
      }
    }

    Ok(())
  }
}
