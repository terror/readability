use {
  context::Context,
  document::Document,
  dom_query::{NodeId, NodeRef, Tree},
  extraction::Extraction,
  flags::Flags,
  metadata::Metadata,
  node_ext::{NodeExt, create_element, text_length},
  pipeline::Pipeline,
  re::{
    AD_WORDS, B64_DATA_URL, BYLINE, COMMAS, DISPLAY_NONE, HAS_CONTENT,
    HASH_URL, IMAGE_EXTENSION, LAZY_IMAGE_SRC, LAZY_IMAGE_SRCSET,
    LOADING_WORDS, MAYBE_CANDIDATE, META_NAME, META_PROPERTY, NEGATIVE,
    NORMALIZE_WHITESPACE, NUMERIC_HTML_ENTITY, POSITIVE, SENTENCE_END,
    SHARE_ELEMENTS, SRCSET_URL, TITLE_HIERARCHICAL_SEPARATOR,
    TITLE_LEADING_JUNK, TITLE_SEPARATOR, TITLE_WHITESPACE,
    TITLE_WORD_SEPARATOR, TOKENIZE, UNLIKELY_CANDIDATE, VIDEOS,
  },
  regex::{Captures, Regex},
  rule::{Action, Rule, traverse},
  serde::{Deserialize, Serialize},
  stage::{
    Clean, CleanClasses, CleanConditionally, CleanHeaders, CleanShareElements,
    CleanStyles, ConvertDivsToParagraphs, ElementLimit, ExtractArticle,
    ExtractByline, ExtractDir, ExtractExcerpt, ExtractJsonLd, ExtractLang,
    ExtractMetaTags, ExtractTitle, FixLazyImages, FixRelativeUris, GrabArticle,
    MarkDataTables, PrepareNodes, RemoveBreaksBeforeParagraphs, RemoveComments,
    RemoveDisallowedNodes, RemoveDuplicateTitleHeader, RemoveEmptyContainers,
    RemoveEmptyParagraphs, RemoveHiddenNodes, RemoveUnlikelyCandidates,
    RenameHeadings, RewriteFontTags, RewriteLineBreaks, ScoreCandidates,
    SelectTopCandidate, SimplifyNestedElements, Stage, UnescapeHtmlEntities,
    UnwrapNoscriptImages, UnwrapSingleCellTables, WrapArticlePage,
  },
  std::{
    collections::{HashMap, HashSet},
    iter, mem,
    sync::LazyLock,
  },
  text_similarity::text_similarity,
  url::Url,
};

pub use crate::{
  article::Article,
  error::Error,
  options::{ParserOptions, ParserOptionsBuilder},
  parser::Parser,
};

mod article;
mod context;
mod document;
mod error;
mod extraction;
mod flags;
mod metadata;
mod node_ext;
mod options;
mod parser;
mod pipeline;
mod re;
mod readerable;
mod rule;
mod stage;
mod text_similarity;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
