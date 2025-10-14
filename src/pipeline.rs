use super::*;

pub(crate) struct Pipeline<'a> {
  context: Context<'a>,
  stages: Vec<Box<dyn Stage + 'a>>,
}

impl<'a> Pipeline<'a> {
  fn add_stage(&mut self, stage: Box<dyn Stage + 'a>) {
    self.stages.push(stage);
  }

  pub(crate) fn new(context: Context<'a>) -> Self {
    Self {
      context,
      stages: Vec::new(),
    }
  }

  pub(crate) fn run(mut self) -> Result<Context<'a>> {
    for stage in &mut self.stages {
      stage.run(&mut self.context)?;
    }

    Ok(self.context)
  }

  pub(crate) fn with_default_stages(
    context: Context<'a>,
    base_url: Option<&'a Url>,
  ) -> Self {
    let mut pipeline = Self::new(context);

    pipeline.add_stage(Box::new(ElementLimitStage));
    pipeline.add_stage(Box::new(RemoveDisallowedNodesStage));
    pipeline.add_stage(Box::new(RewriteFontTagsStage));
    pipeline.add_stage(Box::new(RemoveUnlikelyCandidatesStage));
    pipeline.add_stage(Box::new(ReplaceBreakSequencesStage));
    pipeline.add_stage(Box::new(NormalizeContainersStage));
    pipeline.add_stage(Box::new(LanguageStage));
    pipeline.add_stage(Box::new(MetadataStage));
    pipeline.add_stage(Box::new(ArticleStage));
    pipeline.add_stage(Box::new(FixRelativeUrisStage::new(base_url)));
    pipeline.add_stage(Box::new(CleanClassAttributesStage));
    pipeline.add_stage(Box::new(NormalizeArticleWhitespaceStage));
    pipeline.add_stage(Box::new(FinalizeArticleMarkupStage));

    pipeline
  }
}
