use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct TextLength {
  content: bool,
  leading_whitespace: bool,
  pub(crate) length: usize,
  trailing_whitespace: bool,
}

impl Add for TextLength {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    if !self.content {
      Self {
        leading_whitespace: self.leading_whitespace || other.leading_whitespace,
        ..other
      }
    } else if !other.content {
      Self {
        trailing_whitespace: self.trailing_whitespace
          || other.leading_whitespace,
        ..self
      }
    } else {
      Self {
        content: true,
        leading_whitespace: self.leading_whitespace,
        length: self.length
          + other.length
          + usize::from(self.trailing_whitespace || other.leading_whitespace),
        trailing_whitespace: other.trailing_whitespace,
      }
    }
  }
}

impl From<&NodeRef<'_>> for TextLength {
  fn from(node: &NodeRef<'_>) -> Self {
    node.query_or(Self::default(), |node| match &node.data {
      NodeData::Text { contents } => Self::from(&**contents),
      _ => Self::default(),
    })
  }
}

impl From<&str> for TextLength {
  fn from(text: &str) -> Self {
    let mut length = Self::default();

    for character in text.chars() {
      if !character.is_whitespace() {
        if length.trailing_whitespace {
          length.length += 1;
          length.trailing_whitespace = false;
        }

        length.length += character.len_utf16();
        length.content = true;
      } else if length.content {
        length.trailing_whitespace = true;
      } else {
        length.leading_whitespace = true;
      }
    }

    length
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add() {
    #[track_caller]
    fn case(parts: &[&str]) {
      assert_eq!(
        parts
          .iter()
          .map(|part| TextLength::from(*part))
          .fold(TextLength::default(), |total, part| total + part),
        TextLength::from(parts.concat().as_str()),
      );
    }

    case(&[]);
    case(&["", ""]);
    case(&[" ", " "]);
    case(&["foo", "bar"]);
    case(&["foo ", "bar"]);
    case(&["foo", " bar"]);
    case(&["foo ", " ", " bar"]);
    case(&[" ", "foo", " "]);
    case(&["\n", "foo  bar", "\u{a0}", "baz\t"]);
  }

  #[test]
  fn from_str() {
    #[track_caller]
    fn case(text: &str, expected: usize) {
      assert_eq!(TextLength::from(text).length, expected);
    }

    case("", 0);
    case("  ", 0);
    case("foo", 3);
    case(" foo  bar ", 7);
    case("foo\nbar", 7);
    case("\u{1F600}", 2);
  }
}
