use super::*;

pub(crate) struct UnwrapSingleCellTables;

impl Stage for UnwrapSingleCellTables {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(article) = context.article else {
      return Ok(());
    };

    for table in context.node(article).elements_with_tags(&["TABLE"]) {
      let body = if table.has_single_tag_inside_element("TBODY") {
        table.element_children()[0].clone()
      } else {
        table.clone()
      };

      if !body.has_single_tag_inside_element("TR") {
        continue;
      }

      let row = body.element_children()[0].clone();

      if !row.has_single_tag_inside_element("TD") {
        continue;
      }

      let cell = row.element_children()[0].clone();

      if cell.children().iter().all(NodeExt::is_phrasing_content) {
        cell.set_tag("p");
      } else {
        cell.set_tag("div");
      }

      table.replace_with(&cell);
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn unwraps_single_cell_tables() {
    Test::new()
      .stage(UnwrapSingleCellTables)
      .article()
      .document(
        "<html><body><table><tr><td>foo</td></tr></table><table><tr><td><div>bar</div></td></tr></table><table><tr><td>baz</td><td>qux</td></tr></table></body></html>",
      )
      .expected_html(
        "<html><head></head><body><p>foo</p><div><div>bar</div></div><table><tbody><tr><td>baz</td><td>qux</td></tr></tbody></table></body></html>",
      )
      .run();
  }
}
