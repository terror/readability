use super::*;

mod element_limit;
mod excerpt_fallback;
mod extract_byline;
mod extract_dir;
mod extract_json_ld;
mod extract_lang;
mod extract_meta_tags;
mod remove_disallowed_nodes;
mod rewrite_font_tags;
mod rewrite_line_breaks;
mod unescape_html_entities;
mod unwrap_noscript_images;

#[cfg(test)]
mod test;

pub(crate) use {
  element_limit::ElementLimit, excerpt_fallback::ExcerptFallback,
  extract_byline::ExtractByline, extract_dir::ExtractDir,
  extract_json_ld::ExtractJsonLd, extract_lang::ExtractLang,
  extract_meta_tags::ExtractMetaTags,
  remove_disallowed_nodes::RemoveDisallowedNodes,
  rewrite_font_tags::RewriteFontTags, rewrite_line_breaks::RewriteLineBreaks,
  unescape_html_entities::UnescapeHtmlEntities,
  unwrap_noscript_images::UnwrapNoscriptImages,
};

#[cfg(test)]
pub(crate) use test::Test;

pub(crate) trait Stage {
  fn run(&mut self, context: &mut Context<'_>) -> Result;
}
