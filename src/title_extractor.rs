use super::*;

/// Titles shorter than this are considered suspect and trigger an H1 lookup.
const MIN_TITLE_LENGTH: usize = 15;

/// Titles longer than this are considered suspect and trigger an H1 lookup.
const MAX_TITLE_LENGTH: usize = 150;

/// After splitting on the last colon, the suffix must have at least this many
/// words to be used as the title. If it falls short, the first colon is tried
/// instead.
const MIN_COLON_SUFFIX_WORDS: usize = 3;

/// If the text before the first colon has more than this many words, the title
/// structure is considered unusual and the raw title is kept as-is.
const MAX_COLON_PREFIX_WORDS: usize = 5;

/// After splitting on the last separator, the candidate must have at least this
/// many words before the prefix-strip fallback is attempted.
const MIN_SEPARATOR_CANDIDATE_WORDS: usize = 3;

/// A separator candidate with this many words or fewer is considered too short
/// and falls back to the raw title, unless hierarchical separators were present
/// and exactly one word was removed.
const MAX_SHORT_TITLE_WORDS: usize = 4;

pub(crate) struct TitleExtractor<'a> {
  document: &'a dom_query::Document,
}

impl<'a> TitleExtractor<'a> {
  fn colon_candidate(&self, raw: &str) -> Option<String> {
    if !raw.contains(": ") {
      return None;
    }

    let heading_matches = self
      .document
      .select("h1, h2")
      .nodes()
      .iter()
      .any(|h| h.text().trim() == raw.trim());

    if heading_matches {
      return None;
    }

    let word_count = |s: &str| s.split_whitespace().count();

    let last_colon = raw.rfind(':').unwrap();
    let after_last = raw[last_colon + 1..].trim().to_string();

    if word_count(&after_last) >= MIN_COLON_SUFFIX_WORDS {
      return Some(after_last);
    }

    let first_colon = raw.find(':').unwrap();
    let before_first = &raw[..first_colon];

    if word_count(before_first) > MAX_COLON_PREFIX_WORDS {
      return None;
    }

    Some(raw[first_colon + 1..].trim().to_string())
  }

  pub(crate) fn extract(&self, raw: &str) -> Option<String> {
    if raw.is_empty() {
      return None;
    }

    let title = Self::separator_candidate(raw)
      .or_else(|| self.colon_candidate(raw))
      .or_else(|| self.header_candidate(raw))
      .unwrap_or_else(|| raw.to_string());

    let title = TITLE_NORMALIZE_WHITESPACE
      .replace_all(title.trim(), " ")
      .to_string();

    if title.is_empty() { None } else { Some(title) }
  }

  fn header_candidate(&self, raw: &str) -> Option<String> {
    if raw.len() >= MIN_TITLE_LENGTH && raw.len() <= MAX_TITLE_LENGTH {
      return None;
    }

    let headers = self.document.select("h1");

    if headers.length() != 1 {
      return None;
    }

    Some(headers.first().text().trim().to_string())
  }

  pub(crate) fn new(document: &'a dom_query::Document) -> Self {
    Self { document }
  }

  fn separator_candidate(raw: &str) -> Option<String> {
    if !TITLE_SEPARATOR.is_match(raw) {
      return None;
    }

    let last_sep_start = TITLE_SEPARATOR.find_iter(raw).last().unwrap().start();

    let mut candidate = raw[..last_sep_start].to_string();

    let word_count = |string: &str| string.split_whitespace().count();

    if word_count(&candidate) < MIN_SEPARATOR_CANDIDATE_WORDS {
      candidate = TITLE_LEADING_JUNK.replace(raw, "").trim().to_string();
    }

    candidate = TITLE_NORMALIZE_WHITESPACE
      .replace_all(candidate.trim(), " ")
      .to_string();

    let candidate_words = word_count(&candidate);
    let raw_words_without_seps =
      word_count(&TITLE_SEPARATOR.replace_all(raw, ""));

    let had_hierarchical = TITLE_HIERARCHICAL_SEPARATOR.is_match(raw);
    let too_short = candidate_words <= MAX_SHORT_TITLE_WORDS;

    let not_one_word_shorter =
      candidate_words != raw_words_without_seps.saturating_sub(1);

    if too_short && (!had_hierarchical || not_one_word_shorter) {
      return None;
    }

    Some(candidate)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn run(raw: &str, body: &str) -> Option<String> {
    TitleExtractor::new(&dom_query::Document::from(
      format!("<html><head></head><body>{body}</body></html>").as_str(),
    ))
    .extract(raw)
  }

  #[test]
  fn colon_falls_back_to_first_when_last_too_short() {
    assert_eq!(
      run("site: foo bar baz qux: hi", ""),
      Some("foo bar baz qux: hi".into())
    );
  }

  #[test]
  fn colon_skipped_when_heading_matches() {
    assert_eq!(
      run("site: foo bar baz qux", "<h1>site: foo bar baz qux</h1>"),
      Some("site: foo bar baz qux".into())
    );
  }

  #[test]
  fn colon_strips_site_name() {
    assert_eq!(
      run("site: foo bar baz qux", ""),
      Some("foo bar baz qux".into())
    );
  }

  #[test]
  fn colon_uses_last_colon_when_long_enough() {
    assert_eq!(
      run("site: section: foo bar baz", ""),
      Some("foo bar baz".into())
    );
  }

  #[test]
  fn colon_uses_raw_when_prefix_too_long() {
    assert_eq!(
      run("one two three four five six: hi", ""),
      Some("one two three four five six: hi".into())
    );
  }

  #[test]
  fn empty_returns_none() {
    assert_eq!(run("", ""), None);
  }

  #[test]
  fn h1_skipped_when_multiple() {
    assert_eq!(run("hi", "<h1>foo</h1><h1>bar</h1>"), Some("hi".into()));
  }

  #[test]
  fn h1_used_when_title_too_long() {
    assert_eq!(
      run(&"a".repeat(151), "<h1>foo bar</h1>"),
      Some("foo bar".into())
    );
  }

  #[test]
  fn h1_used_when_title_too_short() {
    assert_eq!(run("hi", "<h1>foo bar</h1>"), Some("foo bar".into()));
  }

  #[test]
  fn normalize_collapses_whitespace() {
    assert_eq!(run("foo   bar", ""), Some("foo bar".into()));
  }

  #[test]
  fn plain_title_returned_as_is() {
    assert_eq!(run("foo bar", ""), Some("foo bar".into()));
  }

  #[test]
  fn separator_short_candidate_tries_prefix_strip() {
    assert_eq!(
      run("site name | foo bar baz qux quux", ""),
      Some("foo bar baz qux quux".into())
    );
  }

  #[test]
  fn separator_strips_site_name() {
    assert_eq!(
      run("foo bar baz qux quux | site name", ""),
      Some("foo bar baz qux quux".into())
    );
  }

  #[test]
  fn separator_too_short_uses_raw() {
    assert_eq!(
      run("foo bar | site name", ""),
      Some("foo bar | site name".into())
    );
  }
}
