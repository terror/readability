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

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArticleDetails {
  pub byline: Option<String>,
  pub dir: Option<String>,
  pub lang: Option<String>,
  pub excerpt: Option<String>,
  pub site_name: Option<String>,
  pub published_time: Option<String>,
}

impl Article {
  pub fn new(
    title: String,
    content: String,
    text_content: String,
    metadata: ArticleMetadata,
    details: ArticleDetails,
  ) -> Self {
    let ArticleDetails {
      byline,
      dir,
      lang,
      excerpt,
      site_name,
      published_time,
    } = details;

    let length = text_content.chars().count();

    Self {
      title,
      byline,
      dir,
      lang,
      content,
      text_content,
      length,
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
