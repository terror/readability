use super::*;

const MEDIA_TAGS: &[&str] =
  &["IMG", "PICTURE", "FIGURE", "VIDEO", "AUDIO", "SOURCE"];

pub(crate) struct FixRelativeUris;

impl FixRelativeUris {
  fn normalize_drive_letter(mut url: Url) -> Url {
    if url.scheme() != "file" {
      return url;
    }

    let path = url.path();

    let bytes = path.as_bytes();

    if bytes.len() >= 3
      && bytes[0] == b'/'
      && bytes[1].is_ascii_alphabetic()
      && bytes[2] == b'|'
      && (bytes.len() == 3 || bytes[3] == b'/')
    {
      let path = format!("/{}:{}", char::from(bytes[1]), &path[3..]);
      url.set_path(&path);
    }

    url
  }

  fn unwrap_link(link: &NodeRef) {
    let children = link.children();

    if let [child] = children.as_slice()
      && child.is_text()
    {
      link.replace_with(&link.tree.new_text(link.text()));
      return;
    }

    let span = create_element(link.tree, "span");

    for child in children {
      span.append_child(&child);
    }

    link.replace_with(&span);
  }
}

impl Stage for FixRelativeUris {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let (Some(article), Some(document_url)) =
      (context.extraction.article, context.url.clone())
    else {
      return Ok(());
    };

    let base_url = context
      .document
      .select("base[href]")
      .nodes()
      .first()
      .and_then(|base| base.attr("href"))
      .and_then(|href| document_url.join(&href).ok())
      .unwrap_or_else(|| document_url.clone());

    let resolve = |uri: &str| {
      if base_url == document_url && uri.starts_with('#') {
        return uri.to_string();
      }
      base_url.join(uri).map_or_else(
        |_| uri.to_string(),
        |url| Self::normalize_drive_letter(url).to_string(),
      )
    };

    let article = context.node(article);

    for link in article.elements_with_tags(&["A"]) {
      let Some(href) = link.attr("href").filter(|href| !href.is_empty()) else {
        continue;
      };

      if href.starts_with("javascript:") {
        Self::unwrap_link(&link);
      } else {
        link.set_attr("href", &resolve(&href));
      }
    }

    for media in article.elements_with_tags(MEDIA_TAGS) {
      for name in ["src", "poster"] {
        if let Some(value) = media.attr(name).filter(|value| !value.is_empty())
        {
          media.set_attr(name, &resolve(&value));
        }
      }

      if let Some(srcset) =
        media.attr("srcset").filter(|value| !value.is_empty())
      {
        let srcset = SRCSET_URL.replace_all(&srcset, |captures: &Captures| {
          format!(
            "{}{}{}",
            resolve(&captures[1]),
            captures.get(2).map_or("", |descriptor| descriptor.as_str()),
            &captures[3],
          )
        });

        media.set_attr("srcset", &srcset);
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[track_caller]
  fn case(html: &str, expected: &str) {
    let mut document = dom_query::Document::from(html);

    let options = ParserOptions::default();

    let mut context = Context::new(&mut document, &options);

    context.url = Some(Url::parse("http://foo.com/bar/baz.html").unwrap());

    context.extraction.article = context
      .document
      .select("body")
      .nodes()
      .first()
      .map(|node| node.id);

    FixRelativeUris.run(&mut context).unwrap();

    assert_eq!(
      context.document.select("body").nodes()[0]
        .inner_html()
        .to_string(),
      expected
    );
  }

  #[test]
  fn normalizes_windows_drive_letters() {
    case(
      r#"<img src="file:///C|/foo.gif">"#,
      r#"<img src="file:///C:/foo.gif">"#,
    );
  }

  #[test]
  fn resolves_against_base_element() {
    case(
      r##"<html><head><base href="/qux/"></head><body><a href="foo">bar</a><a href="#baz">qux</a></body></html>"##,
      r#"<a href="http://foo.com/qux/foo">bar</a><a href="http://foo.com/qux/#baz">qux</a>"#,
    );
  }

  #[test]
  fn resolves_links_and_media() {
    case(
      r##"<a href="qux">foo</a><a href="#bar">baz</a><img src="/qux.png" srcset="a.png 1x, b.png 2x"><video poster="c.png"></video>"##,
      r##"<a href="http://foo.com/bar/qux">foo</a><a href="#bar">baz</a><img src="http://foo.com/qux.png" srcset="http://foo.com/bar/a.png 1x, http://foo.com/bar/b.png 2x"><video poster="http://foo.com/bar/c.png"></video>"##,
    );
  }

  #[test]
  fn unwraps_javascript_links() {
    case(
      r#"<a href="javascript:foo()">bar</a><a href="javascript:baz()"><b>qux</b></a>"#,
      "bar<span><b>qux</b></span>",
    );
  }
}
