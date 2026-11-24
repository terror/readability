#![allow(clippy::missing_errors_doc)]

use {
  serde::Serialize,
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

#[wasm_bindgen(js_name = isProbablyReaderable)]
pub fn is_probably_readerable(html: &str) -> Result<bool, JsError> {
  Ok(
    Parser::new(html, None, ParserOptions::default())?.is_probably_readerable(),
  )
}

#[wasm_bindgen]
pub fn parse(html: &str, base_url: Option<String>) -> Result<JsValue, JsError> {
  let article =
    Parser::new(html, base_url.as_deref(), ParserOptions::default())?
      .parse()?;

  Ok(serde_wasm_bindgen::to_value(&Article::from(article))?)
}

#[cfg(test)]
mod tests {
  use super::*;

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
