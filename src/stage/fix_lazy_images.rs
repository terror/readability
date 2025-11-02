use super::*;

/// Copies lazy-loading image sources into standard attributes so images load without JS.
pub struct FixLazyImagesStage;

impl Stage for FixLazyImagesStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(fragment) = context.article_fragment_mut() else {
      return Ok(());
    };

    Self::fix_lazy_images(fragment);

    Ok(())
  }
}

impl FixLazyImagesStage {
  fn class_contains_lazy(element: &Element) -> bool {
    element
      .attr("class")
      .is_some_and(|class| class.to_ascii_lowercase().contains("lazy"))
  }

  fn collect_lazy_sources(element: &Element) -> Vec<(String, String)> {
    element
      .attrs
      .iter()
      .filter_map(|(name, value)| {
        let attribute_name = name.local.as_ref();
        if matches!(attribute_name, "src" | "srcset" | "alt") {
          return None;
        }

        let attr_value = value.to_string();

        if re::SRCSET_CANDIDATE_VALUE.is_match(&attr_value) {
          Some(("srcset".to_owned(), attr_value))
        } else if re::LAZY_IMAGE_SRC_VALUE.is_match(&attr_value) {
          Some(("src".to_owned(), attr_value))
        } else {
          None
        }
      })
      .collect()
  }

  fn create_element(tag: &str) -> Node {
    Node::Element(Element::new(
      QualName::new(None, ns!(html), LocalName::from(tag)),
      Vec::new(),
    ))
  }

  fn find_attribute_index(element: &Element, name: &str) -> Option<usize> {
    element
      .attrs
      .iter()
      .position(|(attr_name, _)| attr_name.local.as_ref() == name)
  }

  fn fix_lazy_images(fragment: &mut ArticleFragment) {
    let Some(root) = fragment.html.tree.get(fragment.root_id) else {
      return;
    };

    let node_ids: Vec<NodeId> = root
      .descendants()
      .filter(|node| {
        matches!(
          node.value(),
          Node::Element(element)
            if matches!(element.name(), "img" | "picture" | "figure")
        )
      })
      .map(|node| node.id())
      .collect();

    for node_id in node_ids {
      Self::process_node(fragment, node_id);
    }
  }

  fn has_descendant_media(node: NodeRef<'_, Node>) -> bool {
    node
      .descendants()
      .skip(1)
      .any(|descendant| match descendant.value() {
        Node::Element(element) => matches!(element.name(), "img" | "picture"),
        _ => false,
      })
  }

  fn has_src(element: &Element) -> bool {
    element
      .attr("src")
      .is_some_and(|value| !value.trim().is_empty())
  }

  fn has_srcset(element: &Element) -> bool {
    element.attr("srcset").is_some_and(|value| {
      let trimmed = value.trim();
      !trimmed.is_empty() && !trimmed.eq_ignore_ascii_case("null")
    })
  }

  fn process_node(fragment: &mut ArticleFragment, node_id: NodeId) {
    let (tag_name, has_descendant_media) = {
      let Some(node) = fragment.html.tree.get(node_id) else {
        return;
      };

      let Node::Element(element) = node.value() else {
        return;
      };

      (
        element.name().to_string(),
        if element.name() == "figure" {
          Self::has_descendant_media(node)
        } else {
          false
        },
      )
    };

    let instructions;
    let mut needs_child_image = false;

    {
      let Some(mut node) = fragment.html.tree.get_mut(node_id) else {
        return;
      };

      let Node::Element(element) = node.value() else {
        return;
      };

      Self::remove_placeholder_src(element);

      let has_src = Self::has_src(element);
      let has_srcset = Self::has_srcset(element);
      let class_contains_lazy = Self::class_contains_lazy(element);

      if (has_src || has_srcset) && !class_contains_lazy {
        return;
      }

      instructions = Self::collect_lazy_sources(element);

      if instructions.is_empty() {
        return;
      }

      match tag_name.as_str() {
        "img" | "picture" => {
          for (attr, value) in &instructions {
            Self::set_attribute(element, attr, value);
          }
        }
        "figure" => {
          if has_descendant_media {
            return;
          }

          needs_child_image = true;
        }
        _ => return,
      }
    }

    if needs_child_image
      && let Some(mut figure_node) = fragment.html.tree.get_mut(node_id)
    {
      let mut img_node = Self::create_element("img");

      if let Node::Element(ref mut img_element) = img_node {
        for (attr, value) in &instructions {
          Self::set_attribute(img_element, attr, value);
        }
      }

      figure_node.append(img_node);
    }
  }

  fn remove_placeholder_src(element: &mut Element) {
    let Some(index) = Self::find_attribute_index(element, "src") else {
      return;
    };

    let src_value = element.attrs[index].1.to_string();

    let Some(captures) = re::BASE64_DATA_URL.captures(&src_value) else {
      return;
    };

    if captures
      .get(1)
      .is_some_and(|mime| mime.as_str().eq_ignore_ascii_case("image/svg+xml"))
    {
      return;
    }

    let src_could_be_removed =
      element
        .attrs
        .iter()
        .enumerate()
        .any(|(attr_index, (_, value))| {
          attr_index != index
            && re::IMAGE_EXTENSION_SUFFIX.is_match(value.as_ref())
        });

    if !src_could_be_removed {
      return;
    }

    let prefix_len = captures
      .name("data")
      .map(|m| m.start())
      .unwrap_or(src_value.len());

    let b64_length = src_value.len().saturating_sub(prefix_len);

    if b64_length < 133 {
      element.attrs.remove(index);
    }
  }

  fn set_attribute(element: &mut Element, name: &str, value: &str) {
    if let Some(index) = Self::find_attribute_index(element, name) {
      element.attrs[index].1.clear();
      element.attrs[index].1.push_slice(value);
    } else {
      let mut attr_value = StrTendril::new();
      attr_value.push_slice(value);
      element.attrs.push((
        QualName::new(None, ns!(), LocalName::from(name)),
        attr_value,
      ));
    }
  }
}
