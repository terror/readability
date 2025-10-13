use super::*;

/// The extracted article content produced by the readability parser.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Article {
  pub title: String,
  pub byline: Option<String>,
  pub dir: Option<String>,
  pub lang: Option<String>,
  pub content: String,
  pub text_content: String,
  pub length: usize,
  pub excerpt: Option<String>,
  pub site_name: Option<String>,
  pub published_time: Option<String>,
  pub metadata: ArticleMetadata,
}

impl Article {
  pub fn new(
    title: String,
    byline: Option<String>,
    dir: Option<String>,
    lang: Option<String>,
    content: String,
    text_content: String,
    excerpt: Option<String>,
    site_name: Option<String>,
    published_time: Option<String>,
    metadata: ArticleMetadata,
  ) -> Self {
    let length = text_content.chars().count();
    Self {
      title,
      byline,
      dir,
      lang,
      length,
      content,
      text_content,
      excerpt,
      site_name,
      published_time,
      metadata,
    }
  }
}

/// Metadata captured while parsing an article.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleMetadata {
  pub title: Option<String>,
  pub byline: Option<String>,
  pub excerpt: Option<String>,
  pub site_name: Option<String>,
  pub published_time: Option<String>,
}
