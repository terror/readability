use super::*;

struct Attempt {
  dir: Option<String>,
  document: dom_query::Document,
  extraction: Extraction,
  length: usize,
}

pub(crate) struct GrabArticle {
  stages: Vec<Box<dyn Stage>>,
}

impl GrabArticle {
  pub(crate) fn new(stages: Vec<Box<dyn Stage>>) -> Self {
    Self { stages }
  }
}

impl Stage for GrabArticle {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let cache = context.document.clone();

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

      let length = context.node(article).inner_text_length();

      if length >= context.options.min_text_length {
        return Ok(());
      }

      let document = mem::replace(&mut *context.document, cache.clone());

      attempts.push(Attempt {
        dir: context.dir.take(),
        document,
        extraction: mem::take(&mut context.extraction),
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
    context.extraction = best.extraction;
    context.dir = best.dir;

    Ok(())
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

    let options = ParserOptions::default();

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
  fn fails_without_content() {
    let flags = Rc::new(RefCell::new(Vec::new()));

    let (result, _) = grab("<html><body></body></html>", &flags);

    assert!(matches!(result, Err(Error::MissingArticleContent)));
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
  fn restores_best_attempt_state() {
    struct Attempts(usize);

    impl Stage for Attempts {
      fn run(&mut self, context: &mut Context<'_>) -> Result {
        let article = create_element(&context.document.tree, "div");

        article.append_child(
          &context
            .document
            .tree
            .new_text(["foo", "foo bar", "bar", "baz"][self.0]),
        );

        self.0 += 1;

        context.dir = Some(self.0.to_string());
        context.extraction.article = Some(article.id);
        context.extraction.top_candidate = Some(article.id);

        Ok(())
      }
    }

    let mut document = dom_query::Document::from("<p>foo</p>");

    let options = ParserOptions::default();

    let mut context = Context::new(&mut document, &options);

    GrabArticle::new(vec![Box::new(Attempts(0))])
      .run(&mut context)
      .unwrap();

    assert_eq!(context.dir.as_deref(), Some("2"));

    assert_eq!(context.extraction.top_candidate, context.extraction.article);

    assert_eq!(
      context
        .node(context.extraction.article.unwrap())
        .html()
        .to_string(),
      "<div>foo bar</div>",
    );
  }
}
