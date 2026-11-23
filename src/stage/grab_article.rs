use super::*;

struct Attempt {
  article: NodeId,
  document: dom_query::Document,
  length: usize,
}

pub(crate) struct GrabArticle {
  stages: Vec<Box<dyn Stage>>,
}

impl Stage for GrabArticle {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let cache = context.document.html().to_string();

    let mut attempts = Vec::new();

    loop {
      context.extraction = Extraction::default();

      for stage in &mut self.stages {
        stage.run(context)?;
      }

      let article = context
        .extraction
        .article
        .ok_or(Error::MissingArticleContent)?;

      let length = text_length(&context.node(article).inner_text());

      if length >= context.options.min_text_length {
        return Ok(());
      }

      let document = mem::replace(
        &mut *context.document,
        dom_query::Document::from(cache.as_str()),
      );

      attempts.push(Attempt {
        article,
        document,
        length,
      });

      if !context.flags.disable_next() {
        break;
      }
    }

    let best = attempts
      .into_iter()
      .reduce(|best, attempt| {
        if attempt.length > best.length {
          attempt
        } else {
          best
        }
      })
      .ok_or(Error::MissingArticleContent)?;

    if best.length == 0 {
      return Err(Error::MissingArticleContent);
    }

    *context.document = best.document;
    context.extraction.article = Some(best.article);

    Ok(())
  }
}

impl GrabArticle {
  pub(crate) fn new(stages: Vec<Box<dyn Stage>>) -> Self {
    Self { stages }
  }
}

#[cfg(test)]
mod tests {
  use {
    super::*,
    std::{cell::RefCell, rc::Rc},
  };

  struct RecordFlags(Rc<RefCell<Vec<Flags>>>);

  impl Stage for RecordFlags {
    fn run(&mut self, context: &mut Context<'_>) -> Result {
      self.0.borrow_mut().push(context.flags);
      Ok(())
    }
  }

  fn grab(html: &str, flags: &Rc<RefCell<Vec<Flags>>>) -> (Result, String) {
    let mut document = dom_query::Document::from(html);

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    let result = GrabArticle::new(vec![
      Box::new(RecordFlags(flags.clone())),
      Box::new(ScoreCandidates),
      Box::new(SelectTopCandidate),
      Box::new(ExtractArticle),
    ])
    .run(&mut context);

    let article = context
      .extraction
      .article
      .map_or_default(|id| context.node(id).html().to_string());

    (result, article)
  }

  #[test]
  fn single_attempt_when_long_enough() {
    let flags = Rc::new(RefCell::new(Vec::new()));

    let paragraph = format!("<p>{}</p>", "foo ".repeat(200));

    let (result, article) = grab(
      &format!("<html><body><div>{paragraph}</div></body></html>"),
      &flags,
    );

    result.unwrap();

    assert_eq!(article, format!("<div><div>{paragraph}</div></div>"));

    assert_eq!(*flags.borrow(), vec![Flags::default()]);
  }

  #[test]
  fn retries_with_flags_disabled() {
    let flags = Rc::new(RefCell::new(Vec::new()));

    let (result, article) =
      grab("<html><body><p>foo</p></body></html>", &flags);

    result.unwrap();

    assert_eq!(article, "<div><div><p>foo</p></div></div>");

    assert_eq!(flags.borrow().len(), 4);

    assert_eq!(
      flags.borrow()[3],
      Flags {
        clean_conditionally: false,
        strip_unlikelys: false,
        weight_classes: false,
      }
    );
  }

  #[test]
  fn fails_without_content() {
    let flags = Rc::new(RefCell::new(Vec::new()));

    let (result, _) = grab("<html><body></body></html>", &flags);

    assert!(matches!(result, Err(Error::MissingArticleContent)));
  }
}
