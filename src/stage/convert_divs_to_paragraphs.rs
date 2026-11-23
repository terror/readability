use super::*;

const MAX_LINK_DENSITY: f64 = 0.25;

pub(crate) struct ConvertDivsToParagraphs;

impl Stage for ConvertDivsToParagraphs {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let divs = context.document.select("div").nodes().to_vec();

    for div in divs {
      Self::wrap_phrasing_content(&div);

      if div.has_single_tag_inside_element("P")
        && div.link_density() < MAX_LINK_DENSITY
      {
        div.replace_with(&div.element_children()[0]);
      } else if !div.has_child_block_element() {
        div.set_tag("p");
      }
    }

    Ok(())
  }
}

impl ConvertDivsToParagraphs {
  fn wrap_phrasing_content(div: &NodeRef) {
    let mut child = div.first_child();

    while let Some(node) = child {
      if !node.is_phrasing_content() {
        child = node.next_sibling();
        continue;
      }

      let mut run = vec![node.clone()];
      let mut next = node.next_sibling();

      while let Some(sibling) =
        next.clone().filter(NodeExt::is_phrasing_content)
      {
        next = sibling.next_sibling();
        run.push(sibling);
      }

      let start = run.iter().position(|node| !node.is_whitespace());
      let end = run.iter().rposition(|node| !node.is_whitespace());

      if let (Some(start), Some(end)) = (start, end) {
        for node in run[..start].iter().chain(&run[end + 1..]) {
          node.remove_from_parent();
        }

        let paragraph = create_element(div.tree, "p");

        match &next {
          Some(next) => next.insert_before(&paragraph),
          None => div.append_child(&paragraph),
        }

        for node in &run[start..=end] {
          paragraph.append_child(node);
        }
      } else {
        for node in &run {
          node.remove_from_parent();
        }
      }

      child = next;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn converts_div_without_block_children() {
    Test::new()
      .stage(ConvertDivsToParagraphs)
      .document(r#"<html><body><div class="foo"><iframe></iframe></div></body></html>"#)
      .expected_html(
        r#"<html><head></head><body><p class="foo"><iframe></iframe></p></body></html>"#,
      )
      .run();
  }

  #[test]
  fn replaces_div_with_single_paragraph() {
    Test::new()
      .stage(ConvertDivsToParagraphs)
      .document("<html><body><div> <p>foo</p> </div></body></html>")
      .expected_html("<html><head></head><body><p>foo</p></body></html>")
      .run();
  }

  #[test]
  fn keeps_div_with_single_link_paragraph() {
    Test::new()
      .stage(ConvertDivsToParagraphs)
      .document(
        r#"<html><body><div><p><a href="/">foo</a></p></div></body></html>"#,
      )
      .expected_html(
        r#"<html><head></head><body><div><p><a href="/">foo</a></p></div></body></html>"#,
      )
      .run();
  }

  #[test]
  fn wraps_phrasing_content() {
    Test::new()
      .stage(ConvertDivsToParagraphs)
      .document(
        "<html><body><div> <br>foo <b>bar</b><br> <p>baz</p>qux</div></body></html>",
      )
      .expected_html(
        "<html><head></head><body><div><p>foo <b>bar</b></p><p>baz</p><p>qux</p></div></body></html>",
      )
      .run();
  }

  #[test]
  fn removes_whitespace_only_runs() {
    Test::new()
      .stage(ConvertDivsToParagraphs)
      .document("<html><body><div> <br> <p>foo</p><p>bar</p></div></body></html>")
      .expected_html(
        "<html><head></head><body><div><p>foo</p><p>bar</p></div></body></html>",
      )
      .run();
  }
}
