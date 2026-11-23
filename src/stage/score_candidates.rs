use super::*;

const MAX_ANCESTOR_DEPTH: usize = 5;

const MIN_PARAGRAPH_LENGTH: usize = 25;

const TAGS_TO_SCORE: &str = "section, h2, h3, h4, h5, h6, p, td, pre";

pub(crate) struct ScoreCandidates;

impl Stage for ScoreCandidates {
  #[allow(clippy::cast_precision_loss)]
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let weight_classes = context.flags.weight_classes;

    let elements = context.document.select(TAGS_TO_SCORE).nodes().to_vec();

    for element in elements {
      if !element.parent().is_some_and(|parent| parent.is_element()) {
        continue;
      }

      let text = element.inner_text();

      let length = text_length(&text);

      if length < MIN_PARAGRAPH_LENGTH {
        continue;
      }

      let score = 1.0
        + (COMMAS.find_iter(&text).count() + 1) as f64
        + (length / 100).min(3) as f64;

      for (level, ancestor) in element
        .ancestors(Some(MAX_ANCESTOR_DEPTH))
        .iter()
        .enumerate()
      {
        if !ancestor.is_element()
          || !ancestor.parent().is_some_and(|parent| parent.is_element())
        {
          continue;
        }

        let divider = match level {
          0 => 1.0,
          1 => 2.0,
          _ => level as f64 * 3.0,
        };

        let entry = context
          .extraction
          .scores
          .entry(ancestor.id)
          .or_insert_with(|| {
            context.extraction.candidates.push(ancestor.id);
            ancestor.initial_score(weight_classes)
          });

        *entry += score / divider;
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn scores_ancestors() {
    let mut document = dom_query::Document::from(format!(
      "<html><body><div><section><p>{}, foo</p></section></div></body></html>",
      "a".repeat(100),
    ));

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    ScoreCandidates.run(&mut context).unwrap();

    let scores = context
      .extraction
      .candidates
      .iter()
      .map(|id| (context.node(*id).tag(), context.extraction.scores[id]))
      .collect::<Vec<_>>();

    assert_eq!(
      scores,
      vec![
        ("DIV".into(), 11.0),
        ("BODY".into(), 2.0 + 4.0 / 6.0),
        ("SECTION".into(), 4.0),
      ]
    );
  }

  #[test]
  fn skips_short_paragraphs() {
    let mut document = dom_query::Document::from(
      "<html><body><div><p>foo</p></div></body></html>",
    );

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    ScoreCandidates.run(&mut context).unwrap();

    assert_eq!(context.extraction.candidates, Vec::new());
  }
}
