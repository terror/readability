use super::*;

pub struct LanguageStage;

impl Stage for LanguageStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let lang = context
      .document()
      .html_element()
      .and_then(ElementRef::wrap)
      .and_then(|el| el.value().attr("lang"))
      .map(str::to_string);

    context.set_document_lang(lang);

    Ok(())
  }
}
