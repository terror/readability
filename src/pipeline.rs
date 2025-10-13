use super::*;

pub(crate) struct Pipeline<'a> {
  stages: Vec<Box<dyn Stage + 'a>>,
  context: Context<'a>,
}

impl<'a> Pipeline<'a> {
  pub(crate) fn new(context: Context<'a>) -> Self {
    Self {
      stages: Vec::new(),
      context,
    }
  }

  pub(crate) fn with_default_stages(context: Context<'a>) -> Self {
    let mut pipeline = Self::new(context);

    pipeline.add_stage(Box::new(ElementLimitStage));
    pipeline.add_stage(Box::new(SanitizationStage));
    pipeline.add_stage(Box::new(LanguageStage));
    pipeline.add_stage(Box::new(MetadataStage));
    pipeline.add_stage(Box::new(ArticleStage));

    pipeline
  }

  fn add_stage(&mut self, stage: Box<dyn Stage + 'a>) {
    self.stages.push(stage);
  }

  pub(crate) fn run(mut self) -> Result<Context<'a>> {
    for stage in self.stages.iter_mut() {
      stage.run(&mut self.context)?;
    }

    Ok(self.context)
  }
}
