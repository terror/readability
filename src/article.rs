use super::*;

/// The extracted article content produced by the readability parser.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
  /// Human-readable title resolved for the article.
  pub title: String,
  /// Author name or attribution string when available.
  pub byline: Option<String>,
  /// Text direction hint sourced from the document.
  pub dir: Option<String>,
  /// Language hint discovered during parsing.
  pub lang: Option<String>,
  /// HTML markup representing the extracted article content.
  pub content: String,
  /// Plain-text version of the extracted content.
  pub text_content: String,
  /// Character count of the plain-text content.
  pub length: usize,
  /// Summary or first paragraph of the article.
  pub excerpt: Option<String>,
  /// Name of the website that published the article.
  pub site_name: Option<String>,
  /// Publication timestamp for the article if present.
  pub published_time: Option<String>,
}
