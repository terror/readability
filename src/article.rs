use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
  /// Author name or attribution string when available.
  pub byline: Option<String>,
  /// HTML markup representing the extracted article content.
  pub content: String,
  /// Text direction from the nearest `dir` attribute on the article's
  /// ancestors.
  pub dir: Option<String>,
  /// Summary or first paragraph of the article.
  pub excerpt: Option<String>,
  /// Language hint discovered during parsing.
  pub lang: Option<String>,
  /// Length of the plain-text content in UTF-16 code units.
  pub length: usize,
  /// Publication timestamp for the article if present.
  pub published_time: Option<String>,
  /// Name of the website that published the article.
  pub site_name: Option<String>,
  /// Plain-text version of the extracted content.
  pub text_content: String,
  /// Human-readable title resolved for the article.
  pub title: String,
}
