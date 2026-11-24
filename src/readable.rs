use super::*;

const MIN_CONTENT_LENGTH: usize = 140;

const MIN_SCORE: f64 = 20.0;

#[allow(clippy::cast_precision_loss)]
pub(crate) fn is_readable(document: &dom_query::Document) -> bool {
  let mut seen = HashSet::new();

  let score = document
    .select("p, pre, article")
    .nodes()
    .iter()
    .copied()
    .chain(
      document
        .select("div > br")
        .nodes()
        .iter()
        .filter_map(NodeRef::parent),
    )
    .filter(|node| seen.insert(node.id))
    .filter(|node| {
      is_visible(node) && !is_unlikely(node) && !is_list_paragraph(node)
    })
    .map(|node| text_length(node.text().trim()))
    .filter(|length| *length >= MIN_CONTENT_LENGTH)
    .map(|length| ((length - MIN_CONTENT_LENGTH) as f64).sqrt())
    .sum::<f64>();

  score > MIN_SCORE
}

fn is_list_paragraph(node: &NodeRef) -> bool {
  node.tag() == "P" && node.has_ancestor_tag("LI", None, |_| true)
}

fn is_unlikely(node: &NodeRef) -> bool {
  let match_string = node.match_string();

  UNLIKELY_CANDIDATE.is_match(&match_string)
    && !MAYBE_CANDIDATE.is_match(&match_string)
}

fn is_visible(node: &NodeRef) -> bool {
  let display_none = node
    .attr("style")
    .is_some_and(|style| DISPLAY_NONE.is_match(&style));

  let aria_hidden = node
    .attr("aria-hidden")
    .is_some_and(|value| value.as_ref() == "true")
    && !node
      .attr("class")
      .is_some_and(|class| class.contains("fallback-image"));

  !display_none && !node.has_attr("hidden") && !aria_hidden
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn readable() {
    #[track_caller]
    fn case(html: &str, expected: bool) {
      assert_eq!(is_readable(&dom_query::Document::from(html)), expected);
    }

    let paragraph =
      |repeat: usize| format!("<p>{}</p>", "foo bar ".repeat(repeat));

    case(&paragraph(10), false);
    case(&paragraph(100), true);
    case(
      &format!("<div>{}<br>{}</div>", "foo ".repeat(150), "bar"),
      true,
    );
    case(
      &format!(r#"<div class="sidebar">{}</div>"#, paragraph(100)),
      true,
    );
    case(
      &paragraph(100).replace("<p>", r#"<p class="sidebar">"#),
      false,
    );
    case(&paragraph(100).replace("<p>", "<p hidden>"), false);
    case(
      &paragraph(100).replace("<p>", r#"<p style="display: none">"#),
      false,
    );
    case(&format!("<ul><li>{}</li></ul>", paragraph(100)), false);
  }
}
