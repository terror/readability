use super::*;

pub struct LanguageStage;

impl Stage for LanguageStage {
  fn run(&mut self, ctx: &mut Context<'_>) -> Result<()> {
    let lang = ctx
      .document()
      .html_element()
      .and_then(ElementRef::wrap)
      .and_then(|el| el.value().attr("lang"))
      .map(|value| value.to_string());

    ctx.set_document_lang(lang);

    Ok(())
  }
}
