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
    _base_url: Option<&'a Url>,
  ) -> Self {
    let mut pipeline = Self::new(context);

    let stages: Vec<Box<dyn Stage>> = vec![Box::new(ElementLimitStage)];

    for stage in stages {
      pipeline.add_stage(stage);
    }

    pipeline
  }
}
