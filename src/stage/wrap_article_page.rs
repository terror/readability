use super::*;

const PAGE_CLASS: &str = "page";

const PAGE_ID: &str = "readability-page-1";

pub(crate) struct WrapArticlePage;

impl Stage for WrapArticlePage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(article) = context.article else {
      return Ok(());
    };

    let article = context.node(article);

    let page = if context.created_top_candidate {
      context.top_candidate.map(|id| context.node(id))
    } else {
      None
    };

    let page = page.unwrap_or_else(|| {
      let page = context.document.tree.new_element("div");

      while let Some(child) = article.first_child() {
        page.append_child(&child);
      }

      article.append_child(&page);

      page
    });

    page.set_attr("id", PAGE_ID);
    page.set_attr("class", PAGE_CLASS);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn wraps_article_children() {
    let mut document = dom_query::Document::from("<html><body></body></html>");

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    let article = context.document.tree.new_element("div");

    article.append_html("<p>foo</p><p>bar</p>");

    context.article = Some(article.id);

    WrapArticlePage.run(&mut context).unwrap();

    assert_eq!(
      context.node(context.article.unwrap()).html().to_string(),
      r#"<div><div id="readability-page-1" class="page"><p>foo</p><p>bar</p></div></div>"#,
    );
  }

  #[test]
  fn reuses_created_top_candidate() {
    let mut document =
      dom_query::Document::from("<html><body>foo</body></html>");

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    SelectTopCandidate.run(&mut context).unwrap();
    ExtractArticle.run(&mut context).unwrap();
    WrapArticlePage.run(&mut context).unwrap();

    assert_eq!(
      context.node(context.article.unwrap()).html().to_string(),
      r#"<div><div id="readability-page-1" class="page">foo</div></div>"#,
    );
  }
}
