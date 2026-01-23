use super::*;

const IMAGE_EXTENSIONS: &[&str] = &[".jpg", ".jpeg", ".png", ".webp"];

const SOURCE_ATTRIBUTES: &[&str] =
  &["src", "srcset", "data-src", "data-srcset"];

pub struct UnwrapNoscriptImages;

impl Stage for UnwrapNoscriptImages {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    Self::unwrap_noscript_images(context);
    Self::remove_placeholder_images(context);
    Ok(())
  }
}

impl UnwrapNoscriptImages {
  fn has_image_source(node: &NodeRef) -> bool {
    node.attrs().iter().any(|attr| {
      SOURCE_ATTRIBUTES.contains(&attr.name.local.as_ref())
        || IMAGE_EXTENSIONS
          .iter()
          .any(|ext| attr.value.to_lowercase().contains(ext))
    })
  }

  fn remove_placeholder_images(context: &mut Context<'_>) {
    for node in context.document.select("img").nodes().to_vec() {
      if !Self::has_image_source(&node) {
        node.remove_from_parent();
      }
    }
  }

  fn single_image<'a>(selection: &Selection<'a>) -> Option<NodeRef<'a>> {
    if !selection.text().trim().is_empty() {
      return None;
    }

    let (direct, nested) = (selection.filter("img"), selection.select("img"));

    if direct.length() + nested.length() != 1 {
      return None;
    }

    direct.nodes().first().or(nested.nodes().first()).cloned()
  }

  fn unwrap_noscript_images(context: &mut Context<'_>) {
    for node in context.document.select("noscript").nodes().to_vec() {
      let fragment = dom_query::Document::from(node.inner_html().as_ref());

      let Some(new_image) = Self::single_image(&fragment.select("body > *"))
      else {
        continue;
      };

      let Some(prev_element_sibling) = node.prev_element_sibling() else {
        continue;
      };

      let Some(placeholder) =
        Self::single_image(&Selection::from(prev_element_sibling.clone()))
      else {
        continue;
      };

      placeholder.remove_attrs(SOURCE_ATTRIBUTES);

      for attribute in &new_image.attrs() {
        placeholder.set_attr(&attribute.name.local, &attribute.value);
      }

      if placeholder.id != prev_element_sibling.id {
        prev_element_sibling.replace_with(&placeholder);
      }

      node.remove_from_parent();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  test! {
    name: basic_unwrap,
    stage: UnwrapNoscriptImages,
    content: "<html><body><img alt=\"x\"/><noscript><img src=\"real.jpg\"/></noscript></body></html>",
    expected: "<html><head></head><body><img alt=\"x\" src=\"real.jpg\"></body></html>",
  }

  test! {
    name: remove_placeholder_without_src,
    stage: UnwrapNoscriptImages,
    content: "<html><body><img class=\"lazy\"/></body></html>",
    expected: "<html><head></head><body></body></html>",
  }

  test! {
    name: keep_valid_image,
    stage: UnwrapNoscriptImages,
    content: "<html><body><img src=\"valid.jpg\"/></body></html>",
    expected: "<html><head></head><body><img src=\"valid.jpg\"></body></html>",
  }

  test! {
    name: non_image_noscript_unchanged,
    stage: UnwrapNoscriptImages,
    content: "<html><body><noscript><p>Enable JS</p></noscript></body></html>",
    expected: "<html><head></head><body><noscript><p>Enable JS</p></noscript></body></html>",
  }

  test! {
    name: no_previous_sibling_unchanged,
    stage: UnwrapNoscriptImages,
    content: "<html><body><noscript><img src=\"x.jpg\"/></noscript></body></html>",
    expected: "<html><head></head><body><noscript><img src=\"x.jpg\"></noscript></body></html>",
  }

  test! {
    name: nested_wrapper,
    stage: UnwrapNoscriptImages,
    content: "<html><body><div><img/></div><noscript><img src=\"real.jpg\"/></noscript></body></html>",
    expected: "<html><head></head><body><img src=\"real.jpg\"></body></html>",
  }

  test! {
    name: preserves_data_src,
    stage: UnwrapNoscriptImages,
    content: "<html><body><img data-src=\"lazy.jpg\"/></body></html>",
    expected: "<html><head></head><body><img data-src=\"lazy.jpg\"></body></html>",
  }

  test! {
    name: preserves_srcset,
    stage: UnwrapNoscriptImages,
    content: "<html><body><img srcset=\"img-320w.jpg 320w, img-480w.jpg 480w\"/></body></html>",
    expected: "<html><head></head><body><img srcset=\"img-320w.jpg 320w, img-480w.jpg 480w\"></body></html>",
  }

  test! {
    name: preserves_image_extension_in_attr,
    stage: UnwrapNoscriptImages,
    content: "<html><body><img data-lazy=\"image.png\"/></body></html>",
    expected: "<html><head></head><body><img data-lazy=\"image.png\"></body></html>",
  }
}
