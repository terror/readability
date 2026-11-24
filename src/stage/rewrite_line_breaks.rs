use super::*;

/// Converts `<br><br>` paragraph separators into explicit paragraph elements.
///
/// The stage scans `br` chains, replaces break runs with `<p>`, and moves
/// following phrasing content into the new paragraph until a block boundary or
/// the next break run is reached. It also trims trailing whitespace-like nodes
/// and rewrites parent `<p>` containers to `<div>` when nested paragraphs would
/// otherwise be produced.
pub(crate) struct RewriteLineBreaks;

impl RewriteLineBreaks {
  fn next_significant<'a>(start: Option<&NodeRef<'a>>) -> Option<NodeRef<'a>> {
    iter::successors(start.copied(), NodeRef::next_sibling).find(|node| {
      node.is_element() || (node.is_text() && !node.text().trim().is_empty())
    })
  }
}

impl Stage for RewriteLineBreaks {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    for node in context.document.select("br").nodes().to_vec() {
      if node.parent().is_none() {
        continue;
      }

      let mut next = node.next_sibling();
      let mut replaced = false;

      while let Some(sibling) = Self::next_significant(next.as_ref()) {
        if !sibling.is("br") {
          break;
        }

        replaced = true;
        next = sibling.next_sibling();
        sibling.remove_from_parent();
      }

      if !replaced {
        continue;
      }

      let par = create_element(&context.document.tree, "p");
      node.replace_with(&par);

      let mut next = par.next_sibling();

      while let Some(sibling) = next {
        if sibling.is("br")
          && Self::next_significant(sibling.next_sibling().as_ref())
            .is_some_and(|node| node.is("br"))
        {
          break;
        }

        if !sibling.is_phrasing_content() {
          break;
        }

        next = sibling.next_sibling();
        par.append_child(&sibling);
      }

      while par.last_child().is_some_and(|node| node.is_whitespace()) {
        par.last_child().unwrap().remove_from_parent();
      }

      if par.parent().is_some_and(|parent| parent.is("p")) {
        par.parent().unwrap().set_tag("div");
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn anchor_with_phrasing_children_is_phrasing() {
    Test::new()
      .stage(RewriteLineBreaks)
      .document(
        "<html><body><div>foo<br><br><a href=\"#\"><b>link</b></a></div></body></html>",
      )
      .expected_html(
        "<html><head></head><body><div>foo<p><a href=\"#\"><b>link</b></a></p></div></body></html>",
      )
      .run();
  }

  #[test]
  fn collects_phrasing_content() {
    Test::new()
      .stage(RewriteLineBreaks)
      .document(
        "<html><body><div>foo<br><br>bar <b>baz</b></div></body></html>",
      )
      .expected_html(
        "<html><head></head><body><div>foo<p>bar <b>baz</b></p></div></body></html>",
      )
      .run();
  }

  #[test]
  fn handles_br_at_end() {
    Test::new()
      .stage(RewriteLineBreaks)
      .document("<html><body><div>foo<br><br></div></body></html>")
      .expected_html(
        "<html><head></head><body><div>foo<p></p></div></body></html>",
      )
      .run();
  }

  #[test]
  fn parent_p_becomes_div() {
    Test::new()
      .stage(RewriteLineBreaks)
      .document("<html><body><p>foo<br><br>bar</p></body></html>")
      .expected_html(
        "<html><head></head><body><div>foo<p>bar</p></div></body></html>",
      )
      .run();
  }

  #[test]
  fn replaces_double_br_with_p() {
    Test::new()
      .stage(RewriteLineBreaks)
      .document("<html><body><div>foo<br><br>bar</div></body></html>")
      .expected_html(
        "<html><head></head><body><div>foo<p>bar</p></div></body></html>",
      )
      .run();
  }

  #[test]
  fn single_br_unchanged() {
    Test::new()
      .stage(RewriteLineBreaks)
      .document("<html><body><div>foo<br>bar</div></body></html>")
      .expected_html(
        "<html><head></head><body><div>foo<br>bar</div></body></html>",
      )
      .run();
  }

  #[test]
  fn stops_at_block_element() {
    Test::new()
      .stage(RewriteLineBreaks)
      .document(
        "<html><body><div>foo<br><br>bar<div>block</div></div></body></html>",
      )
      .expected_html(
        "<html><head></head><body><div>foo<p>bar</p><div>block</div></div></body></html>",
      )
      .run();
  }

  #[test]
  fn stops_at_next_br_chain() {
    Test::new()
      .stage(RewriteLineBreaks)
      .document("<html><body><div>a<br><br>b<br><br>c</div></body></html>")
      .expected_html(
        "<html><head></head><body><div>a<p>b</p><p>c</p></div></body></html>",
      )
      .run();
  }

  #[test]
  fn trims_trailing_whitespace_nodes() {
    Test::new()
      .stage(RewriteLineBreaks)
      .document("<html><body><div>foo<br><br>bar<br></div></body></html>")
      .expected_html(
        "<html><head></head><body><div>foo<p>bar</p></div></body></html>",
      )
      .run();
  }

  #[test]
  fn triple_br_becomes_single_p() {
    Test::new()
      .stage(RewriteLineBreaks)
      .document("<html><body><div>foo<br><br><br>bar</div></body></html>")
      .expected_html(
        "<html><head></head><body><div>foo<p>bar</p></div></body></html>",
      )
      .run();
  }

  #[test]
  fn whitespace_between_brs_ignored_for_chain_detection() {
    Test::new()
      .stage(RewriteLineBreaks)
      .document("<html><body><div>foo<br> <br>bar</div></body></html>")
      .expected_html(
        "<html><head></head><body><div>foo<p> bar</p></div></body></html>",
      )
      .run();
  }
}
