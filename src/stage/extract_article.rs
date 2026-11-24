use super::*;

const ALTER_TO_DIV_EXCEPTIONS: &[&str] =
  &["DIV", "ARTICLE", "SECTION", "P", "OL", "UL"];

const MAX_PARAGRAPH_LINK_DENSITY: f64 = 0.25;

const MIN_PARAGRAPH_LENGTH: usize = 80;

const MIN_SIBLING_SCORE_THRESHOLD: f64 = 10.0;

const SIBLING_SCORE_RATIO: f64 = 0.2;

pub(crate) struct ExtractArticle;

impl ExtractArticle {
  fn is_content_paragraph(node: &NodeRef) -> bool {
    let link_density = node.link_density();

    let text = node.inner_text();

    let length = text_length(&text);

    if length > MIN_PARAGRAPH_LENGTH {
      link_density < MAX_PARAGRAPH_LINK_DENSITY
    } else {
      length > 0
        && length < MIN_PARAGRAPH_LENGTH
        && link_density == 0.0
        && SENTENCE_END.is_match(&text)
    }
  }
}

impl Stage for ExtractArticle {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let Some(top_candidate) = context.extraction.top_candidate else {
      return Ok(());
    };

    let top_candidate = context.node(top_candidate);

    let Some(parent) = top_candidate.parent() else {
      return Ok(());
    };

    let top_score = context.extraction.scores[&top_candidate.id];

    let threshold =
      MIN_SIBLING_SCORE_THRESHOLD.max(top_score * SIBLING_SCORE_RATIO);

    let top_class = top_candidate.attr("class").unwrap_or_default();

    let article = create_element(&context.document.tree, "div");

    for sibling in parent.element_children() {
      let append = sibling.id == top_candidate.id || {
        let bonus = if !top_class.is_empty()
          && sibling.attr("class").unwrap_or_default() == top_class
        {
          top_score * SIBLING_SCORE_RATIO
        } else {
          0.0
        };

        context
          .extraction
          .scores
          .get(&sibling.id)
          .is_some_and(|score| score + bonus >= threshold)
          || (sibling.tag() == "P" && Self::is_content_paragraph(&sibling))
      };

      if !append {
        continue;
      }

      if !ALTER_TO_DIV_EXCEPTIONS.contains(&sibling.tag().as_str()) {
        sibling.set_tag("div");
      }

      article.append_child(&sibling);
    }

    let (article, parent) = (article.id, parent.id);

    context.extraction.article = Some(article);
    context.extraction.top_candidate_parent = Some(parent);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn article(html: &str) -> String {
    let mut document = dom_query::Document::from(html);

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    ScoreCandidates.run(&mut context).unwrap();
    SelectTopCandidate.run(&mut context).unwrap();
    ExtractArticle.run(&mut context).unwrap();

    context
      .node(context.extraction.article.unwrap())
      .html()
      .to_string()
  }

  #[test]
  fn appends_content_siblings() {
    let paragraph = format!("<p>{}</p>", "foo, ".repeat(10));

    assert_eq!(
      article(&format!(
        r#"<html><body><div id="foo">{paragraph}{paragraph}</div><p>bar.</p><p>baz</p><form><p>qux</p></form></body></html>"#
      )),
      format!(
        r#"<div><div id="foo">{paragraph}{paragraph}</div><p>bar.</p></div>"#
      ),
    );
  }

  #[test]
  fn alters_top_candidate_to_div() {
    let paragraph = format!("<p>{}</p>", "foo, ".repeat(10));

    assert_eq!(
      article(&format!(
        "<html><body><form>{paragraph}{paragraph}</form><span></span></body></html>"
      )),
      format!("<div><div>{paragraph}{paragraph}</div></div>"),
    );
  }
}
