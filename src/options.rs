#[derive(Debug, Clone)]
pub struct ReadabilityOptions {
  /// Whether to enable logging of debug messages.
  ///
  /// Default: false
  pub debug: bool,
  /// Whether to preserve all classes on HTML elements.
  ///
  /// Default: false
  pub keep_classes: bool,
  /// A weighting applied to the link density calculation.
  ///
  /// Default: 0.0
  pub link_density_bias: f32,
  /// The maximum number of DOM elements to parse before aborting.
  ///
  /// Default: None
  pub max_elements: Option<usize>,
  /// The minimum number of characters required for an article to be considered valid.
  ///
  /// Default: 500
  pub min_text_length: usize,
  /// The number of top candidate nodes to analyze during the scoring pass.
  ///
  /// Default: 5
  pub n_top_candidates: usize,
  /// A list of specific class names to preserve when `keep_classes` is false.
  ///
  /// Default: \["page"\]
  pub preserved_classes: Vec<String>,
  /// Whether to extract metadata from JSON-LD.
  ///
  /// Default: true
  pub use_json_ld: bool,
}

impl Default for ReadabilityOptions {
  fn default() -> Self {
    Self {
      debug: false,
      keep_classes: false,
      link_density_bias: 0.0,
      max_elements: None,
      min_text_length: 500,
      n_top_candidates: 5,
      preserved_classes: vec!["page".to_string()],
      use_json_ld: true,
    }
  }
}

impl ReadabilityOptions {
  #[must_use]
  pub fn builder() -> ReadabilityOptionsBuilder {
    ReadabilityOptionsBuilder::default()
  }
}

#[derive(Default)]
pub struct ReadabilityOptionsBuilder {
  inner: ReadabilityOptions,
}

impl ReadabilityOptionsBuilder {
  #[must_use]
  pub fn build(self) -> ReadabilityOptions {
    self.inner
  }

  #[must_use]
  pub fn classes_to_preserve<I, S>(self, classes: I) -> Self
  where
    I: IntoIterator<Item = S>,
    S: Into<String>,
  {
    Self {
      inner: ReadabilityOptions {
        preserved_classes: classes.into_iter().map(Into::into).collect(),
        ..self.inner
      },
    }
  }

  #[must_use]
  pub fn debug(self, debug: bool) -> Self {
    Self {
      inner: ReadabilityOptions {
        debug,
        ..self.inner
      },
    }
  }

  #[must_use]
  pub fn keep_classes(self, keep_classes: bool) -> Self {
    Self {
      inner: ReadabilityOptions {
        keep_classes,
        ..self.inner
      },
    }
  }

  #[must_use]
  pub fn link_density_bias(self, link_density_bias: f32) -> Self {
    Self {
      inner: ReadabilityOptions {
        link_density_bias,
        ..self.inner
      },
    }
  }

  #[must_use]
  pub fn max_elements(self, max_elements: Option<usize>) -> Self {
    Self {
      inner: ReadabilityOptions {
        max_elements,
        ..self.inner
      },
    }
  }

  #[must_use]
  pub fn min_text_length(self, min_text_length: usize) -> Self {
    Self {
      inner: ReadabilityOptions {
        min_text_length,
        ..self.inner
      },
    }
  }

  #[must_use]
  pub fn n_top_candidates(self, n_top_candidates: usize) -> Self {
    Self {
      inner: ReadabilityOptions {
        n_top_candidates,
        ..self.inner
      },
    }
  }

  #[must_use]
  pub fn use_json_ld(self, use_json_ld: bool) -> Self {
    Self {
      inner: ReadabilityOptions {
        use_json_ld,
        ..self.inner
      },
    }
  }
}
