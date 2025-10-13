#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("aborting parsing document; {found} elements found (limit: {limit})")]
  ElementLimitExceeded { found: usize, limit: usize },
  #[error("invalid base url")]
  InvalidBaseUrl {
    #[from]
    source: url::ParseError,
  },
  #[error("failed to identify article content")]
  MissingArticleContent,
}
