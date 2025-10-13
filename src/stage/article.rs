use super::*;

const DEFAULT_TAGS_TO_SCORE: &[&str] =
  &["section", "h2", "h3", "h4", "h5", "h6", "p", "td", "pre"];

static REGEX_COMMAS: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"[,،﹐︐﹑⹀⸲，]").unwrap());

struct ArticleContent {
  body_lang: Option<String>,
  markup: String,
}

#[derive(Debug, Clone)]
struct Candidate {
  node: NodeId,
  score: f64,
}

pub struct ArticleStage;

impl Stage for ArticleStage {
  fn run(&mut self, ctx: &mut Context<'_>) -> Result<()> {
    let article =
      Self::extract(ctx.document()).ok_or(Error::MissingArticleContent)?;

    ctx.set_body_lang(article.body_lang);
    ctx.set_article_markup(article.markup);

    Ok(())
  }
}

impl ArticleStage {
  fn extract(document: Document<'_>) -> Option<ArticleContent> {
    let body = document.body_element()?;

    let body_id = body.id();

    let body_lang = document
      .node(body_id)
      .and_then(ElementRef::wrap)
      .and_then(|el| el.value().attr("lang"))
      .map(str::to_string);

    let body = document.node(body_id)?;

    let mut candidates: HashMap<NodeId, Candidate> = HashMap::new();

    for node in body.descendants() {
      let Some(element) = ElementRef::wrap(node) else {
        continue;
      };

      if !DEFAULT_TAGS_TO_SCORE.contains(&element.value().name()) {
        continue;
      }

      let text = element.text().collect::<Vec<_>>().join(" ");
      let text = text.trim();

      if text.len() < 25 {
        continue;
      }

      let mut score = 1.0;
      score += u32::try_from(REGEX_COMMAS.find_iter(text).count())
        .map_or(0.0, f64::from);
      score += u32::try_from((text.len() / 100).min(3)).map_or(0.0, f64::from);

      let mut node = element.deref().parent();
      let mut level = 0;

      while let Some(parent) = node {
        let entry = candidates.entry(parent.id()).or_insert(Candidate {
          node: parent.id(),
          score: 0.0,
        });

        let divider = match level {
          0 => 1.0,
          1 => 2.0,
          _ => (f64::from(level) + 1.0) * 3.0,
        };

        entry.score += score / divider;

        level += 1;

        if level >= 5 {
          break;
        }

        node = parent.parent();
      }
    }

    if candidates.is_empty() {
      return None;
    }

    let mut top_candidates: Vec<Candidate> =
      candidates.values().cloned().collect();

    top_candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    let top_score = top_candidates
      .first()
      .map_or(0.0, |candidate| candidate.score);

    let sibling_score_threshold = (top_score * 0.2).max(10.0);
    let top_candidate = top_candidates.first()?.node;

    let mut article_parts = Vec::new();

    let top_node = document.node(top_candidate)?;
    let parent = top_node.parent();

    if let Some(parent) = parent {
      for child in parent.children() {
        let Some(element) = ElementRef::wrap(child) else {
          continue;
        };

        let append = if child.id() == top_candidate {
          true
        } else {
          let candidate_score = candidates
            .get(&child.id())
            .map_or(0.0, |candidate| candidate.score);

          if candidate_score >= sibling_score_threshold {
            true
          } else if element.value().name() == "p" {
            let text = document.collect_text(child.id(), true);
            let link_density = document.link_density(child.id());
            let len = text.len();

            (len > 80 && link_density < 0.25)
              || (len > 0
                && len <= 80
                && link_density == 0.0
                && text.contains('.'))
          } else {
            false
          }
        };

        if append {
          article_parts.push(element.html());
        }
      }
    } else if let Some(element) = ElementRef::wrap(top_node) {
      article_parts.push(element.html());
    }

    if article_parts.is_empty() {
      return None;
    }

    let markup = format!(
      "<div id=\"readability-content\"><div id=\"readability-page-1\" class=\"page\">{}</div></div>",
      article_parts.join("")
    );

    Some(ArticleContent { body_lang, markup })
  }
}
