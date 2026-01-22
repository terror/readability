use {
  context::Context,
  document::Document,
  dom_query::{NodeRef, Selection},
  html_escape::encode_double_quoted_attribute_to_string,
  metadata::Metadata,
  pipeline::Pipeline,
  serde::{Deserialize, Serialize},
  stage::{
    ElementLimit, RemoveDisallowedNodes, RewriteFontTags, Stage,
    UnwrapNoscriptImages,
  },
  std::fmt::Write,
  url::Url,
};

pub use crate::{
  article::Article,
  error::Error,
  options::{ReadabilityOptions, ReadabilityOptionsBuilder},
  readability::Readability,
};

mod article;
mod context;
mod document;
mod error;
mod metadata;
mod options;
mod pipeline;
mod readability;
mod stage;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
