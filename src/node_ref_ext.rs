use super::*;

pub(crate) trait NodeRefExt {
  fn is_hidden(&self) -> bool;
}

impl NodeRefExt for NodeRef<'_> {
  fn is_hidden(&self) -> bool {
    if self.attr("hidden").is_some() {
      return true;
    }

    if self
      .attr("aria-hidden")
      .is_some_and(|value| value.as_ref() == "true")
    {
      return true;
    }

    let style = self.attr("style").unwrap_or_default().to_lowercase();

    style.contains("display:none")
      || style.contains("display: none")
      || style.contains("visibility:hidden")
      || style.contains("visibility: hidden")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn is_hidden(html: &str) -> bool {
    let document = dom_query::Document::from(
      format!("<html><body><span{html}</span></body></html>").as_str(),
    );

    document.select("span").nodes().first().unwrap().is_hidden()
  }

  #[test]
  fn aria_hidden_false() {
    assert!(!is_hidden(r#" aria-hidden="false">foo"#));
  }

  #[test]
  fn aria_hidden_true() {
    assert!(is_hidden(r#" aria-hidden="true">foo"#));
  }

  #[test]
  fn display_none() {
    assert!(is_hidden(r#" style="display:none">foo"#));
  }

  #[test]
  fn display_none_with_space() {
    assert!(is_hidden(r#" style="display: none">foo"#));
  }

  #[test]
  fn hidden_attribute() {
    assert!(is_hidden(" hidden>foo"));
  }

  #[test]
  fn visible() {
    assert!(!is_hidden(r#">foo"#));
  }

  #[test]
  fn visibility_hidden() {
    assert!(is_hidden(r#" style="visibility:hidden">foo"#));
  }

  #[test]
  fn visibility_hidden_with_space() {
    assert!(is_hidden(r#" style="visibility: hidden">foo"#));
  }
}
