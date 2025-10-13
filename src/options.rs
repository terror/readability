#[derive(Debug, Clone)]
pub struct ReadabilityOptions {
  pub debug: bool,
  pub max_elems_to_parse: Option<usize>,
  pub nb_top_candidates: usize,
  pub char_threshold: usize,
  pub classes_to_preserve: Vec<String>,
  pub keep_classes: bool,
  pub disable_json_ld: bool,
  pub link_density_modifier: f32,
}

impl Default for ReadabilityOptions {
  fn default() -> Self {
    Self {
      debug: false,
      max_elems_to_parse: None,
      nb_top_candidates: 5,
      char_threshold: 500,
      classes_to_preserve: vec!["page".to_string()],
      keep_classes: false,
      disable_json_ld: false,
      link_density_modifier: 0.0,
    }
  }
}

impl ReadabilityOptions {
  pub fn builder() -> ReadabilityOptionsBuilder {
    ReadabilityOptionsBuilder::default()
  }
}

#[derive(Default)]
pub struct ReadabilityOptionsBuilder {
  inner: ReadabilityOptions,
}

impl ReadabilityOptionsBuilder {
  pub fn debug(mut self, debug: bool) -> Self {
    self.inner.debug = debug;
    self
  }

  pub fn max_elems_to_parse(mut self, max: Option<usize>) -> Self {
    self.inner.max_elems_to_parse = max;
    self
  }

  pub fn nb_top_candidates(mut self, value: usize) -> Self {
    self.inner.nb_top_candidates = value;
    self
  }

  pub fn char_threshold(mut self, value: usize) -> Self {
    self.inner.char_threshold = value;
    self
  }

  pub fn classes_to_preserve<I, S>(mut self, classes: I) -> Self
  where
    I: IntoIterator<Item = S>,
    S: Into<String>,
  {
    self.inner.classes_to_preserve =
      classes.into_iter().map(Into::into).collect();
    self
  }

  pub fn keep_classes(mut self, keep: bool) -> Self {
    self.inner.keep_classes = keep;
    self
  }

  pub fn disable_json_ld(mut self, disable: bool) -> Self {
    self.inner.disable_json_ld = disable;
    self
  }

  pub fn link_density_modifier(mut self, modifier: f32) -> Self {
    self.inner.link_density_modifier = modifier;
    self
  }

  pub fn build(self) -> ReadabilityOptions {
    self.inner
  }
}
