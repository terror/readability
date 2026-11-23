use super::*;

pub(crate) struct PrepareNodes {
  rules: Vec<Box<dyn Rule>>,
}

impl Stage for PrepareNodes {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let mut rules = self
      .rules
      .iter_mut()
      .map(|rule| &mut **rule as &mut dyn Rule)
      .collect::<Vec<_>>();

    traverse(context, &mut rules);

    Ok(())
  }
}

impl PrepareNodes {
  pub(crate) fn new(rules: Vec<Box<dyn Rule>>) -> Self {
    Self { rules }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn applies_rules_in_document_order() {
    Test::new()
      .stage(PrepareNodes::new(vec![
        Box::new(RemoveEmptyContainers),
        Box::new(ConvertDivsToParagraphs),
      ]))
      .document("<html><body><div><div></div><img></div></body></html>")
      .expected_html(
        "<html><head></head><body><div><p><img></p></div></body></html>",
      )
      .run();
  }
}
