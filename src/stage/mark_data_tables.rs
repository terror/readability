use super::*;

const DATA_TABLE_DESCENDANTS: &[&str] =
  &["COL", "COLGROUP", "TFOOT", "THEAD", "TH"];

pub(crate) struct MarkDataTables;

impl Stage for MarkDataTables {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(article) = context.article else {
      return Ok(());
    };

    let tables = context
      .node(article)
      .elements_with_tags(&["TABLE"])
      .into_iter()
      .filter(Self::is_data_table)
      .map(|table| table.id)
      .collect::<Vec<_>>();

    context.data_tables.extend(tables);

    Ok(())
  }
}

impl MarkDataTables {
  fn is_data_table(table: &NodeRef) -> bool {
    if table
      .attr("role")
      .is_some_and(|role| role.as_ref() == "presentation")
    {
      return false;
    }

    if table
      .attr("datatable")
      .is_some_and(|value| value.as_ref() == "0")
    {
      return false;
    }

    if table
      .attr("summary")
      .is_some_and(|summary| !summary.is_empty())
    {
      return true;
    }

    if table
      .elements_with_tags(&["CAPTION"])
      .first()
      .is_some_and(|caption| caption.first_child().is_some())
    {
      return true;
    }

    if !table.elements_with_tags(DATA_TABLE_DESCENDANTS).is_empty() {
      return true;
    }

    if !table.elements_with_tags(&["TABLE"]).is_empty() {
      return false;
    }

    let (rows, columns) = Self::row_and_column_count(table);

    if rows == 1 || columns == 1 {
      return false;
    }

    if rows >= 10 || columns > 4 {
      return true;
    }

    rows * columns > 10
  }

  fn row_and_column_count(table: &NodeRef) -> (usize, usize) {
    let span = |node: &NodeRef, name: &str| {
      node
        .attr(name)
        .and_then(|value| parse_int(&value))
        .filter(|value| *value != 0)
        .map_or(1, |value| usize::try_from(value).unwrap_or(0))
    };

    table.elements_with_tags(&["TR"]).iter().fold(
      (0, 0),
      |(rows, columns), row| {
        let row_columns = row
          .elements_with_tags(&["TD"])
          .iter()
          .map(|cell| span(cell, "colspan"))
          .sum::<usize>();

        (rows + span(row, "rowspan"), columns.max(row_columns))
      },
    )
  }
}

pub(crate) fn parse_int(value: &str) -> Option<i64> {
  let value = value.trim_start();

  let (sign, digits) = match value.strip_prefix('-') {
    Some(rest) => (-1, rest),
    None => (1, value.strip_prefix('+').unwrap_or(value)),
  };

  let end = digits
    .find(|c: char| !c.is_ascii_digit())
    .unwrap_or(digits.len());

  digits[..end].parse::<i64>().ok().map(|value| sign * value)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[track_caller]
  fn is_data_table(html: &str) -> bool {
    let document = dom_query::Document::from(html);
    MarkDataTables::is_data_table(&document.select("table").nodes()[0])
  }

  #[test]
  fn data_tables() {
    #[track_caller]
    fn case(html: &str, expected: bool) {
      assert_eq!(is_data_table(html), expected);
    }

    let row = format!("<tr>{}</tr>", "<td>foo</td>".repeat(3));

    case(r#"<table role="presentation"><th>foo</th></table>"#, false);
    case(r#"<table datatable="0"><th>foo</th></table>"#, false);
    case(r#"<table summary="foo"></table>"#, true);
    case("<table><caption>foo</caption></table>", true);
    case("<table><thead></thead></table>", true);
    case("<table><tr><td><table></table></td></tr></table>", false);
    case(&format!("<table>{row}</table>"), false);
    case(&format!("<table>{}</table>", row.repeat(4)), true);
    case(&format!("<table>{}</table>", row.repeat(3)), false);
    case(
      r#"<table><tr><td colspan="5">foo</td></tr><tr><td>bar</td></tr></table>"#,
      true,
    );
  }

  #[test]
  fn parse_int() {
    #[track_caller]
    fn case(value: &str, expected: Option<i64>) {
      assert_eq!(super::parse_int(value), expected);
    }

    case(" 12px", Some(12));
    case("-3", Some(-3));
    case("foo", None);
  }
}
