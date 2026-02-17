use super::*;

/// Enforces the configured maximum number of DOM elements.
///
/// If `Options::max_elements` is set, this stage counts elements in the parsed
/// document and returns `Error::ElementLimitExceeded` when the count is above
/// the limit, preventing expensive processing of unusually large inputs.
pub(crate) struct ElementLimit;

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
