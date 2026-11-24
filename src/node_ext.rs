use super::*;

const DIV_TO_P_ELEMS: &[&str] = &[
  "BLOCKQUOTE",
  "DIV",
  "DL",
  "IMG",
  "OL",
  "P",
  "PRE",
  "TABLE",
  "UL",
];

const PHRASING_ELEMS: &[&str] = &[
  "ABBR", "AUDIO", "B", "BDO", "BR", "BUTTON", "CITE", "CODE", "DATA",
  "DATALIST", "DFN", "EM", "EMBED", "I", "IMG", "INPUT", "KBD", "LABEL",
  "MARK", "MATH", "METER", "NOSCRIPT", "OBJECT", "OUTPUT", "PROGRESS", "Q",
  "RUBY", "SAMP", "SCRIPT", "SELECT", "SMALL", "SPAN", "STRONG", "SUB", "SUP",
  "TEXTAREA", "TIME", "VAR", "WBR",
];

pub(crate) fn create_element<'a>(tree: &'a Tree, name: &str) -> NodeRef<'a> {
  let element = tree.new_element(name);

  let namespace = tree.root().first_element_child().and_then(|root| {
    root
      .query(|node| node.as_element().map(|element| element.name.ns.clone()))
      .flatten()
  });

  if let Some(namespace) = namespace {
    element.update(|node| {
      if let Some(element) = node.as_element_mut() {
        element.name.ns = namespace;
      }
    });
  }

  element
}

#[allow(clippy::cast_precision_loss)]
pub(crate) fn link_density(
  length: usize,
  links: usize,
  hash_links: usize,
) -> f64 {
  if length == 0 {
    return 0.0;
  }

  (links as f64 + hash_links as f64 * 0.3) / length as f64
}

pub(crate) fn text_length(text: &str) -> usize {
  text.encode_utf16().count()
}

pub(crate) trait NodeExt {
  fn class_weight(&self) -> f64;

  fn elements_with_tags(&self, tags: &[&str]) -> Vec<Self>
  where
    Self: Sized;

  fn has_ancestor_tag(
    &self,
    tag: &str,
    max_depth: Option<usize>,
    filter: impl Fn(&Self) -> bool,
  ) -> bool
  where
    Self: Sized;

  fn has_any_tag(&self, tags: &[&str]) -> bool;

  fn has_child_block_element(&self) -> bool;

  fn has_single_tag_inside_element(&self, tag: &str) -> bool;

  fn has_tag(&self, tag: &str) -> bool;

  fn initial_score(&self, weight_classes: bool) -> f64;

  fn inner_text(&self) -> String;

  fn inner_text_length(&self) -> usize;

  fn is_element_without_content(&self) -> bool;

  fn is_hash_link(&self) -> bool;

  fn is_phrasing_content(&self) -> bool;

  fn is_whitespace(&self) -> bool;

  fn link_density(&self) -> f64;

  fn match_string(&self) -> String;

  fn set_tag(&self, name: &str);

  fn tag(&self) -> String;
}

impl NodeExt for NodeRef<'_> {
  fn class_weight(&self) -> f64 {
    ["class", "id"]
      .iter()
      .filter_map(|name| self.attr(name))
      .filter(|value| !value.is_empty())
      .map(|value| {
        let negative = if NEGATIVE.is_match(&value) {
          -25.0
        } else {
          0.0
        };
        let positive = if POSITIVE.is_match(&value) { 25.0 } else { 0.0 };
        negative + positive
      })
      .sum()
  }

  fn elements_with_tags(&self, tags: &[&str]) -> Vec<Self> {
    self
      .descendants_it()
      .filter(|node| node.has_any_tag(tags))
      .collect()
  }

  fn has_ancestor_tag(
    &self,
    tag: &str,
    max_depth: Option<usize>,
    filter: impl Fn(&Self) -> bool,
  ) -> bool {
    self
      .ancestors_it(None)
      .enumerate()
      .take_while(|(depth, _)| max_depth.is_none_or(|max| *depth <= max))
      .any(|(_, ancestor)| ancestor.has_tag(tag) && filter(&ancestor))
  }

  fn has_any_tag(&self, tags: &[&str]) -> bool {
    self.element_ref().is_some_and(|element| {
      tags
        .iter()
        .any(|tag| element.name.local.as_ref().eq_ignore_ascii_case(tag))
    })
  }

  fn has_child_block_element(&self) -> bool {
    self.children_it(false).any(|child| {
      child.has_any_tag(DIV_TO_P_ELEMS) || child.has_child_block_element()
    })
  }

  fn has_single_tag_inside_element(&self, tag: &str) -> bool {
    let children = self.element_children();

    if children.len() != 1 || !children[0].has_tag(tag) {
      return false;
    }

    !self
      .children()
      .iter()
      .any(|child| child.is_text() && HAS_CONTENT.is_match(&child.text()))
  }

  fn has_tag(&self, tag: &str) -> bool {
    self.has_any_tag(&[tag])
  }

  fn initial_score(&self, weight_classes: bool) -> f64 {
    let score = match self.tag().as_str() {
      "DIV" => 5.0,
      "PRE" | "TD" | "BLOCKQUOTE" => 3.0,
      "ADDRESS" | "OL" | "UL" | "DL" | "DD" | "DT" | "LI" | "FORM" => -3.0,
      "H1" | "H2" | "H3" | "H4" | "H5" | "H6" | "TH" => -5.0,
      _ => 0.0,
    };

    if weight_classes {
      score + self.class_weight()
    } else {
      score
    }
  }

  fn inner_text(&self) -> String {
    NORMALIZE_WHITESPACE
      .replace_all(self.text().trim(), " ")
      .to_string()
  }

  fn inner_text_length(&self) -> usize {
    iter::once(*self)
      .chain(self.descendants_it())
      .fold(TextLength::default(), |length, node| {
        length + TextLength::from(&node)
      })
      .length
  }

  fn is_element_without_content(&self) -> bool {
    if !self.is_element()
      || iter::once(*self)
        .chain(self.descendants_it())
        .any(|node| node.is_nonempty_text())
    {
      return false;
    }

    let children = self.element_children().len();

    children == 0 || children == self.elements_with_tags(&["BR", "HR"]).len()
  }

  fn is_hash_link(&self) -> bool {
    self
      .attr("href")
      .is_some_and(|href| HASH_URL.is_match(&href))
  }

  fn is_phrasing_content(&self) -> bool {
    if self.is_text() {
      return true;
    }

    self.has_any_tag(PHRASING_ELEMS)
      || (self.has_any_tag(&["A", "DEL", "INS"])
        && self
          .children_it(false)
          .all(|child| child.is_phrasing_content()))
  }

  fn is_whitespace(&self) -> bool {
    (self.is_text() && self.text().trim().is_empty()) || self.has_tag("BR")
  }

  fn link_density(&self) -> f64 {
    let (links, hash_links) = self
      .descendants_it()
      .filter(|node| node.has_tag("A"))
      .fold((0, 0), |(links, hash_links), link| {
        let length = link.inner_text_length();

        if link.is_hash_link() {
          (links, hash_links + length)
        } else {
          (links + length, hash_links)
        }
      });

    link_density(self.inner_text_length(), links, hash_links)
  }

  fn match_string(&self) -> String {
    let class = self.attr("class").unwrap_or_default();
    let id = self.attr("id").unwrap_or_default();
    format!("{class} {id}")
  }

  fn set_tag(&self, name: &str) {
    self.update(|node| {
      if let Some(element) = node.as_element_mut() {
        element.name.local = name.into();
      }
    });
  }

  fn tag(&self) -> String {
    if self.is_element() {
      self.node_name().map_or_default(|name| name.to_uppercase())
    } else {
      String::new()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn class_weight() {
    #[track_caller]
    fn case(html: &str, expected: f64) {
      let document = dom_query::Document::from(html);
      let node = document.select("div").nodes()[0];
      assert_eq!(node.class_weight(), expected);
    }

    case(r#"<div class="content">foo</div>"#, 25.0);
    case(r#"<div class="sidebar">foo</div>"#, -25.0);
    case(r#"<div id="sidebar" class="content">foo</div>"#, 0.0);
    case(r#"<div class="post-comment">foo</div>"#, 0.0);
    case("<div>foo</div>", 0.0);
  }

  #[test]
  fn create_element() {
    let document = dom_query::Document::from("<html><body></body></html>");

    let body = document.select("body").nodes()[0];

    body.append_child(&super::create_element(&document.tree, "img"));

    assert_eq!(body.html().to_string(), "<body><img></body>");
  }

  #[test]
  fn has_ancestor_tag() {
    #[track_caller]
    fn case(html: &str, max_depth: Option<usize>, expected: bool) {
      let document = dom_query::Document::from(html);
      let node = document.select("span").nodes()[0];
      assert_eq!(
        node.has_ancestor_tag("TABLE", max_depth, |_| true),
        expected
      );
    }

    let deep = "<table><tr><td><div><div><div><span></span></div></div></div></td></tr></table>";

    case(deep, Some(3), false);
    case(deep, None, true);
    case(
      "<table><tr><td><span></span></td></tr></table>",
      Some(3),
      true,
    );
  }

  #[test]
  fn has_any_tag() {
    #[track_caller]
    fn case(html: &str, tags: &[&str], expected: bool) {
      let document = dom_query::Document::from(html);
      let node = document.select("div").nodes()[0];
      assert_eq!(node.has_any_tag(tags), expected);
    }

    case("<div></div>", &["P", "DIV"], true);
    case("<div></div>", &["P", "SPAN"], false);
    case("<div></div>", &[], false);
  }

  #[test]
  fn has_child_block_element() {
    #[track_caller]
    fn case(html: &str, expected: bool) {
      let document = dom_query::Document::from(html);
      let node = document.select("div").nodes()[0];
      assert_eq!(node.has_child_block_element(), expected);
    }

    case("<div><span><img></span></div>", true);
    case("<div><span>foo</span></div>", false);
  }

  #[test]
  fn has_single_tag_inside_element() {
    #[track_caller]
    fn case(html: &str, expected: bool) {
      let document = dom_query::Document::from(html);
      let node = document.select("div").nodes()[0];
      assert_eq!(node.has_single_tag_inside_element("P"), expected);
    }

    case("<div> <p>foo</p> </div>", true);
    case("<div>bar<p>foo</p></div>", false);
    case("<div><p>foo</p><p>bar</p></div>", false);
    case("<div><span>foo</span></div>", false);
  }

  #[test]
  fn has_tag() {
    #[track_caller]
    fn case(html: &str, selector: &str, tag: &str, expected: bool) {
      let document = dom_query::Document::from(html);
      let node = document.select(selector).nodes()[0];
      assert_eq!(node.has_tag(tag), expected);
    }

    case("<div></div>", "div", "DIV", true);
    case("<div></div>", "div", "div", true);
    case("<div></div>", "div", "P", false);
    case("<div>foo</div>", "div", "", false);
  }

  #[test]
  fn initial_score() {
    #[track_caller]
    fn case(html: &str, selector: &str, weight_classes: bool, expected: f64) {
      let document = dom_query::Document::from(html);
      let node = document.select(selector).nodes()[0];
      assert_eq!(node.initial_score(weight_classes), expected);
    }

    case(r#"<div class="content"></div>"#, "div", true, 30.0);
    case(r#"<div class="content"></div>"#, "div", false, 5.0);
    case("<pre></pre>", "pre", true, 3.0);
    case("<ul></ul>", "ul", true, -3.0);
    case("<h1></h1>", "h1", true, -5.0);
    case("<span></span>", "span", true, 0.0);
  }

  #[test]
  fn inner_text() {
    let document = dom_query::Document::from("<p>  foo \n\n bar  </p>");
    let node = document.select("p").nodes()[0];
    assert_eq!(node.inner_text(), "foo bar");
  }

  #[test]
  fn inner_text_length() {
    #[track_caller]
    fn case(html: &str) {
      let document = dom_query::Document::from(html);
      let node = document.select("div").nodes()[0];
      assert_eq!(node.inner_text_length(), text_length(&node.inner_text()));
    }

    case("<div></div>");
    case("<div> \n </div>");
    case("<div>  foo \n\n bar  </div>");
    case("<div>foo<b> </b>bar</div>");
    case("<div>foo<b>  </b> bar</div>");
    case("<div> <p>foo</p>\n<p>\u{1F600} bar\u{a0}</p> </div>");
  }

  #[test]
  fn is_element_without_content() {
    #[track_caller]
    fn case(html: &str, expected: bool) {
      let document = dom_query::Document::from(html);
      let node = document.select("div").nodes()[0];
      assert_eq!(node.is_element_without_content(), expected);
    }

    case("<div> </div>", true);
    case("<div><br><span></span></div>", false);
    case("<div><span><br></span></div>", true);
    case("<div><img></div>", false);
    case("<div>foo</div>", false);
  }

  #[test]
  fn is_phrasing_content() {
    #[track_caller]
    fn case(html: &str, selector: &str, expected: bool) {
      let document = dom_query::Document::from(html);
      let node = document.select(selector).nodes()[0];
      assert_eq!(node.is_phrasing_content(), expected);
    }

    case("<span>foo</span>", "span", true);
    case("<a><b>foo</b></a>", "a", true);
    case("<a><div>foo</div></a>", "a", false);
    case("<div>foo</div>", "div", false);
  }

  #[test]
  fn link_density() {
    #[track_caller]
    fn case(html: &str, expected: f64) {
      let document = dom_query::Document::from(html);
      let node = document.select("div").nodes()[0];
      assert_eq!(node.link_density(), expected);
    }

    case(r#"<div><a href="/">foo</a>bar</div>"#, 0.5);
    case(r##"<div><a href="#x">foo</a>bar</div>"##, 0.15);
    case("<div></div>", 0.0);
    case(r#"<div>foo</div><a href="/">bar</a>"#, 0.0);
  }

  #[test]
  fn match_string() {
    let document =
      dom_query::Document::from(r#"<div class="foo" id="bar"></div><p></p>"#);

    let div = document.select("div").nodes()[0];
    let paragraph = document.select("p").nodes()[0];

    assert_eq!(div.match_string(), "foo bar");
    assert_eq!(paragraph.match_string(), " ");
  }

  #[test]
  fn set_tag() {
    let document = dom_query::Document::from(
      r#"<html><body><div class="foo">bar</div></body></html>"#,
    );

    let node = document.select("div").nodes()[0];

    node.set_tag("p");

    assert_eq!(document.select("p.foo").length(), 1);
  }
}
