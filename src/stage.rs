use super::*;

mod article;
mod element_limit;
mod language;
mod metadata;
mod normalize_containers;
mod postprocess;
mod remove_disallowed_nodes;
mod remove_unlikely_candidates;
mod replace_break_sequences;
mod rewrite_font_tags;

pub use {
  article::ArticleStage,
  element_limit::ElementLimitStage,
  language::LanguageStage,
  metadata::MetadataStage,
  normalize_containers::NormalizeContainersStage,
  postprocess::{
    CleanClassAttributesStage, FinalizeArticleMarkupStage,
    FixRelativeUrisStage, NormalizeArticleWhitespaceStage,
  },
  remove_disallowed_nodes::RemoveDisallowedNodesStage,
  remove_unlikely_candidates::RemoveUnlikelyCandidatesStage,
  replace_break_sequences::ReplaceBreakSequencesStage,
  rewrite_font_tags::RewriteFontTagsStage,
};

pub(crate) trait Stage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()>;
}
