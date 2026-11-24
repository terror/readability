use super::*;

mod clean;
mod clean_classes;
mod clean_conditionally;
mod clean_headers;
mod clean_share_elements;
mod clean_styles;
mod element_limit;
mod extract_article;
mod extract_dir;
mod extract_excerpt;
mod extract_json_ld;
mod extract_lang;
mod extract_meta_tags;
mod extract_title;
mod fix_lazy_images;
mod fix_relative_uris;
mod grab_article;
mod mark_data_tables;
mod prepare_nodes;
mod remove_breaks_before_paragraphs;
mod remove_comments;
mod remove_disallowed_nodes;
mod remove_empty_paragraphs;
mod rename_headings;
mod rewrite_font_tags;
mod rewrite_line_breaks;
mod score_candidates;
mod select_top_candidate;
mod simplify_nested_elements;
mod unescape_html_entities;
mod unwrap_noscript_images;
mod unwrap_single_cell_tables;
mod wrap_article_page;

#[cfg(test)]
mod test;

pub(crate) use {
  clean::{Clean, is_allowed_video},
  clean_classes::CleanClasses,
  clean_conditionally::CleanConditionally,
  clean_headers::CleanHeaders,
  clean_share_elements::CleanShareElements,
  clean_styles::CleanStyles,
  element_limit::ElementLimit,
  extract_article::ExtractArticle,
  extract_dir::ExtractDir,
  extract_excerpt::ExtractExcerpt,
  extract_json_ld::ExtractJsonLd,
  extract_lang::ExtractLang,
  extract_meta_tags::ExtractMetaTags,
  extract_title::ExtractTitle,
  fix_lazy_images::FixLazyImages,
  fix_relative_uris::FixRelativeUris,
  grab_article::GrabArticle,
  mark_data_tables::MarkDataTables,
  prepare_nodes::PrepareNodes,
  remove_breaks_before_paragraphs::RemoveBreaksBeforeParagraphs,
  remove_comments::RemoveComments,
  remove_disallowed_nodes::RemoveDisallowedNodes,
  remove_empty_paragraphs::RemoveEmptyParagraphs,
  rename_headings::RenameHeadings,
  rewrite_font_tags::RewriteFontTags,
  rewrite_line_breaks::RewriteLineBreaks,
  score_candidates::ScoreCandidates,
  select_top_candidate::SelectTopCandidate,
  simplify_nested_elements::SimplifyNestedElements,
  unescape_html_entities::UnescapeHtmlEntities,
  unwrap_noscript_images::UnwrapNoscriptImages,
  unwrap_single_cell_tables::UnwrapSingleCellTables,
  wrap_article_page::WrapArticlePage,
};

#[cfg(test)]
pub(crate) use test::Test;

pub(crate) trait Stage {
  fn run(&mut self, context: &mut Context<'_>) -> Result;
}
