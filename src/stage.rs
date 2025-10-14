use super::*;

mod article;
mod element_limit;
mod language;
mod metadata;
mod postprocess;
mod sanitization;

pub use {
  article::ArticleStage, element_limit::ElementLimitStage,
  language::LanguageStage, metadata::MetadataStage,
  postprocess::PostProcessStage, sanitization::SanitizationStage,
};

pub(crate) trait Stage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()>;
}
