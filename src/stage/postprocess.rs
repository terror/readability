use super::*;

pub struct EnforceVoidSelfClosingStage;

impl Stage for EnforceVoidSelfClosingStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let Some(fragment) = context.take_article_fragment() else {
      return Ok(());
    };

    context.set_article_markup(Self::finalize_fragment(fragment));

    Ok(())
  }
}

impl EnforceVoidSelfClosingStage {
  fn enforce_void_self_closing(markup: String) -> String {
    const BR_PLACEHOLDER: &str = "__readability_br_placeholder__";

    let intermediate = markup
      .replace("<br />", BR_PLACEHOLDER)
      .replace("<br>", "<br />")
      .replace(BR_PLACEHOLDER, "<br />");

    let mut result = String::with_capacity(intermediate.len());
    let mut remainder = intermediate.as_str();

    while let Some(idx) = remainder.find("<img") {
      let (before, after) = remainder.split_at(idx);
      result.push_str(before);

      if let Some(end) = after.find('>') {
        let (tag, rest) = after.split_at(end + 1);
        if tag.trim_end().ends_with("/>") {
          result.push_str(tag);
        } else {
          let trimmed = tag.trim_end_matches('>');
          result.push_str(trimmed);
          result.push_str(" />");
        }
        remainder = rest;
      } else {
        result.push_str(after);
        remainder = "";
        break;
      }
    }

    result.push_str(remainder);
    result
  }

  fn finalize_fragment(fragment: ArticleFragment) -> String {
    let markup = fragment.serialize().unwrap_or_default();

    if markup.is_empty() {
      return markup;
    }

    if let Ok(selector) = Selector::parse("#readability-page-1")
      && let Some(element) = fragment.html().select(&selector).next()
    {
      let inner = element.inner_html();

      let markup =
        format!("<div id=\"readability-page-1\" class=\"page\">{inner}</div>");

      return Self::enforce_void_self_closing(markup);
    }

    Self::enforce_void_self_closing(markup)
  }
}
