use super::*;

pub struct EnforceVoidSelfClosingStage;

impl Stage for EnforceVoidSelfClosingStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(markup) = context.take_article_markup() else {
      return Ok(());
    };

    let updated_markup = re::IMG_MISSING_SELF_CLOSING
      .replace_all(&markup.replace("<br>", "<br />"), "<img$1 />")
      .to_string();

    context.set_article_markup(updated_markup);

    Ok(())
  }
}
