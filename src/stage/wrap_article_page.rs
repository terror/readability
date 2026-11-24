use super::*;

const PAGE_CLASS: &str = "page";

const PAGE_ID: &str = "readability-page-1";

pub(crate) struct WrapArticlePage;

impl Stage for WrapArticlePage {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(article) = context.extraction.article else {
      return Ok(());
    };

    let article = context.node(article);

    let page = if context.extraction.created_top_candidate {
      context.extraction.top_candidate.map(|id| context.node(id))
    } else {
      None
    };

    let page = page.unwrap_or_else(|| {
      let page = create_element(&context.document.tree, "div");

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
  fn reuses_created_top_candidate() {
    let mut document =
      dom_query::Document::from("<html><body>foo</body></html>");

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    SelectTopCandidate.run(&mut context).unwrap();
    ExtractArticle.run(&mut context).unwrap();
    WrapArticlePage.run(&mut context).unwrap();

    assert_eq!(
      context
        .node(context.extraction.article.unwrap())
        .html()
        .to_string(),
      r#"<div><div id="readability-page-1" class="page">foo</div></div>"#,
    );
  }

  #[test]
  fn wraps_article_children() {
    let mut document = dom_query::Document::from("<html><body></body></html>");

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    let article = create_element(&context.document.tree, "div");

    article.append_html("<p>foo</p><p>bar</p>");

    context.extraction.article = Some(article.id);

    WrapArticlePage.run(&mut context).unwrap();

    assert_eq!(
      context
        .node(context.extraction.article.unwrap())
        .html()
        .to_string(),
      r#"<div><div id="readability-page-1" class="page"><p>foo</p><p>bar</p></div></div>"#,
    );
  }
}
