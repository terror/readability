use super::*;

const HEADING_TAGS: &[&str] = &["H1", "H2", "H3", "H4", "H5", "H6"];

const TEXTISH_TAGS: &[&str] = &[
  "SPAN",
  "LI",
  "TD",
  "BLOCKQUOTE",
  "DL",
  "DIV",
  "IMG",
  "OL",
  "P",
  "PRE",
  "TABLE",
  "UL",
];

pub(crate) struct CleanConditionally(pub(crate) &'static str);

impl CleanConditionally {
  #[allow(clippy::cast_precision_loss)]
  fn should_remove(&self, context: &Context<'_>, node: &NodeRef) -> bool {
    let is_data_table =
      |table: &NodeRef| context.extraction.data_tables.contains(&table.id);

    let text = node.text();

    let content_length = node.inner_text_length();

    let is_list = matches!(self.0, "UL" | "OL") || {
      let list_length = node
        .elements_with_tags(&["UL", "OL"])
        .iter()
        .map(NodeExt::inner_text_length)
        .sum::<usize>();

      if content_length == 0 {
        list_length > 0
      } else {
        list_length as f64 / content_length as f64 > 0.9
      }
    };

    if self.0 == "TABLE" && is_data_table(node) {
      return false;
    }

    if node.has_ancestor_tag("TABLE", None, is_data_table) {
      return false;
    }

    if node.has_ancestor_tag("CODE", Some(3), |_| true) {
      return false;
    }

    if node
      .elements_with_tags(&["TABLE"])
      .iter()
      .any(is_data_table)
    {
      return false;
    }

    let weight = if context.flags.weight_classes {
      node.class_weight()
    } else {
      0.0
    };

    if weight < 0.0 {
      return true;
    }

    if text.matches(',').count() >= 10 {
      return false;
    }

    let count = |tag: &str| node.elements_with_tags(&[tag]).len();

    let p = count("P") as f64;
    let img = count("IMG");
    let li = count("LI") as f64 - 100.0;
    let input = count("INPUT");

    let heading_density = Self::text_density(node, HEADING_TAGS);

    let embeds = node.elements_with_tags(&["OBJECT", "EMBED", "IFRAME"]);

    if embeds.iter().any(is_allowed_video) {
      return false;
    }

    let embed_count = embeds.len();

    if AD_WORDS.is_match(text.trim()) || LOADING_WORDS.is_match(text.trim()) {
      return true;
    }

    let link_density = node.link_density();

    let link_density_modifier = context.options.link_density_bias;

    let text_density = Self::text_density(node, TEXTISH_TAGS);

    let is_figure_child = node.has_ancestor_tag("FIGURE", Some(3), |_| true);

    let img_count = img as f64;

    let remove = (!is_figure_child && img > 1 && p / img_count < 0.5)
      || (!is_list && li > p)
      || (input as f64 > (p / 3.0).floor())
      || (!is_list
        && !is_figure_child
        && heading_density < 0.9
        && content_length < 25
        && (img == 0 || img > 2)
        && link_density > 0.0)
      || (!is_list
        && weight < 25.0
        && link_density > 0.2 + link_density_modifier)
      || (weight >= 25.0 && link_density > 0.5 + link_density_modifier)
      || ((embed_count == 1 && content_length < 75) || embed_count > 1)
      || (img == 0 && text_density == 0.0);

    if is_list && remove {
      if node
        .element_children()
        .iter()
        .any(|child| child.element_children().len() > 1)
      {
        return true;
      }

      if img == count("LI") {
        return false;
      }
    }

    remove
  }

  #[allow(clippy::cast_precision_loss)]
  fn text_density(node: &NodeRef, tags: &[&str]) -> f64 {
    let length = node.inner_text_length();

    if length == 0 {
      return 0.0;
    }

    let children_length = node
      .elements_with_tags(tags)
      .iter()
      .map(NodeExt::inner_text_length)
      .sum::<usize>();

    children_length as f64 / length as f64
  }
}

impl Stage for CleanConditionally {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    if !context.flags.clean_conditionally {
      return Ok(());
    }

    let Some(article) = context.extraction.article else {
      return Ok(());
    };

    for node in context
      .node(article)
      .elements_with_tags(&[self.0])
      .iter()
      .rev()
    {
      if node.parent().is_some() && self.should_remove(context, node) {
        node.remove_from_parent();
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[track_caller]
  fn case(tag: &'static str, html: &str, expected: &str) {
    Test::new()
      .stage(CleanConditionally(tag))
      .article()
      .document(&format!("<html><body>{html}</body></html>"))
      .expected_html(&format!(
        "<html><head></head><body>{expected}</body></html>"
      ))
      .run();
  }

  #[test]
  fn keeps_comma_heavy_div() {
    let text = "foo, ".repeat(10);
    case(
      "DIV",
      &format!(r#"<div><a href="/">{text}</a></div>"#),
      &format!(r#"<div><a href="/">{text}</a></div>"#),
    );
  }

  #[test]
  fn keeps_content_div() {
    case(
      "DIV",
      "<div><p>foo bar baz qux quux corge grault garply</p></div>",
      "<div><p>foo bar baz qux quux corge grault garply</p></div>",
    );
  }

  #[test]
  fn keeps_image_lists() {
    case(
      "UL",
      r#"<ul><li><img src="foo"></li><li><img src="bar"></li></ul>"#,
      r#"<ul><li><img src="foo"></li><li><img src="bar"></li></ul>"#,
    );
  }

  #[test]
  fn removes_ad_words() {
    case("DIV", "<div><p>Advertisement</p></div>", "");
  }

  #[test]
  fn removes_linky_div() {
    case(
      "DIV",
      r#"<div><p><a href="/">foo bar baz</a> qux</p></div>"#,
      "",
    );
  }

  #[test]
  fn removes_negative_weight() {
    case("DIV", r#"<div class="sidebar"><p>foo</p></div>"#, "");
  }

  #[test]
  fn skips_when_flag_disabled() {
    Test::new()
      .stage(CleanConditionally("DIV"))
      .article()
      .flags(Flags {
        clean_conditionally: false,
        ..Flags::default()
      })
      .document(r#"<html><body><div class="sidebar">foo</div></body></html>"#)
      .expected_html(r#"<html><head></head><body><div class="sidebar">foo</div></body></html>"#)
      .run();
  }
}
