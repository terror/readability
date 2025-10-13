#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("aborting parsing document; {found} elements found (limit: {limit})")]
  ElementLimitExceeded { found: usize, limit: usize },
  #[error("invalid base url: {source}")]
  InvalidBaseUrl {
    #[from]
    source: url::ParseError,
  },
  #[error("invalid selector: {0}")]
  InvalidSelector(String),
  #[error("failed to identify article content")]
  MissingArticleContent,
}
