#![allow(clippy::missing_errors_doc)]

use {
  serde::{Deserialize, Serialize},
  unclutter::{Parser, ParserOptions},
  wasm_bindgen::prelude::*,
};

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct Article {
  byline: Option<String>,
  content: String,
  dir: Option<String>,
  excerpt: Option<String>,
  lang: Option<String>,
  length: usize,
  published_time: Option<String>,
  site_name: Option<String>,
  text_content: String,
  title: String,
}

impl From<unclutter::Article> for Article {
  fn from(article: unclutter::Article) -> Self {
    Self {
      byline: article.byline,
      content: article.content,
      dir: article.dir,
      excerpt: article.excerpt,
      lang: article.lang,
      length: article.length,
      published_time: article.published_time,
      site_name: article.site_name,
      text_content: article.text_content,
      title: article.title,
    }
  }
}

#[derive(Debug, Default, Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "camelCase")]
struct Options {
  classes_to_preserve: Vec<String>,
  keep_classes: Option<bool>,
  link_density_bias: Option<f64>,
  max_elements: Option<usize>,
  min_text_length: Option<usize>,
  n_top_candidates: Option<usize>,
  use_json_ld: Option<bool>,
}

impl From<Options> for ParserOptions {
  fn from(options: Options) -> Self {
    let defaults = ParserOptions::default();

    ParserOptions::builder()
      .classes_to_preserve(options.classes_to_preserve)
      .keep_classes(options.keep_classes.unwrap_or(defaults.keep_classes))
      .link_density_bias(
        options
          .link_density_bias
          .unwrap_or(defaults.link_density_bias),
      )
      .max_elements(options.max_elements)
      .min_text_length(
        options.min_text_length.unwrap_or(defaults.min_text_length),
      )
      .n_top_candidates(
        options
          .n_top_candidates
          .unwrap_or(defaults.n_top_candidates),
      )
      .use_json_ld(options.use_json_ld.unwrap_or(defaults.use_json_ld))
      .build()
  }
}

#[wasm_bindgen(js_name = isReadable)]
#[must_use]
pub fn is_readable(html: &str) -> bool {
  Parser::new(html, None, ParserOptions::default())
    .is_ok_and(|parser| parser.is_readable())
}

#[wasm_bindgen]
pub fn parse(
  html: &str,
  base_url: Option<String>,
  options: JsValue,
) -> Result<JsValue, JsError> {
  let options = if options.is_undefined() || options.is_null() {
    Options::default()
  } else {
    serde_wasm_bindgen::from_value::<Options>(options)?
  };

  let article =
    Parser::new(html, base_url.as_deref(), options.into())?.parse()?;

  Ok(serde_wasm_bindgen::to_value(&Article::from(article))?)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn options() {
    let options = ParserOptions::from(
      serde_json::from_value::<Options>(serde_json::json!({
        "classesToPreserve": ["foo"],
        "keepClasses": true,
        "linkDensityBias": 0.5,
        "maxElements": 1,
        "minTextLength": 2,
        "nTopCandidates": 3,
        "useJsonLd": false,
      }))
      .unwrap(),
    );

    assert_eq!(options.classes_to_preserve, ["page", "foo"]);
    assert!(options.keep_classes);
    assert_eq!(options.link_density_bias, 0.5);
    assert_eq!(options.max_elements, Some(1));
    assert_eq!(options.min_text_length, 2);
    assert_eq!(options.n_top_candidates, 3);
    assert!(!options.use_json_ld);
  }

  #[test]
  fn article_fields_are_camel_case() {
    assert_eq!(
      serde_json::to_value(Article::from(unclutter::Article {
        byline: Some("foo".into()),
        content: "bar".into(),
        dir: Some("baz".into()),
        excerpt: Some("qux".into()),
        lang: Some("quux".into()),
        length: 1,
        published_time: Some("corge".into()),
        site_name: Some("grault".into()),
        text_content: "garply".into(),
        title: "waldo".into(),
      }))
      .unwrap(),
      serde_json::json!({
        "byline": "foo",
        "content": "bar",
        "dir": "baz",
        "excerpt": "qux",
        "lang": "quux",
        "length": 1,
        "publishedTime": "corge",
        "siteName": "grault",
        "textContent": "garply",
        "title": "waldo",
      }),
    );
  }
}
