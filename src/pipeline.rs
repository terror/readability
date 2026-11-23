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
    let mut pipeline = Self::new(Context {
      url: base_url.cloned(),
      ..context
    });

    let stages: Vec<Box<dyn Stage>> = vec![
      Box::new(ElementLimit),
      Box::new(RemoveComments),
      Box::new(UnwrapNoscriptImages),
      Box::new(ExtractLang),
      Box::new(ExtractJsonLd),
      Box::new(ExtractMetaTags),
      Box::new(RemoveDisallowedNodes),
      Box::new(RewriteLineBreaks),
      Box::new(RewriteFontTags),
      Box::new(ExtractTitle),
      Box::new(UnescapeHtmlEntities),
      Box::new(GrabArticle::new(vec![
        Box::new(PrepareNodes::new(vec![
          Box::new(RemoveHiddenNodes),
          Box::new(ExtractByline),
          Box::new(RemoveDuplicateTitleHeader::default()),
          Box::new(RemoveUnlikelyCandidates),
          Box::new(RemoveEmptyContainers),
          Box::new(ConvertDivsToParagraphs),
        ])),
        Box::new(ScoreCandidates),
        Box::new(SelectTopCandidate),
        Box::new(ExtractArticle),
        Box::new(CleanStyles),
        Box::new(MarkDataTables),
        Box::new(FixLazyImages),
        Box::new(CleanConditionally("FORM")),
        Box::new(CleanConditionally("FIELDSET")),
        Box::new(Clean("OBJECT")),
        Box::new(Clean("EMBED")),
        Box::new(Clean("FOOTER")),
        Box::new(Clean("LINK")),
        Box::new(Clean("ASIDE")),
        Box::new(CleanShareElements),
        Box::new(Clean("IFRAME")),
        Box::new(Clean("INPUT")),
        Box::new(Clean("TEXTAREA")),
        Box::new(Clean("SELECT")),
        Box::new(Clean("BUTTON")),
        Box::new(CleanHeaders),
        Box::new(CleanConditionally("TABLE")),
        Box::new(CleanConditionally("UL")),
        Box::new(CleanConditionally("DIV")),
        Box::new(RenameHeadings),
        Box::new(RemoveEmptyParagraphs),
        Box::new(RemoveBreaksBeforeParagraphs),
        Box::new(UnwrapSingleCellTables),
        Box::new(WrapArticlePage),
        Box::new(ExtractDir),
      ])),
      Box::new(FixRelativeUris),
      Box::new(SimplifyNestedElements),
      Box::new(CleanClasses),
      Box::new(ExtractExcerpt),
    ];

    for stage in stages {
      pipeline.add_stage(stage);
    }

    pipeline
  }
}
