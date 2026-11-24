use super::*;

/// Enforces the configured maximum number of DOM elements.
///
/// If `ParserOptions::max_elements` is set, this stage counts elements in the parsed
/// document and returns `Error::ElementLimitExceeded` when the count is above
/// the limit, preventing expensive processing of unusually large inputs.
pub(crate) struct ElementLimit;

impl Stage for ElementLimit {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(limit) = context.options().max_elements.filter(|limit| *limit > 0)
    else {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn zero_disables_limit() {
    let mut document = dom_query::Document::from("<p>foo</p>");

    let options = ParserOptions {
      max_elements: Some(0),
      ..ParserOptions::default()
    };

    ElementLimit
      .run(&mut Context::new(&mut document, &options))
      .unwrap();
  }
}
