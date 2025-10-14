use super::*;

mod article;
mod clean_class_attributes;
mod element_limit;
mod enforce_void_self_closing;
mod ensure_paragraph_trailing_newline;
mod fix_relative_uris;
mod flatten_simple_tables;
mod language;
mod metadata;
mod normalize_article_headings;
mod normalize_article_root;
mod normalize_article_whitespace;
mod normalize_containers;
mod remove_disallowed_nodes;
mod remove_non_content_elements;
mod remove_unlikely_candidates;
mod replace_break_sequences;
mod rewrite_font_tags;
mod strip_presentational_attributes;

pub use {
  article::ArticleStage, clean_class_attributes::CleanClassAttributesStage,
  element_limit::ElementLimitStage,
  enforce_void_self_closing::EnforceVoidSelfClosingStage,
  ensure_paragraph_trailing_newline::EnsureParagraphTrailingNewlineStage,
  fix_relative_uris::FixRelativeUrisStage,
  flatten_simple_tables::FlattenSimpleTablesStage, language::LanguageStage,
  metadata::MetadataStage,
  normalize_article_headings::NormalizeArticleHeadingsStage,
  normalize_article_root::NormalizeArticleRootStage,
  normalize_article_whitespace::NormalizeArticleWhitespaceStage,
  normalize_containers::NormalizeContainersStage,
  remove_disallowed_nodes::RemoveDisallowedNodesStage,
  remove_non_content_elements::RemoveNonContentElementsStage,
  remove_unlikely_candidates::RemoveUnlikelyCandidatesStage,
  replace_break_sequences::ReplaceBreakSequencesStage,
  rewrite_font_tags::RewriteFontTagsStage,
  strip_presentational_attributes::StripPresentationalAttributesStage,
};

pub(crate) trait Stage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()>;
}
