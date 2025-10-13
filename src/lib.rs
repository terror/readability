mod article;
mod context;
mod document;
mod error;
mod options;
mod pipeline;
mod readability;
mod stage;

use {
  context::{CollectedMetadata, Context},
  document::Document,
  ego_tree::{NodeId, NodeRef},
  html5ever::{LocalName, QualName, ns},
  pipeline::Pipeline,
  regex::Regex,
  scraper::{ElementRef, Html, Node, Selector, node::Element},
  serde::{Deserialize, Serialize},
  stage::{
    ArticleStage, ElementLimitStage, LanguageStage, MetadataStage,
    SanitizationStage, Stage,
  },
  std::{collections::HashMap, ops::Deref, sync::LazyLock},
  url::Url,
};

pub use crate::{
  article::Article,
  error::Error,
  options::{ReadabilityOptions, ReadabilityOptionsBuilder},
  readability::Readability,
};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
