use super::*;

pub struct LanguageStage;

impl Stage for LanguageStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let root = context.document().html_element();

    context.set_document_lang(
      root
        .and_then(ElementRef::wrap)
        .and_then(|element| element.value().attr("lang"))
        .map(str::to_string),
    );

    Ok(())
  }
}
