mod article;
mod article_fragment;
mod context;
mod document;
mod error;
mod metadata;
mod options;
mod pipeline;
mod readability;
mod serializable_node;
mod stage;

use {
  article_fragment::ArticleFragment,
  context::Context,
  document::Document,
  ego_tree::{NodeId, NodeRef, iter::Edge},
  html5ever::{
    LocalName, QualName, ns,
    serialize::{SerializeOpts, Serializer, TraversalScope, serialize},
  },
  metadata::Metadata,
  pipeline::Pipeline,
  regex::Regex,
  scraper::{ElementRef, Html, Node, Selector, node::Element},
  serde::{Deserialize, Serialize},
  serializable_node::SerializableNode,
  stage::{
    ArticleStage, CleanClassAttributesStage, ElementLimitStage,
    EnforceVoidSelfClosingStage, EnsureParagraphTrailingNewlineStage,
    FixRelativeUrisStage, FlattenSimpleTablesStage, LanguageStage,
    MetadataStage, NormalizeArticleHeadingsStage,
    NormalizeArticleRootStage, NormalizeArticleWhitespaceStage,
    NormalizeContainersStage, RemoveDisallowedNodesStage,
    RemoveNonContentElementsStage, RemoveUnlikelyCandidatesStage,
    ReplaceBreakSequencesStage, RewriteFontTagsStage, Stage,
    StripPresentationalAttributesStage,
  },
  std::{collections::HashMap, io, sync::LazyLock},
  url::Url,
};

pub use crate::{
  article::Article,
  error::Error,
  options::{ReadabilityOptions, ReadabilityOptionsBuilder},
  readability::Readability,
};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
