use {
  context::Context,
  document::Document,
  dom_query::NodeRef,
  metadata::Metadata,
  pipeline::Pipeline,
  serde::{Deserialize, Serialize},
  stage::{ElementLimitStage, RemoveDisallowedNodesStage, Stage},
  std::mem,
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
