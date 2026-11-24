use super::*;

const MAX_PLACEHOLDER_BASE64_LENGTH: usize = 133;

pub(crate) struct FixLazyImages;

impl FixLazyImages {
  fn fix(element: &NodeRef) {
    let tag = element.tag();

    if tag == "IMG" {
      if Self::is_svg_placeholder(element) {
        return;
      }

      Self::remove_placeholder_src(element);
    }

    let has_src = tag == "IMG" && element.has_attr("src");

    let has_srcset = tag == "IMG"
      && element
        .attr("srcset")
        .is_some_and(|srcset| !srcset.is_empty() && srcset.as_ref() != "null");

    let lazy = element
      .attr("class")
      .is_some_and(|class| class.to_lowercase().contains("lazy"));

    if (has_src || has_srcset) && !lazy {
      return;
    }

    for attribute in element.attrs() {
      let name = attribute.name.local.as_ref();

      if matches!(name, "src" | "srcset" | "alt") {
        continue;
      }

      let value = attribute.value.as_ref();

      let copy_to = if LAZY_IMAGE_SRCSET.is_match(value) {
        "srcset"
      } else if LAZY_IMAGE_SRC.is_match(value) {
        "src"
      } else {
        continue;
      };

      if tag == "IMG" || tag == "PICTURE" {
        element.set_attr(copy_to, value);
      } else if element.elements_with_tags(&["IMG", "PICTURE"]).is_empty() {
        let image = create_element(element.tree, "img");
        image.set_attr(copy_to, value);
        element.append_child(&image);
      }
    }
  }

  fn is_svg_placeholder(element: &NodeRef) -> bool {
    element.attr("src").is_some_and(|src| {
      B64_DATA_URL
        .captures(&src)
        .is_some_and(|captures| &captures[1] == "image/svg+xml")
    })
  }

  fn remove_placeholder_src(element: &NodeRef) {
    let Some(src) = element.attr("src") else {
      return;
    };

    let Some(captures) = B64_DATA_URL.captures(&src) else {
      return;
    };

    let has_other_image = element.attrs().iter().any(|attribute| {
      attribute.name.local.as_ref() != "src"
        && IMAGE_EXTENSION.is_match(&attribute.value)
    });

    if has_other_image
      && text_length(&src) - text_length(&captures[0])
        < MAX_PLACEHOLDER_BASE64_LENGTH
    {
      element.remove_attr("src");
    }
  }
}

impl Stage for FixLazyImages {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(article) = context.extraction.article else {
      return Ok(());
    };

    for element in context
      .node(article)
      .elements_with_tags(&["IMG", "PICTURE", "FIGURE"])
    {
      Self::fix(&element);
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn copies_lazy_sources() {
    Test::new()
      .stage(FixLazyImages)
      .article()
      .document(
        r#"<html><body><img data-src="foo.jpg"><img src="bar.png" class="lazy" data-srcset="baz.jpg 2x"><img src="qux.png" data-src="foo.jpg"><figure data-src="foo.jpg"></figure></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><img data-src="foo.jpg" src="foo.jpg"><img src="bar.png" class="lazy" data-srcset="baz.jpg 2x" srcset="baz.jpg 2x"><img src="qux.png" data-src="foo.jpg"><figure data-src="foo.jpg"><img src="foo.jpg"></figure></body></html>"#,
      )
      .run();
  }

  #[test]
  fn removes_placeholder_data_url() {
    Test::new()
      .stage(FixLazyImages)
      .article()
      .document(
        r#"<html><body><img src="data:image/png;base64,AAAA" data-src="foo.jpg"><img class="lazy" src="data:image/svg+xml;base64,AAAA" data-src="foo.jpg"></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><img data-src="foo.jpg" src="foo.jpg"><img class="lazy" src="data:image/svg+xml;base64,AAAA" data-src="foo.jpg"></body></html>"#,
      )
      .run();
  }
}
