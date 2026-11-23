use super::*;

const SOURCE_ATTRIBUTES: &[&str] =
  &["src", "srcset", "data-src", "data-srcset"];

/// Resolves lazy-loaded image placeholders paired with `<noscript>` fallbacks.
///
/// First removes `<img>` elements that have no source attribute and no
/// attribute value that looks like an image URL. Then, for each `noscript`
/// element that contains a single image and follows an element that is itself
/// a single image, replaces that element with the `noscript` image. Source
/// attributes from the replaced image are copied onto the new one, prefixed
/// with `data-old-` when the new image already has a different value.
pub(crate) struct UnwrapNoscriptImages;

impl Stage for UnwrapNoscriptImages {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    for image in context.document.select("img").nodes() {
      if !Self::has_image_source(image) {
        image.remove_from_parent();
      }
    }

    for noscript in context.document.select("noscript").nodes() {
      Self::unwrap(noscript);
    }

    Ok(())
  }
}

impl UnwrapNoscriptImages {
  fn first_image<'a>(node: &NodeRef<'a>) -> Option<NodeRef<'a>> {
    if node.tag() == "IMG" {
      Some(node.clone())
    } else {
      node.elements_with_tags(&["IMG"]).into_iter().next()
    }
  }

  fn has_image_source(image: &NodeRef) -> bool {
    image.attrs().iter().any(|attribute| {
      SOURCE_ATTRIBUTES.contains(&attribute.name.local.as_ref())
        || IMAGE_EXTENSION.is_match(&attribute.value)
    })
  }

  fn is_single_image(node: &NodeRef) -> bool {
    let mut node = node.clone();

    loop {
      if node.tag() == "IMG" {
        return true;
      }

      let children = node.element_children();

      if children.len() != 1 || !node.text().trim().is_empty() {
        return false;
      }

      node = children[0].clone();
    }
  }

  fn unwrap(noscript: &NodeRef) {
    if !Self::is_single_image(noscript) {
      return;
    }

    let Some(previous) = noscript
      .prev_element_sibling()
      .filter(Self::is_single_image)
    else {
      return;
    };

    let (Some(replacement), Some(old_image)) =
      (noscript.first_element_child(), Self::first_image(&previous))
    else {
      return;
    };

    let Some(new_image) = Self::first_image(&replacement) else {
      return;
    };

    for attribute in old_image.attrs() {
      let name = attribute.name.local.as_ref();
      let value = attribute.value.as_ref();

      if value.is_empty()
        || !(matches!(name, "src" | "srcset")
          || IMAGE_EXTENSION.is_match(value))
        || new_image
          .attr(name)
          .is_some_and(|current| current.as_ref() == value)
      {
        continue;
      }

      if new_image.has_attr(name) {
        new_image.set_attr(&format!("data-old-{name}"), value);
      } else {
        new_image.set_attr(name, value);
      }
    }

    previous.replace_with(&replacement);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn basic_unwrap() {
    Test::new()
      .stage(UnwrapNoscriptImages)
      .document(r#"<html><body><img src="foo.gif" alt="bar"><noscript><img src="baz.jpg"></noscript></body></html>"#)
      .expected_html(r#"<html><head></head><body><img src="baz.jpg" data-old-src="foo.gif"><noscript></noscript></body></html>"#)
      .run();
  }

  #[test]
  fn remove_placeholder_without_src() {
    Test::new()
      .stage(UnwrapNoscriptImages)
      .document("<html><body><img class=\"lazy\"/></body></html>")
      .expected_html("<html><head></head><body></body></html>")
      .run();
  }

  #[test]
  fn keep_valid_image() {
    Test::new()
      .stage(UnwrapNoscriptImages)
      .document("<html><body><img src=\"valid.jpg\"/></body></html>")
      .expected_html(
        "<html><head></head><body><img src=\"valid.jpg\"></body></html>",
      )
      .run();
  }

  #[test]
  fn non_image_noscript_unchanged() {
    Test::new()
      .stage(UnwrapNoscriptImages)
      .document("<html><body><noscript><p>Enable JS</p></noscript></body></html>")
      .expected_html(
        "<html><head></head><body><noscript><p>Enable JS</p></noscript></body></html>",
      )
      .run();
  }

  #[test]
  fn no_previous_sibling_unchanged() {
    Test::new()
      .stage(UnwrapNoscriptImages)
      .document("<html><body><noscript><img src=\"x.jpg\"/></noscript></body></html>")
      .expected_html(
        "<html><head></head><body><noscript><img src=\"x.jpg\"></noscript></body></html>",
      )
      .run();
  }

  #[test]
  fn nested_wrapper() {
    Test::new()
      .stage(UnwrapNoscriptImages)
      .document(r#"<html><body><div><img data-src="foo.jpg"></div><noscript><img src="bar.jpg"></noscript></body></html>"#)
      .expected_html(r#"<html><head></head><body><img src="bar.jpg" data-src="foo.jpg"><noscript></noscript></body></html>"#)
      .run();
  }

  #[test]
  fn preserves_data_src() {
    Test::new()
      .stage(UnwrapNoscriptImages)
      .document("<html><body><img data-src=\"lazy.jpg\"/></body></html>")
      .expected_html(
        "<html><head></head><body><img data-src=\"lazy.jpg\"></body></html>",
      )
      .run();
  }

  #[test]
  fn preserves_srcset() {
    Test::new()
      .stage(UnwrapNoscriptImages)
      .document("<html><body><img srcset=\"img-320w.jpg 320w, img-480w.jpg 480w\"/></body></html>")
      .expected_html(
        "<html><head></head><body><img srcset=\"img-320w.jpg 320w, img-480w.jpg 480w\"></body></html>",
      )
      .run();
  }

  #[test]
  fn preserves_image_extension_in_attr() {
    Test::new()
      .stage(UnwrapNoscriptImages)
      .document("<html><body><img data-lazy=\"image.png\"/></body></html>")
      .expected_html(
        "<html><head></head><body><img data-lazy=\"image.png\"></body></html>",
      )
      .run();
  }
}
