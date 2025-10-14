use super::*;

const DEFAULT_TAGS_TO_SCORE: &[&str] =
  &["section", "h2", "h3", "h4", "h5", "h6", "p", "td", "pre"];

static REGEX_COMMAS: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"[,،﹐︐﹑⹀⸲，]").unwrap());

const MIN_TEXT_LENGTH: usize = 25;
const SIBLING_SCORE_RATIO: f64 = 0.2;
const MIN_SIBLING_SCORE: f64 = 10.0;
const MAX_PARENT_DEPTH: usize = 5;

struct ArticleContent {
  body_lang: Option<String>,
  fragment: ArticleFragment,
}

#[derive(Debug, Clone)]
struct Candidate {
  node: NodeId,
  score: f64,
}

pub struct ArticleStage;

impl Stage for ArticleStage {
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let article =
      Self::extract(context.document()).ok_or(Error::MissingArticleContent)?;

    context.set_body_lang(article.body_lang);
    context.set_article_fragment(article.fragment);

    Ok(())
  }
}

impl ArticleStage {
  fn calculate_element_score(element: ElementRef<'_>) -> Option<f64> {
    let text = element.text().collect::<Vec<_>>().join(" ");

    let text = text.trim();

    if text.len() < MIN_TEXT_LENGTH {
      return None;
    }

    let comma_count = f64::from(
      u32::try_from(REGEX_COMMAS.find_iter(text).count()).unwrap_or(u32::MAX),
    );

    let length_bonus =
      f64::from(u32::try_from((text.len() / 100).min(3)).unwrap_or(3));

    Some(1.0 + comma_count + length_bonus)
  }

  fn collect_article_parts(
    document: Document<'_>,
    top_candidate: NodeId,
    candidates: &HashMap<NodeId, Candidate>,
  ) -> Option<String> {
    let top_node = document.node(top_candidate)?;
    let top_score = candidates.get(&top_candidate)?.score;

    let threshold = (top_score * SIBLING_SCORE_RATIO).max(MIN_SIBLING_SCORE);

    let parts = top_node
      .parent()
      .map(|parent| {
        parent
          .children()
          .filter_map(|child| {
            Self::process_sibling(
              document,
              child,
              top_candidate,
              candidates,
              threshold,
            )
          })
          .collect::<String>()
      })
      .or_else(|| ElementRef::wrap(top_node).map(|el| el.html()))?;

    if parts.is_empty() { None } else { Some(parts) }
  }

  fn extract(document: Document<'_>) -> Option<ArticleContent> {
    let body_id = document.body_element()?.id();

    let candidates = Self::score_candidates(document, body_id);

    let top_candidate = Self::find_top_candidate(&candidates)?;

    let article_html =
      Self::collect_article_parts(document, top_candidate, &candidates)?;

    Some(ArticleContent {
      body_lang: Self::extract_body_lang(document, body_id),
      fragment: ArticleFragment::from_markup(&article_html),
    })
  }

  fn extract_body_lang(
    document: Document<'_>,
    body_id: NodeId,
  ) -> Option<String> {
    document
      .node(body_id)
      .and_then(ElementRef::wrap)
      .and_then(|el| el.value().attr("lang"))
      .map(str::to_string)
  }

  fn find_top_candidate(
    candidates: &HashMap<NodeId, Candidate>,
  ) -> Option<NodeId> {
    candidates
      .values()
      .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
      .map(|c| c.node)
  }

  fn is_valid_paragraph(document: Document<'_>, node_id: NodeId) -> bool {
    let (text, link_density) = (
      document.collect_text(node_id, true),
      document.link_density(node_id),
    );

    let len = text.len();

    (len > 80 && link_density < 0.25)
      || (len > 0 && len <= 80 && link_density == 0.0 && text.contains('.'))
  }

  fn process_sibling(
    document: Document<'_>,
    child: ego_tree::NodeRef<'_, Node>,
    top_candidate: NodeId,
    candidates: &HashMap<NodeId, Candidate>,
    threshold: f64,
  ) -> Option<String> {
    match child.value() {
      Node::Text(text) => {
        let text = text.to_string();

        if text.is_empty() { None } else { Some(text) }
      }
      Node::Element(_) => {
        let element = ElementRef::wrap(child)?;

        if Self::should_include_sibling(
          document,
          element,
          child.id(),
          top_candidate,
          candidates,
          threshold,
        ) {
          Some(element.html())
        } else {
          None
        }
      }
      _ => None,
    }
  }

  fn propagate_score_to_parents<'a>(
    node: &'a ego_tree::NodeRef<'a, Node>,
    score: f64,
  ) -> impl Iterator<Item = (NodeId, f64)> + 'a {
    std::iter::successors(node.parent(), NodeRef::parent)
      .take(MAX_PARENT_DEPTH)
      .enumerate()
      .map(move |(level, parent)| {
        let divider = match level {
          0 => 1.0,
          1 => 2.0,
          _ => {
            (f64::from(u32::try_from(level).unwrap_or(u32::MAX)) + 1.0) * 3.0
          }
        };
        (parent.id(), score / divider)
      })
  }

  fn score_candidates(
    document: Document<'_>,
    body_id: NodeId,
  ) -> HashMap<NodeId, Candidate> {
    let Some(body) = document.node(body_id) else {
      return HashMap::new();
    };

    body
      .descendants()
      .filter_map(ElementRef::wrap)
      .filter(|el| DEFAULT_TAGS_TO_SCORE.contains(&el.value().name()))
      .filter_map(|el| {
        Self::calculate_element_score(el).map(|score| (el, score))
      })
      .flat_map(|(el, score)| {
        Self::propagate_score_to_parents(&el, score).collect::<Vec<_>>()
      })
      .fold(HashMap::new(), |mut acc, (node_id, score)| {
        acc
          .entry(node_id)
          .and_modify(|c| c.score += score)
          .or_insert(Candidate {
            node: node_id,
            score,
          });
        acc
      })
  }

  fn should_include_sibling(
    document: Document<'_>,
    element: ElementRef<'_>,
    child_id: NodeId,
    top_candidate: NodeId,
    candidates: &HashMap<NodeId, Candidate>,
    threshold: f64,
  ) -> bool {
    if child_id == top_candidate {
      return true;
    }

    let candidate_score = candidates.get(&child_id).map_or(0.0, |c| c.score);

    if candidate_score >= threshold {
      return true;
    }

    if element.value().name() == "p" {
      Self::is_valid_paragraph(document, child_id)
    } else {
      false
    }
  }
}
