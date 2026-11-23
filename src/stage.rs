use super::*;

mod convert_divs_to_paragraphs;
mod element_limit;
mod extract_article;
mod extract_byline;
mod extract_dir;
mod extract_excerpt;
mod extract_json_ld;
mod extract_lang;
mod extract_meta_tags;
mod extract_title;
mod grab_article;
mod remove_disallowed_nodes;
mod remove_duplicate_title_header;
mod remove_empty_containers;
mod remove_hidden_nodes;
mod remove_unlikely_candidates;
mod rewrite_font_tags;
mod rewrite_line_breaks;
mod score_candidates;
mod select_top_candidate;
mod unescape_html_entities;
mod unwrap_noscript_images;
mod wrap_article_page;

#[cfg(test)]
mod test;

pub(crate) use {
  convert_divs_to_paragraphs::ConvertDivsToParagraphs,
  element_limit::ElementLimit, extract_article::ExtractArticle,
  extract_byline::ExtractByline, extract_dir::ExtractDir,
  extract_excerpt::ExtractExcerpt, extract_json_ld::ExtractJsonLd,
  extract_lang::ExtractLang, extract_meta_tags::ExtractMetaTags,
  extract_title::ExtractTitle, grab_article::GrabArticle,
  remove_disallowed_nodes::RemoveDisallowedNodes,
  remove_duplicate_title_header::RemoveDuplicateTitleHeader,
  remove_empty_containers::RemoveEmptyContainers,
  remove_hidden_nodes::RemoveHiddenNodes,
  remove_unlikely_candidates::RemoveUnlikelyCandidates,
  rewrite_font_tags::RewriteFontTags, rewrite_line_breaks::RewriteLineBreaks,
  score_candidates::ScoreCandidates, select_top_candidate::SelectTopCandidate,
  unescape_html_entities::UnescapeHtmlEntities,
  unwrap_noscript_images::UnwrapNoscriptImages,
  wrap_article_page::WrapArticlePage,
};

#[cfg(test)]
pub(crate) use test::Test;

pub(crate) trait Stage {
  fn run(&mut self, context: &mut Context<'_>) -> Result;
}
