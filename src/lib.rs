use {
  context::Context,
  document::Document,
  dom_query::{NodeRef, Selection},
  metadata::Metadata,
  pipeline::Pipeline,
  regex::Regex,
  serde::{Deserialize, Serialize},
  stage::{
    ElementLimit, ExtractJsonLd, ExtractMetaTags, RemoveDisallowedNodes,
    RewriteFontTags, RewriteLineBreaks, Stage, UnwrapNoscriptImages,
  },
  std::{iter, sync::LazyLock},
  title_extractor::TitleExtractor,
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
mod title_extractor;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
