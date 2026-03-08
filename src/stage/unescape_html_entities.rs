use super::*;

/// Unescapes HTML entities in all metadata string fields.
///
/// Named entities (`&amp;`, `&quot;`, `&apos;`, `&lt;`, `&gt;`) and
/// numeric entities (`&#NNN;`, `&#xHHH;`) are decoded.
///
/// Invalid code points are replaced with U+FFFD.
pub(crate) struct UnescapeHtmlEntities;

impl Stage for UnescapeHtmlEntities {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let metadata = mem::take(&mut context.metadata);

    context.metadata = Metadata {
      byline: metadata.byline.map(|s| Self::unescape(&s)),
      excerpt: metadata.excerpt.map(|s| Self::unescape(&s)),
      published_time: metadata.published_time.map(|s| Self::unescape(&s)),
      site_name: metadata.site_name.map(|s| Self::unescape(&s)),
      title: metadata.title.map(|s| Self::unescape(&s)),
    };

    Ok(())
  }
}

impl UnescapeHtmlEntities {
  fn unescape(s: &str) -> String {
    Self::unescape_numeric(&Self::unescape_named(s))
  }

  fn unescape_named(s: &str) -> String {
    s.replace("&quot;", "\"")
      .replace("&amp;", "&")
      .replace("&apos;", "'")
      .replace("&lt;", "<")
      .replace("&gt;", ">")
  }

  fn unescape_numeric(s: &str) -> String {
    static RE: LazyLock<Regex> = LazyLock::new(|| {
      Regex::new(r"(?i)&#(?:x([0-9a-f]+)|([0-9]+));").unwrap()
    });

    RE.replace_all(s, |captures: &regex::Captures| {
      let num = if let Some(hex) = captures.get(1) {
        u32::from_str_radix(hex.as_str(), 16).unwrap_or(0xfffd)
      } else {
        captures[2].parse::<u32>().unwrap_or(0xfffd)
      };

      let c =
        if num == 0 || num > 0x0010_ffff || (0xd800..=0xdfff).contains(&num) {
          '\u{fffd}'
        } else {
          char::from_u32(num).unwrap_or('\u{fffd}')
        };

      c.to_string()
    })
    .into_owned()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn invalid_codepoints_replaced_with_replacement_char() {
    assert_eq!(UnescapeHtmlEntities::unescape_numeric("&#x0;"), "\u{fffd}");

    assert_eq!(
      UnescapeHtmlEntities::unescape_numeric("&#xFFFFFFFF;"),
      "\u{fffd}"
    );

    assert_eq!(
      UnescapeHtmlEntities::unescape_numeric("&#xD800;"),
      "\u{fffd}"
    );

    assert_eq!(
      UnescapeHtmlEntities::unescape_numeric("&#xDFFF;"),
      "\u{fffd}"
    );
  }

  #[test]
  fn invalid_named_entity_left_intact() {
    assert_eq!(UnescapeHtmlEntities::unescape_named("&#xg;"), "&#xg;");
  }

  #[test]
  fn named_entities() {
    assert_eq!(UnescapeHtmlEntities::unescape_named("&quot;"), "\"");
    assert_eq!(UnescapeHtmlEntities::unescape_named("&amp;"), "&");
    assert_eq!(UnescapeHtmlEntities::unescape_named("&apos;"), "'");
    assert_eq!(UnescapeHtmlEntities::unescape_named("&lt;"), "<");
    assert_eq!(UnescapeHtmlEntities::unescape_named("&gt;"), ">");
  }

  #[test]
  fn numeric_decimal() {
    assert_eq!(UnescapeHtmlEntities::unescape_numeric("&#128557;"), "😭");
  }

  #[test]
  fn numeric_hex() {
    assert_eq!(UnescapeHtmlEntities::unescape_numeric("&#x1F62D;"), "😭");
  }

  #[test]
  fn numeric_hex_case_insensitive() {
    assert_eq!(UnescapeHtmlEntities::unescape_numeric("&#X1f62d;"), "😭");
  }

  #[test]
  fn stage_leaves_none_fields_as_none() {
    Test::new()
      .stage(UnescapeHtmlEntities)
      .expected_metadata(Metadata::default())
      .run();
  }

  #[test]
  fn stage_unescapes_all_fields() {
    Test::new()
      .stage(UnescapeHtmlEntities)
      .metadata(Metadata {
        title: Some("foo &amp; bar".into()),
        byline: Some("foo &amp; bar".into()),
        excerpt: Some("foo &amp; bar".into()),
        site_name: Some("foo &amp; bar".into()),
        published_time: Some("foo &amp; bar".into()),
      })
      .expected_metadata(Metadata {
        title: Some("foo & bar".into()),
        byline: Some("foo & bar".into()),
        excerpt: Some("foo & bar".into()),
        site_name: Some("foo & bar".into()),
        published_time: Some("foo & bar".into()),
      })
      .run();
  }
}
