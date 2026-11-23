use super::*;

enum Assertion<T> {
  Expect(T),
  Unset,
}

pub(crate) struct Test {
  article: bool,
  document: Option<String>,
  expected_html: Option<String>,
  expected_lang: Assertion<Option<String>>,
  expected_metadata: Option<Metadata>,
  flags: Flags,
  metadata: Option<Metadata>,
  stages: Vec<Box<dyn Stage>>,
}

impl Test {
  pub(crate) fn article(self) -> Self {
    Self {
      article: true,
      ..self
    }
  }

  pub(crate) fn document(self, html: &str) -> Self {
    Self {
      document: Some(html.to_owned()),
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

  pub(crate) fn flags(self, flags: Flags) -> Self {
    Self { flags, ..self }
  }

  pub(crate) fn metadata(self, metadata: Metadata) -> Self {
    Self {
      metadata: Some(metadata),
      ..self
    }
  }

  pub(crate) fn new() -> Self {
    Self {
      article: false,
      document: None,
      expected_html: None,
      expected_lang: Assertion::Unset,
      expected_metadata: None,
      flags: Flags::default(),
      metadata: None,
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

    let (metadata, lang) = {
      let mut context = Context::new(&mut document, &options);

      context.flags = self.flags;

      if self.article {
        context.article = context
          .document
          .select("body")
          .nodes()
          .first()
          .map(|node| node.id);
      }

      if let Some(metadata) = self.metadata {
        context.metadata = metadata;
      }

      for stage in &mut self.stages {
        stage.run(&mut context).unwrap();
      }

      (context.metadata, context.lang)
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
