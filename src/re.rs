use super::*;

macro_rules! re {
  ($pat:expr) => {
    LazyLock::new(|| Regex::new(concat!("^", $pat, "$")).unwrap())
  };
}

pub(crate) static BASE64_DATA_URL: LazyLock<Regex> =
  re!(r"data:\s*([^\s;,]+)\s*;\s*base64\s*,(?s)(?P<data>.+)");

pub(crate) static BYLINE_HINTS: LazyLock<Regex> =
  re!(r"(?i).*(?:byline|author|dateline|writtenby|p-author).*");

pub(crate) static COMMENT_SECTION_HINT: LazyLock<Regex> = re!(
  r"(?i).*(?:comment|comments|discussion|discuss|respond|reply|talkback).*"
);

pub(crate) static COMMA_VARIANTS: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"[,،﹐︐﹑⹀⸲，]").unwrap());

pub(crate) static FRAGMENT_URL: LazyLock<Regex> = re!(r"#.+");

pub(crate) static IMAGE_EXTENSION_SUFFIX: LazyLock<Regex> =
  re!(r"(?i).*\.(?:jpg|jpeg|png|webp).*");

pub(crate) static IMG_MISSING_SELF_CLOSING: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"<img([^>]*[^/])>").unwrap());

pub(crate) static LAZY_IMAGE_SRC_VALUE: LazyLock<Regex> =
  re!(r"(?i)\s*\S+\.(?:jpg|jpeg|png|webp)\S*\s*");

pub(crate) static NAMED_HTML_ENTITIES: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"&(?P<name>quot|amp|apos|lt|gt);").unwrap());

pub(crate) static NEGATIVE_CONTENT_HINTS: LazyLock<Regex> = re!(concat!(
  r"(?i).*(?:-ad-|hidden|banner|combx|comment|com-|contact|footer|gdpr|",
  r"masthead|media|meta|outbrain|promo|related|scroll|share|shoutbox|",
  r"sidebar|skyscraper|sponsor|shopping|tags|widget|(?:^| )hid(?:$| )).*"
));

pub(crate) static NUMERIC_HTML_ENTITY: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"&#(?:x([0-9a-fA-F]+)|([0-9]+));").unwrap());

pub(crate) static POSSIBLE_CONTENT_CANDIDATE: LazyLock<Regex> =
  re!(r"(?i).*(?:and|article|body|column|content|main|mathjax|shadow).*");

pub(crate) static POSITIVE_CONTENT_HINTS: LazyLock<Regex> = re!(concat!(
  r"(?i).*(?:article|body|content|entry|hentry|h-entry|main|page|pagination|",
  r"post|text|blog|story).*"
));

pub(crate) static SRCSET_CANDIDATE_VALUE: LazyLock<Regex> =
  re!(r"(?i).*\.(?:jpg|jpeg|png|webp)\s+\d.*");

pub(crate) static TITLE_HIERARCHICAL_SEPARATOR: LazyLock<Regex> =
  re!(r".*\s[\\\/>»]\s.*");

pub(crate) static TITLE_LEADING_SEPARATOR: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"^[^\|\-–—\\\/>»]*[\|\-–—\\\/>»]").unwrap());

pub(crate) static TITLE_SEPARATOR_RUN: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\s[\|\-–—\\\/>»]\s").unwrap());

pub(crate) static TOKEN_BOUNDARY: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\W+").unwrap());

pub(crate) static UNLIKELY_CONTENT_CANDIDATES: LazyLock<Regex> = re!(concat!(
  r"(?i).*(?:-ad-|ai2html|banner|breadcrumbs|combx|comment|community|cover-wrap|",
  r"disqus|extra|footer|gdpr|header|legends|menu|related|remark|replies|rss|",
  r"shoutbox|sidebar|skyscraper|social|sponsor|supplemental|ad-break|agegate|",
  r"pagination|pager|popup|yom-remote).*"
));

pub(crate) static WHITESPACE_RUNS: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"\s{2,}").unwrap());

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn base64_data_url_captures_data_segment() {
    let input = "data:image/png;base64,QUJD";

    let captures = BASE64_DATA_URL
      .captures(input)
      .expect("data url should match");

    assert_eq!(&captures[1], "image/png");

    let data = captures.name("data").expect("data segment capture");

    assert_eq!(data.as_str(), "QUJD");
    assert_eq!(data.start(), input.find(',').map(|idx| idx + 1).unwrap());
  }

  #[test]
  fn byline_hints_detects_substrings() {
    assert!(BYLINE_HINTS.is_match("article byline container"));
    assert!(BYLINE_HINTS.is_match("AUTHOR badge"));
    assert!(!BYLINE_HINTS.is_match("contributor details"));
  }

  #[test]
  fn fragment_url_requires_fragment_reference() {
    assert!(FRAGMENT_URL.is_match("#section-1"));
    assert!(!FRAGMENT_URL.is_match("/path#section-1"));
  }

  #[test]
  fn image_extension_suffix_matches_case_insensitively() {
    assert!(IMAGE_EXTENSION_SUFFIX.is_match("photo.JPEG?width=400"));
    assert!(!IMAGE_EXTENSION_SUFFIX.is_match("document.pdf"));
  }

  #[test]
  fn negative_content_hints_handle_hid_variants() {
    assert!(NEGATIVE_CONTENT_HINTS.is_match("hid"));
    assert!(NEGATIVE_CONTENT_HINTS.is_match("header hid footer"));
    assert!(!NEGATIVE_CONTENT_HINTS.is_match("content primary"));
  }

  #[test]
  fn token_boundary_splits_on_non_word_characters() {
    assert_eq!(
      TOKEN_BOUNDARY
        .split("Hello, world!")
        .filter(|token| !token.is_empty())
        .collect::<Vec<&str>>(),
      vec!["Hello", "world"]
    );
  }

  #[test]
  fn title_separator_run_counts_instances() {
    assert_eq!(TITLE_SEPARATOR_RUN.find_iter("Foo - Bar | Baz").count(), 2);
  }

  #[test]
  fn whitespace_runs_collapse_to_single_space() {
    assert_eq!(
      WHITESPACE_RUNS
        .replace_all("foo  \t bar   baz", " ")
        .as_ref(),
      "foo bar baz"
    );
  }
}
