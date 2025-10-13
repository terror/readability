use {
  ego_tree::{NodeId, NodeRef},
  once_cell::sync::Lazy,
  regex::Regex,
  scraper::{ElementRef, Html, Node, Selector},
  serde::{Deserialize, Serialize},
  std::{collections::HashMap, ops::Deref},
  url::Url,
};

mod article;
mod error;
mod options;
mod readability;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub use crate::{
  article::Article,
  error::Error,
  options::{ReadabilityOptions, ReadabilityOptionsBuilder},
  readability::Readability,
};
