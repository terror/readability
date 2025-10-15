use super::*;

pub struct EnforceVoidSelfClosingStage;

impl Stage for EnforceVoidSelfClosingStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(markup) = context.take_article_markup() else {
      return Ok(());
    };

    context.set_article_markup(
      Regex::new(r"<img([^>]*[^/])>")
        .unwrap()
        .replace_all(&markup.replace("<br>", "<br />"), "<img$1 />")
        .to_string(),
    );

    Ok(())
  }
}
