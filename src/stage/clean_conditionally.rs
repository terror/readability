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

#[derive(Default)]
struct Descendants<'a> {
  embeds: Vec<NodeRef<'a>>,
  has_data_table: bool,
  hash_links: usize,
  heading_length: usize,
  img: usize,
  input: usize,
  length: usize,
  li: usize,
  links: usize,
  list_length: usize,
  p: usize,
  textish_length: usize,
}

impl<'a> Descendants<'a> {
  fn add(
    &mut self,
    node: NodeRef<'a>,
    length: usize,
    is_data_table: impl Fn(&NodeRef) -> bool,
  ) {
    let Some(name) =
      node.element_ref().map(|element| element.name.local.clone())
    else {
      return;
    };

    let is = |tags: &[&str]| {
      tags
        .iter()
        .any(|tag| name.as_ref().eq_ignore_ascii_case(tag))
    };

    if is(&["A"]) {
      if node.is_hash_link() {
        self.hash_links += length;
      } else {
        self.links += length;
      }
    } else if is(&["EMBED", "IFRAME", "OBJECT"]) {
      self.embeds.push(node);
    } else if is(&["INPUT"]) {
      self.input += 1;
    } else if is(&["LI"]) {
      self.li += 1;
    }

    if is(&["IMG"]) {
      self.img += 1;
    } else if is(&["P"]) {
      self.p += 1;
    } else if is(&["TABLE"]) {
      self.has_data_table |= is_data_table(&node);
    } else if is(&["OL", "UL"]) {
      self.list_length += length;
    }

    if is(HEADING_TAGS) {
      self.heading_length += length;
    }

    if is(TEXTISH_TAGS) {
      self.textish_length += length;
    }
  }

  fn new(node: &NodeRef<'a>, is_data_table: impl Fn(&NodeRef) -> bool) -> Self {
    let mut descendants = Self::default();

    let mut stack =
      vec![(*node, node.children_it(false), TextLength::from(node))];

    while let Some((_, children, _)) = stack.last_mut() {
      if let Some(child) = children.next() {
        stack.push((child, child.children_it(false), TextLength::from(&child)));
        continue;
      }

      let Some((node, _, length)) = stack.pop() else {
        break;
      };

      let Some((_, _, parent_length)) = stack.last_mut() else {
        descendants.length = length.length;
        break;
      };

      *parent_length = *parent_length + length;

      descendants.add(node, length.length, &is_data_table);
    }

    descendants
  }
}

pub(crate) struct CleanConditionally(pub(crate) &'static str);

impl CleanConditionally {
  #[allow(clippy::cast_precision_loss)]
  fn should_remove(&self, context: &Context<'_>, node: &NodeRef) -> bool {
    let is_data_table =
      |table: &NodeRef| context.extraction.data_tables.contains(&table.id);

    if self.0 == "TABLE" && is_data_table(node) {
      return false;
    }

    if node.has_ancestor_tag("TABLE", None, is_data_table) {
      return false;
    }

    if node.has_ancestor_tag("CODE", Some(3), |_| true) {
      return false;
    }

    let descendants = Descendants::new(node, is_data_table);

    if descendants.has_data_table {
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

    let text = node.text();

    if text.matches(',').count() >= 10 {
      return false;
    }

    if descendants.embeds.iter().any(is_allowed_video) {
      return false;
    }

    if AD_WORDS.is_match(text.trim()) || LOADING_WORDS.is_match(text.trim()) {
      return true;
    }

    let content_length = descendants.length;

    let density = |length: usize| {
      if content_length == 0 {
        0.0
      } else {
        length as f64 / content_length as f64
      }
    };

    let is_list = matches!(self.0, "UL" | "OL")
      || if content_length == 0 {
        descendants.list_length > 0
      } else {
        density(descendants.list_length) > 0.9
      };

    let p = descendants.p as f64;
    let img = descendants.img;
    let li = descendants.li as f64 - 100.0;
    let input = descendants.input;

    let heading_density = density(descendants.heading_length);

    let embed_count = descendants.embeds.len();

    let link_density =
      link_density(content_length, descendants.links, descendants.hash_links);

    let link_density_modifier = context.options.link_density_bias;

    let text_density = density(descendants.textish_length);

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

      if img == descendants.li {
        return false;
      }
    }

    remove
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
