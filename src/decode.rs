use super::*;

static CHARSET: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(r#"(?i)<meta[^>]*charset\s*=\s*["']?\s*([a-z0-9_:.\-]+)"#).unwrap()
});

pub(crate) fn decode(bytes: Vec<u8>) -> Result<String> {
  let bytes = match String::from_utf8(bytes) {
    Ok(html) => return Ok(html),
    Err(error) => error.into_bytes(),
  };

  let (encoding, bom) = Encoding::for_bom(&bytes).unwrap_or_else(|| {
    let encoding = CHARSET
      .captures(&bytes[..bytes.len().min(1024)])
      .and_then(|captures| Encoding::for_label(&captures[1]))
      .unwrap_or(WINDOWS_1252);

    (encoding, 0)
  });

  encoding
    .decode_without_bom_handling_and_without_replacement(&bytes[bom..])
    .map(Cow::into_owned)
    .ok_or_else(|| anyhow!("input is not valid {}", encoding.name()))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn decodes() {
    #[track_caller]
    fn case(bytes: &[u8], expected: &str) {
      assert_eq!(decode(bytes.to_vec()).unwrap(), expected);
    }

    case(b"foo", "foo");
    case(b"b\xe1r", "b\u{e1}r");
    case(
      b"<meta charset=\"iso-8859-7\">\xe1",
      "<meta charset=\"iso-8859-7\">\u{3b1}",
    );
    case(b"\xff\xfef\x00o\x00o\x00", "foo");
  }

  #[test]
  fn rejects_malformed_input() {
    assert_eq!(
      decode(b"<meta charset=utf-8>\xff".to_vec())
        .unwrap_err()
        .to_string(),
      "input is not valid UTF-8",
    );
  }
}
