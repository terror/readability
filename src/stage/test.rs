use super::*;

enum Assertion<T> {
  Expect(T),
  Unset,
}

pub(crate) struct Test {
  document: Option<String>,
  expected_dir: Assertion<Option<String>>,
  expected_html: Option<String>,
  expected_lang: Assertion<Option<String>>,
  expected_metadata: Option<Metadata>,
  stages: Vec<Box<dyn Stage>>,
}

impl Test {
  pub(crate) fn document(self, html: &str) -> Self {
    Self {
      document: Some(html.to_owned()),
      ..self
    }
  }

  pub(crate) fn expected_dir(self, dir: Option<&str>) -> Self {
    Self {
      expected_dir: Assertion::Expect(dir.map(str::to_owned)),
      ..self
    }
  }

  pub(crate) fn expected_html(self, html: &str) -> Self {
    Self {
      expected_html: Some(html.to_owned()),
      ..self
    }
  }

  pub(crate) fn expected_lang(self, lang: Option<&str>) -> Self {
    Self {
      expected_lang: Assertion::Expect(lang.map(str::to_owned)),
      ..self
    }
  }

  pub(crate) fn expected_metadata(self, metadata: Metadata) -> Self {
    Self {
      expected_metadata: Some(metadata),
      ..self
    }
  }

  pub(crate) fn new() -> Self {
    Self {
      document: None,
      expected_dir: Assertion::Unset,
      expected_html: None,
      expected_lang: Assertion::Unset,
      expected_metadata: None,
      stages: Vec::new(),
    }
  }

  #[track_caller]
  pub(crate) fn run(mut self) {
    let html = self
      .document
      .as_deref()
      .unwrap_or("<html><body></body></html>");

    let mut document = dom_query::Document::from(html);

    let options = ReadabilityOptions::default();

    let (metadata, lang, dir) = {
      let mut context = Context::new(&mut document, &options);

      for stage in &mut self.stages {
        stage.run(&mut context).unwrap();
      }

      (context.metadata, context.lang, context.dir)
    };

    if let Some(expected) = self.expected_html {
      assert_eq!(document.html().to_string(), expected);
    }

    if let Some(expected) = self.expected_metadata {
      assert_eq!(metadata, expected);
    }

    if let Assertion::Expect(expected) = self.expected_lang {
      assert_eq!(lang, expected);
    }

    if let Assertion::Expect(expected) = self.expected_dir {
      assert_eq!(dir, expected);
    }
  }

  pub(crate) fn stage(self, stage: impl Stage + 'static) -> Self {
    Self {
      stages: self
        .stages
        .into_iter()
        .chain([Box::new(stage) as _])
        .collect(),
      ..self
    }
  }
}
