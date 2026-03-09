use super::*;

/// Titles shorter than this trigger an H1 lookup.
const MIN_TITLE_LENGTH: usize = 15;

/// Titles longer than this trigger an H1 lookup.
const MAX_TITLE_LENGTH: usize = 150;

/// After splitting on the last colon, the suffix must have at least this many
/// words to be used as the title. If it falls short, the first colon is tried.
const MIN_COLON_SUFFIX_WORDS: usize = 3;

/// If the text before the first colon has more than this many words, the title
/// structure is considered unusual and the raw title is kept as-is.
const MAX_COLON_PREFIX_WORDS: usize = 5;

/// After splitting on the last separator, the candidate must have at least this
/// many words before the prefix-strip fallback is attempted.
const MIN_SEPARATOR_CANDIDATE_WORDS: usize = 3;

/// A separator candidate with this many words or fewer is discarded in favour
/// of the raw title, unless a hierarchical separator was present and exactly
/// one word was removed.
const MAX_SHORT_TITLE_WORDS: usize = 4;

pub(crate) struct ExtractTitle;

impl Stage for ExtractTitle {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    if context.metadata.title.is_some() {
      return Ok(());
    }

    context.metadata.title = Self::extract(context.document);

    Ok(())
  }
}

impl ExtractTitle {
  fn colon_candidate(
    raw: &str,
    document: &dom_query::Document,
  ) -> Option<String> {
    if !raw.contains(": ") {
      return None;
    }

    let heading_matches = document
      .select("h1, h2")
      .nodes()
      .iter()
      .any(|heading| heading.text().trim() == raw.trim());

    if heading_matches {
      return None;
    }

    let last_colon = raw.rfind(':').unwrap();
    let after_last = raw[last_colon + 1..].trim().to_string();

    if after_last.split_whitespace().count() >= MIN_COLON_SUFFIX_WORDS {
      return Some(after_last);
    }

    let first_colon = raw.find(':').unwrap();
    let before_first = &raw[..first_colon];

    if before_first.split_whitespace().count() > MAX_COLON_PREFIX_WORDS {
      return None;
    }

    Some(raw[first_colon + 1..].trim().to_string())
  }

  fn extract(document: &dom_query::Document) -> Option<String> {
    let raw_title = document.select("title").first().text();

    let raw_title_trimmed = raw_title.trim();

    if raw_title_trimmed.is_empty() {
      return None;
    }

    let title = Self::separator_candidate(raw_title_trimmed)
      .or_else(|| Self::colon_candidate(raw_title_trimmed, document))
      .or_else(|| Self::header_candidate(raw_title_trimmed, document))
      .unwrap_or_else(|| raw_title_trimmed.to_string());

    let title = TITLE_NORMALIZE_WHITESPACE
      .replace_all(title.trim(), " ")
      .to_string();

    if title.is_empty() { None } else { Some(title) }
  }

  fn header_candidate(
    raw: &str,
    document: &dom_query::Document,
  ) -> Option<String> {
    if raw.len() >= MIN_TITLE_LENGTH && raw.len() <= MAX_TITLE_LENGTH {
      return None;
    }

    let headers = document.select("h1");

    if headers.length() != 1 {
      return None;
    }

    Some(headers.first().text().trim().to_string())
  }

  fn separator_candidate(raw: &str) -> Option<String> {
    if !TITLE_SEPARATOR.is_match(raw) {
      return None;
    }

    let last_sep_start = TITLE_SEPARATOR.find_iter(raw).last().unwrap().start();

    let mut candidate = raw[..last_sep_start].to_string();

    if candidate.split_whitespace().count() < MIN_SEPARATOR_CANDIDATE_WORDS {
      candidate = TITLE_LEADING_JUNK.replace(raw, "").trim().to_string();
    }

    candidate = TITLE_NORMALIZE_WHITESPACE
      .replace_all(candidate.trim(), " ")
      .to_string();

    let candidate_words = candidate.split_whitespace().count();

    let raw_words_without_seps = TITLE_SEPARATOR
      .replace_all(raw, "")
      .split_whitespace()
      .count();

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

  #[test]
  fn colon_falls_back_to_first_when_last_too_short() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title>site: foo bar baz qux: hi</title></head><body></body></html>")
      .expected_metadata(Metadata {
        title: Some("foo bar baz qux: hi".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn colon_skipped_when_heading_matches() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title>site: foo bar baz qux</title></head><body><h1>site: foo bar baz qux</h1></body></html>")
      .expected_metadata(Metadata {
        title: Some("site: foo bar baz qux".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn colon_strips_site_name() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title>site: foo bar baz qux</title></head><body></body></html>")
      .expected_metadata(Metadata {
        title: Some("foo bar baz qux".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn colon_uses_last_colon_when_long_enough() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title>site: section: foo bar baz</title></head><body></body></html>")
      .expected_metadata(Metadata {
        title: Some("foo bar baz".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn colon_uses_raw_when_prefix_too_long() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title>one two three four five six: hi</title></head><body></body></html>")
      .expected_metadata(Metadata {
        title: Some("one two three four five six: hi".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn empty_returns_none() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title></title></head><body></body></html>")
      .expected_metadata(Metadata::default())
      .run();
  }

  #[test]
  fn h1_skipped_when_multiple() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title>hi</title></head><body><h1>foo</h1><h1>bar</h1></body></html>")
      .expected_metadata(Metadata {
        title: Some("hi".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn h1_used_when_title_too_long() {
    Test::new()
      .stage(ExtractTitle)
      .document(&format!(
        "<html><head><title>{}</title></head><body><h1>foo bar</h1></body></html>",
        "a".repeat(151)
      ))
      .expected_metadata(Metadata {
        title: Some("foo bar".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn h1_used_when_title_too_short() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title>hi</title></head><body><h1>foo bar</h1></body></html>")
      .expected_metadata(Metadata {
        title: Some("foo bar".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn normalize_collapses_whitespace() {
    Test::new()
      .stage(ExtractTitle)
      .document(
        "<html><head><title>foo   bar</title></head><body></body></html>",
      )
      .expected_metadata(Metadata {
        title: Some("foo bar".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn plain_title_returned_as_is() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title>foo bar</title></head><body></body></html>")
      .expected_metadata(Metadata {
        title: Some("foo bar".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn separator_short_candidate_tries_prefix_strip() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title>site name | foo bar baz qux quux</title></head><body></body></html>")
      .expected_metadata(Metadata {
        title: Some("foo bar baz qux quux".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn separator_strips_site_name() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title>foo bar baz qux quux | site name</title></head><body></body></html>")
      .expected_metadata(Metadata {
        title: Some("foo bar baz qux quux".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn separator_too_short_uses_raw() {
    Test::new()
      .stage(ExtractTitle)
      .document("<html><head><title>foo bar | site name</title></head><body></body></html>")
      .expected_metadata(Metadata {
        title: Some("foo bar | site name".into()),
        ..Metadata::default()
      })
      .run();
  }

  #[test]
  fn skips_when_title_already_set() {
    Test::new()
      .stage(ExtractTitle)
      .document(
        "<html><head><title>foo bar baz qux quux | site name</title></head><body></body></html>",
      )
      .metadata(Metadata {
        title: Some("bar".into()),
        ..Metadata::default()
      })
      .expected_metadata(Metadata {
        title: Some("bar".into()),
        ..Metadata::default()
      })
      .run();
  }
}
