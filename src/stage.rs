use super::*;

mod article;
mod clean_class_attributes;
mod element_limit;
mod fix_relative_uris;
mod language;
mod metadata;
mod normalize_article_whitespace;
mod normalize_containers;
mod postprocess;
mod remove_disallowed_nodes;
mod remove_unlikely_candidates;
mod replace_break_sequences;
mod rewrite_font_tags;

pub use {
  article::ArticleStage, clean_class_attributes::CleanClassAttributesStage,
  element_limit::ElementLimitStage, fix_relative_uris::FixRelativeUrisStage,
  language::LanguageStage, metadata::MetadataStage,
  normalize_article_whitespace::NormalizeArticleWhitespaceStage,
  normalize_containers::NormalizeContainersStage,
  postprocess::EnforceVoidSelfClosingStage,
  remove_disallowed_nodes::RemoveDisallowedNodesStage,
  remove_unlikely_candidates::RemoveUnlikelyCandidatesStage,
  replace_break_sequences::ReplaceBreakSequencesStage,
  rewrite_font_tags::RewriteFontTagsStage,
};

pub(crate) trait Stage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()>;
}
