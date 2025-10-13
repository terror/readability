use super::*;

mod article;
mod element_limit;
mod language;
mod metadata;
mod sanitization;

pub use {
  article::ArticleStage, element_limit::ElementLimitStage,
  language::LanguageStage, metadata::MetadataStage,
  sanitization::SanitizationStage,
};

pub(crate) trait Stage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()>;
}
