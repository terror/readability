use {
  context::Context,
  document::Document,
  dom_query::{NodeId, NodeRef, Selection},
  flags::Flags,
  metadata::Metadata,
  node_ext::{NodeExt, text_length},
  pipeline::Pipeline,
  re::{
    BYLINE, COMMAS, HAS_CONTENT, HASH_URL, MAYBE_CANDIDATE, META_PROPERTY,
    NEGATIVE, NUMERIC_HTML_ENTITY, POSITIVE, SENTENCE_END,
    TITLE_HIERARCHICAL_SEPARATOR, TITLE_LEADING_JUNK,
    TITLE_NORMALIZE_WHITESPACE, TITLE_SEPARATOR, TITLE_WHITESPACE,
    TITLE_WORD_SEPARATOR, TOKENIZE, UNLIKELY_CANDIDATE,
  },
  regex::Regex,
  serde::{Deserialize, Serialize},
  stage::{
    ConvertDivsToParagraphs, ElementLimit, ExtractArticle, ExtractByline,
    ExtractDir, ExtractExcerpt, ExtractJsonLd, ExtractLang, ExtractMetaTags,
    ExtractTitle, GrabArticle, RemoveDisallowedNodes,
    RemoveDuplicateTitleHeader, RemoveEmptyContainers, RemoveHiddenNodes,
    RemoveUnlikelyCandidates, RewriteFontTags, RewriteLineBreaks,
    ScoreCandidates, SelectTopCandidate, Stage, UnescapeHtmlEntities,
    UnwrapNoscriptImages, WrapArticlePage,
  },
  std::{collections::HashMap, iter, mem, sync::LazyLock},
  text_similarity::text_similarity,
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
mod flags;
mod metadata;
mod node_ext;
mod options;
mod pipeline;
mod re;
mod readability;
mod stage;
mod text_similarity;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
