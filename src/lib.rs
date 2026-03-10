use {
  context::Context,
  document::Document,
  dom_query::{NodeRef, Selection},
  metadata::Metadata,
  pipeline::Pipeline,
  re::{
    BYLINE, MAYBE_CANDIDATE, META_PROPERTY, NUMERIC_HTML_ENTITY,
    TITLE_HIERARCHICAL_SEPARATOR, TITLE_LEADING_JUNK,
    TITLE_NORMALIZE_WHITESPACE, TITLE_SEPARATOR, UNLIKELY_CANDIDATE,
  },
  regex::Regex,
  serde::{Deserialize, Serialize},
  stage::{
    ElementLimit, ExtractByline, ExtractDir, ExtractExcerpt, ExtractJsonLd,
    ExtractLang, ExtractMetaTags, ExtractTitle, RemoveDisallowedNodes,
    RemoveEmptyContainers, RemoveHiddenNodes, RemoveUnlikelyCandidates,
    RewriteFontTags, RewriteLineBreaks, Stage, UnescapeHtmlEntities,
    UnwrapNoscriptImages,
  },
  std::{collections::HashMap, iter, mem, sync::LazyLock},
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
mod re;
mod readability;
mod stage;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
