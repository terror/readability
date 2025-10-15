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

    let stages: Vec<Box<dyn Stage>> = vec![
      Box::new(ElementLimitStage),
      Box::new(LanguageStage),
      Box::new(MetadataStage),
      Box::new(RemoveDisallowedNodesStage),
      Box::new(RewriteFontTagsStage),
      Box::new(RewriteCenterTagsStage),
      Box::new(RemoveUnlikelyCandidatesStage),
      Box::new(ReplaceBreakSequencesStage),
      Box::new(NormalizeContainersStage),
      Box::new(ArticleStage),
      Box::new(NormalizeArticleRootStage),
      Box::new(NormalizeArticleHeadingsStage),
      Box::new(FlattenSimpleTablesStage),
      Box::new(RemoveNonContentElementsStage),
      Box::new(StripPresentationalAttributesStage),
      Box::new(FixRelativeUrisStage::new(base_url)),
      Box::new(CleanClassAttributesStage),
      Box::new(EnforceVoidSelfClosingStage),
    ];

    for stage in stages {
      pipeline.add_stage(stage);
    }

    pipeline
  }
}
