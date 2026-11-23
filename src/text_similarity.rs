use super::*;

#[allow(clippy::cast_precision_loss)]
pub(crate) fn text_similarity(a: &str, b: &str) -> f64 {
  let a = a.to_lowercase();
  let b = b.to_lowercase();

  let tokens_a = TOKENIZE
    .split(&a)
    .filter(|token| !token.is_empty())
    .collect::<Vec<_>>();

  let tokens_b = TOKENIZE
    .split(&b)
    .filter(|token| !token.is_empty())
    .collect::<Vec<_>>();

  if tokens_a.is_empty() || tokens_b.is_empty() {
    return 0.0;
  }

  let unique_b = tokens_b
    .iter()
    .filter(|token| !tokens_a.contains(token))
    .copied()
    .collect::<Vec<_>>()
    .join(" ");

  1.0 - unique_b.len() as f64 / tokens_b.join(" ").len() as f64
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn similarity() {
    #[track_caller]
    fn case(a: &str, b: &str, expected: f64) {
      assert_eq!(text_similarity(a, b), expected);
    }

    case("foo bar", "Foo, bar", 1.0);
    case("foo", "bar", 0.0);
    case("foo", "foo bar", 1.0 - 3.0 / 7.0);
    case("", "foo", 0.0);
    case("foo", "", 0.0);
  }
}
