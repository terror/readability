use super::*;

pub struct ElementLimitStage;

impl Stage for ElementLimitStage {
  fn run(&mut self, ctx: &mut Context<'_>) -> Result<()> {
    if let Some(limit) = ctx.options().max_elems_to_parse {
      let count = ctx.document().count_elements();

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
