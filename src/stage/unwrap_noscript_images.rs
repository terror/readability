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
  fn build_image_html(
    attributes: impl Iterator<Item = (String, String)>,
  ) -> String {
    let mut html = String::from("<img");

    for (name, value) in attributes {
      write!(html, " {name}=\"").unwrap();
      encode_double_quoted_attribute_to_string(&value, &mut html);
      html.push('"');
    }

    html.push_str("/>");

    html
  }

  fn find_image<'a>(selection: &Selection<'a>) -> Option<NodeRef<'a>> {
    selection
      .nodes()
      .iter()
      .find(|node| node.node_name().as_deref() == Some("img"))
      .cloned()
  }

  fn has_image_source(node: &NodeRef) -> bool {
    let has_source_attribute = SOURCE_ATTRIBUTES
      .iter()
      .filter_map(|attribute| node.attr(attribute))
      .any(|value| !value.trim().is_empty());

    let has_image_extension = node.attrs().into_iter().any(|attribute| {
      let value = attribute.value.to_lowercase();

      IMAGE_EXTENSIONS
        .iter()
        .any(|extension| value.contains(extension))
    });

    has_source_attribute || has_image_extension
  }

  fn is_single_image(selection: &Selection) -> bool {
    if !selection.text().trim().is_empty() {
      return false;
    }

    let direct_image_count = selection
      .nodes()
      .iter()
      .filter(|node| node.node_name().as_deref() == Some("img"))
      .count();

    direct_image_count + selection.select("img").length() == 1
  }

  fn remove_placeholder_images(context: &mut Context<'_>) {
    context
      .document
      .select("img")
      .nodes()
      .iter()
      .filter(|node| !Self::has_image_source(node))
      .cloned()
      .collect::<Vec<_>>()
      .into_iter()
      .for_each(|node| node.remove_from_parent());
  }

  fn unwrap_noscript_images(context: &mut Context<'_>) {
    let nodes = context.document.select("noscript").nodes().to_vec();

    for node in nodes {
      let inner_html = node.inner_html();

      if inner_html.trim().is_empty() {
        continue;
      }

      let fragment = dom_query::Document::from(inner_html.as_ref());

      let fragment_selection = fragment.select("body > *");

      let Some(new_image) = Self::is_single_image(&fragment_selection)
        .then(|| Self::find_image(&fragment_selection))
        .flatten()
      else {
        continue;
      };

      let Some(prev_sibling) = node.prev_element_sibling() else {
        continue;
      };

      let prev_selection = Selection::from(prev_sibling.clone());

      if !Self::is_single_image(&prev_selection) {
        continue;
      }

      let placeholder_image =
        if prev_sibling.node_name().as_deref() == Some("img") {
          Some(prev_sibling.clone())
        } else {
          Self::find_image(&prev_selection)
        };

      let new_attributes = new_image
        .attrs()
        .into_iter()
        .map(|attribute| {
          (
            attribute.name.local.to_string(),
            attribute.value.to_string(),
          )
        })
        .collect::<Vec<_>>();

      let placeholder_attributes = placeholder_image
        .into_iter()
        .flat_map(|image| {
          image.attrs().into_iter().filter_map(|attribute| {
            let name = attribute.name.local.to_string();

            let is_source = SOURCE_ATTRIBUTES.contains(&name.as_str());

            let already_exists = new_attributes
              .iter()
              .any(|(attribute_name, _)| attribute_name == &name);

            if is_source || already_exists {
              return None;
            }

            Some((name, attribute.value.to_string()))
          })
        })
        .collect::<Vec<_>>();

      prev_sibling.replace_with_html(Self::build_image_html(
        new_attributes.into_iter().chain(placeholder_attributes),
      ));

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
    expected: "<html><head></head><body><img src=\"real.jpg\" alt=\"x\"></body></html>",
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
