#[derive(Debug, Clone)]
pub struct ReadabilityOptions {
  pub char_threshold: usize,
  pub classes_to_preserve: Vec<String>,
  pub debug: bool,
  pub disable_json_ld: bool,
  pub keep_classes: bool,
  pub link_density_modifier: f32,
  pub max_elems_to_parse: Option<usize>,
  pub nb_top_candidates: usize,
}

impl Default for ReadabilityOptions {
  fn default() -> Self {
    Self {
      char_threshold: 500,
      classes_to_preserve: vec!["page".to_string()],
      debug: false,
      disable_json_ld: false,
      keep_classes: false,
      link_density_modifier: 0.0,
      max_elems_to_parse: None,
      nb_top_candidates: 5,
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
  pub fn char_threshold(self, char_threshold: usize) -> Self {
    Self {
      inner: ReadabilityOptions {
        char_threshold,
        ..self.inner
      },
    }
  }

  #[must_use]
  pub fn classes_to_preserve<I, S>(self, classes: I) -> Self
  where
    I: IntoIterator<Item = S>,
    S: Into<String>,
  {
    Self {
      inner: ReadabilityOptions {
        classes_to_preserve: classes.into_iter().map(Into::into).collect(),
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
  pub fn disable_json_ld(self, disable_json_ld: bool) -> Self {
    Self {
      inner: ReadabilityOptions {
        disable_json_ld,
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
  pub fn link_density_modifier(self, link_density_modifier: f32) -> Self {
    Self {
      inner: ReadabilityOptions {
        link_density_modifier,
        ..self.inner
      },
    }
  }

  #[must_use]
  pub fn max_elems_to_parse(self, max_elems_to_parse: Option<usize>) -> Self {
    Self {
      inner: ReadabilityOptions {
        max_elems_to_parse,
        ..self.inner
      },
    }
  }

  #[must_use]
  pub fn nb_top_candidates(self, nb_top_candidates: usize) -> Self {
    Self {
      inner: ReadabilityOptions {
        nb_top_candidates,
        ..self.inner
      },
    }
  }
}
