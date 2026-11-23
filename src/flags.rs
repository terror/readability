#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Flags {
  pub(crate) clean_conditionally: bool,
  pub(crate) strip_unlikelys: bool,
  pub(crate) weight_classes: bool,
}

impl Default for Flags {
  fn default() -> Self {
    Self {
      clean_conditionally: true,
      strip_unlikelys: true,
      weight_classes: true,
    }
  }
}

impl Flags {
  pub(crate) fn disable_next(&mut self) -> bool {
    if self.strip_unlikelys {
      self.strip_unlikelys = false;
    } else if self.weight_classes {
      self.weight_classes = false;
    } else if self.clean_conditionally {
      self.clean_conditionally = false;
    } else {
      return false;
    }

    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn disable_next() {
    let mut flags = Flags::default();

    assert!(flags.disable_next());
    assert!(!flags.strip_unlikelys);
    assert!(flags.weight_classes);

    assert!(flags.disable_next());
    assert!(!flags.weight_classes);
    assert!(flags.clean_conditionally);

    assert!(flags.disable_next());

    assert_eq!(
      flags,
      Flags {
        clean_conditionally: false,
        strip_unlikelys: false,
        weight_classes: false,
      }
    );

    assert!(!flags.disable_next());
  }
}
