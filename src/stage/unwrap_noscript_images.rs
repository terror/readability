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
    for attribute in SOURCE_ATTRIBUTES {
      let Some(value) = node.attr(attribute) else {
        continue;
      };

      if !value.trim().is_empty() {
        return true;
      }
    }

    for attribute in node.attrs() {
      let value = attribute.value.to_lowercase();

      for extension in IMAGE_EXTENSIONS {
        if value.contains(extension) {
          return true;
        }
      }
    }

    false
  }

  fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
      .replace('"', "&quot;")
      .replace('<', "&lt;")
      .replace('>', "&gt;")
  }

  fn is_single_image(selection: &dom_query::Selection) -> bool {
    let mut img_count = 0;

    for node in selection.nodes() {
      if node.node_name().as_deref() == Some("img") {
        img_count += 1;
      }
    }

    img_count += selection.select("img").length();

    if img_count != 1 {
      return false;
    }

    selection.text().trim().is_empty()
  }

  fn remove_placeholder_images(context: &mut Context<'_>) {
    let nodes = context.document.select("img").nodes().to_vec();

    for node in nodes {
      if !Self::has_image_source(&node) {
        node.remove_from_parent();
      }
    }
  }

  fn unwrap_noscript_images(context: &mut Context<'_>) {
    let nodes = context.document.select("noscript").nodes().to_vec();

    for noscript_node in nodes {
      let inner_html = noscript_node.inner_html();

      if inner_html.trim().is_empty() {
        continue;
      }

      let fragment = dom_query::Document::from(inner_html.as_ref());

      let fragment_selection = fragment.select("body > *");

      if !Self::is_single_image(&fragment_selection) {
        continue;
      }

      let new_img = {
        let mut found = None;

        for node in fragment_selection.nodes() {
          if node.node_name().as_deref() == Some("img") {
            found = Some(node.clone());
            break;
          }
        }

        if found.is_none() {
          found = fragment_selection.select("img").nodes().first().cloned();
        }

        found
      };

      let Some(new_img) = new_img else {
        continue;
      };

      let Some(prev_sibling) = noscript_node.prev_element_sibling() else {
        continue;
      };

      let prev_selection = dom_query::Selection::from(prev_sibling.clone());

      if !Self::is_single_image(&prev_selection) {
        continue;
      }

      let placeholder_img =
        if prev_sibling.node_name().as_deref() == Some("img") {
          Some(prev_sibling.clone())
        } else {
          prev_selection.select("img").nodes().first().cloned()
        };

      let mut attributes = new_img
        .attrs()
        .into_iter()
        .map(|attribute| {
          (
            attribute.name.local.to_string(),
            attribute.value.to_string(),
          )
        })
        .collect::<Vec<(String, String)>>();

      if let Some(placeholder) = placeholder_img {
        for attribute in placeholder.attrs() {
          let attribute_name = attribute.name.local.to_string();

          if SOURCE_ATTRIBUTES.contains(&attribute_name.as_str()) {
            continue;
          }

          if !attributes.iter().any(|(name, _)| name == &attribute_name) {
            attributes.push((attribute_name, attribute.value.to_string()));
          }
        }
      }

      let mut img_html = String::from("<img");

      for (name, value) in attributes {
        img_html.push(' ');
        img_html.push_str(&name);
        img_html.push_str("=\"");
        img_html.push_str(&Self::html_escape(&value));
        img_html.push('"');
      }

      img_html.push_str("/>");

      prev_sibling.replace_with_html(img_html);

      noscript_node.remove_from_parent();
    }
  }
}

#[cfg(test)]
mod tests {
  use super::super::test;
  use super::*;

  test! {
    name: basic_unwrap,
    stage: UnwrapNoscriptImages,
    content: r#"<html><body><img alt="x"/><noscript><img src="real.jpg"/></noscript></body></html>"#,
    expected: r#"<html><head></head><body><img src="real.jpg" alt="x"></body></html>"#,
  }

  test! {
    name: remove_placeholder_without_src,
    stage: UnwrapNoscriptImages,
    content: r#"<html><body><img class="lazy"/></body></html>"#,
    expected: r#"<html><head></head><body></body></html>"#,
  }

  test! {
    name: keep_valid_image,
    stage: UnwrapNoscriptImages,
    content: r#"<html><body><img src="valid.jpg"/></body></html>"#,
    expected: r#"<html><head></head><body><img src="valid.jpg"></body></html>"#,
  }

  test! {
    name: non_image_noscript_unchanged,
    stage: UnwrapNoscriptImages,
    content: r#"<html><body><noscript><p>Enable JS</p></noscript></body></html>"#,
    expected: r#"<html><head></head><body><noscript><p>Enable JS</p></noscript></body></html>"#,
  }

  test! {
    name: no_previous_sibling_unchanged,
    stage: UnwrapNoscriptImages,
    content: r#"<html><body><noscript><img src="x.jpg"/></noscript></body></html>"#,
    expected: r#"<html><head></head><body><noscript><img src="x.jpg"></noscript></body></html>"#,
  }

  test! {
    name: nested_wrapper,
    stage: UnwrapNoscriptImages,
    content: r#"<html><body><div><img/></div><noscript><img src="real.jpg"/></noscript></body></html>"#,
    expected: r#"<html><head></head><body><img src="real.jpg"></body></html>"#,
  }

  test! {
    name: preserves_data_src,
    stage: UnwrapNoscriptImages,
    content: r#"<html><body><img data-src="lazy.jpg"/></body></html>"#,
    expected: r#"<html><head></head><body><img data-src="lazy.jpg"></body></html>"#,
  }

  test! {
    name: preserves_srcset,
    stage: UnwrapNoscriptImages,
    content: r#"<html><body><img srcset="img-320w.jpg 320w, img-480w.jpg 480w"/></body></html>"#,
    expected: r#"<html><head></head><body><img srcset="img-320w.jpg 320w, img-480w.jpg 480w"></body></html>"#,
  }

  test! {
    name: preserves_image_extension_in_attr,
    stage: UnwrapNoscriptImages,
    content: r#"<html><body><img data-lazy="image.png"/></body></html>"#,
    expected: r#"<html><head></head><body><img data-lazy="image.png"></body></html>"#,
  }
}
