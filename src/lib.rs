use {
  anyhow::{Context, Result, anyhow},
  ego_tree::{NodeId, NodeRef},
  once_cell::sync::Lazy,
  regex::Regex,
  scraper::{ElementRef, Html, Node, Selector},
  serde::{Deserialize, Serialize},
  std::{collections::HashMap, ops::Deref},
  url::Url,
};

mod article;
mod options;
mod readability;

pub use crate::{
  article::{Article, ArticleDetails, ArticleMetadata},
  options::{ReadabilityOptions, ReadabilityOptionsBuilder},
  readability::{Readability, ReadabilityError},
};
