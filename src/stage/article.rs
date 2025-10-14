//! Extracts the most relevant article fragment from a parsed document by
//! scoring candidate nodes and assembling the best matching content.

use super::*;

/// HTML tags considered strong indicators of readable article content.
const DEFAULT_TAGS_TO_SCORE: &[&str] =
  &["section", "h2", "h3", "h4", "h5", "h6", "p", "td", "pre"];

/// Regular expression that captures comma-like punctuation in multiple locales.
static REGEX_COMMAS: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"[,،﹐︐﹑⹀⸲，]").unwrap());

/// Minimum amount of trimmed text a node must contain to be scored.
const MIN_TEXT_LENGTH: usize = 25;
/// Ratio of the top candidate score used to decide if a sibling is included.
const SIBLING_SCORE_RATIO: f64 = 0.2;
/// Absolute sibling score floor to prevent including very weak candidates.
const MIN_SIBLING_SCORE: f64 = 10.0;
/// Maximum depth when propagating scores to ancestor nodes.
const MAX_PARENT_DEPTH: usize = 5;

/// Article fragment paired with the language discovered in the `<body>` tag.
struct ArticleContent {
  /// Language code taken from the document's `<body lang>` attribute.
  body_lang: Option<String>,
  /// HTML fragment representing the primary article content.
  fragment: ArticleFragment,
}

#[derive(Debug, Clone)]
struct Candidate {
  /// Identifier of the DOM node that produced the score.
  node: NodeId,
  /// Aggregated readability score for the candidate node.
  score: f64,
}

/// Stage responsible for selecting the best article fragment from the document.
pub struct ArticleStage;

impl Stage for ArticleStage {
  /// Runs the extraction pipeline and stores the resulting fragment and language
  /// hints into the shared context.
  fn run(&mut self, context: &mut Context<'_>) -> Result<()> {
    let article =
      Self::extract(context.document()).ok_or(Error::MissingArticleContent)?;

    context.set_body_lang(article.body_lang);
    context.set_article_fragment(article.fragment);

    Ok(())
  }
}

impl ArticleStage {
  /// Assigns a readability score to a candidate element based on length and
  /// punctuation density heuristics.
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

  /// Assembles the HTML representing the main article by merging the top
  /// candidate with qualifying sibling nodes.
  fn collect_article_parts(
    document: Document<'_>,
    top_candidate: NodeId,
    candidates: &HashMap<NodeId, Candidate>,
  ) -> Option<String> {
    let (top_node, top_score) = (
      document.node(top_candidate)?,
      candidates.get(&top_candidate)?.score,
    );

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

  /// Extracts the highest scoring article fragment and its metadata from the
  /// provided document.
  fn extract(document: Document<'_>) -> Option<ArticleContent> {
    let body_id = document.body_element()?.id();

    let candidates = Self::score_candidates(document, body_id);

    let top_candidate = Self::promote_single_child_parent(
      document,
      Self::find_top_candidate(&candidates)?,
    );

    let article_html =
      Self::collect_article_parts(document, top_candidate, &candidates)?;

    Some(ArticleContent {
      body_lang: Self::extract_body_lang(document, body_id),
      fragment: ArticleFragment::from_markup(&article_html),
    })
  }

  /// Reads the language specified on the `<body>` element, if any.
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

  /// Returns the identifier associated with the highest scoring candidate node.
  fn find_top_candidate(
    candidates: &HashMap<NodeId, Candidate>,
  ) -> Option<NodeId> {
    candidates
      .values()
      .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
      .map(|c| c.node)
  }

  /// Checks whether a paragraph contains enough natural language text and low
  /// link density to be incorporated into the article.
  fn is_valid_paragraph(document: Document<'_>, node_id: NodeId) -> bool {
    let (text, link_density) = (
      document.collect_text(node_id, true),
      document.link_density(node_id),
    );

    let len = text.len();

    (len > 80 && link_density < 0.25)
      || (len > 0 && len <= 80 && link_density == 0.0 && text.contains('.'))
  }

  /// Produces HTML for a sibling node when it meets the inclusion heuristics.
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

  /// Generates weighted score contributions for the ancestors of a scored node.
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

  /// Computes readability scores for nodes in the `<body>` subtree.
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

  /// Determines whether a sibling element should be merged into the article
  /// output based on scoring and structural heuristics.
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

  /// Promotes a candidate node to its parent when it is the only element child.
  fn promote_single_child_parent(
    document: Document<'_>,
    mut node_id: NodeId,
  ) -> NodeId {
    loop {
      let Some(node) = document.node(node_id) else {
        break;
      };

      let Some(parent) = node.parent() else {
        break;
      };

      let Node::Element(element) = parent.value() else {
        break;
      };

      if element.name() == "body" {
        break;
      }

      let element_children = parent
        .children()
        .filter(|child| child.value().is_element())
        .count();

      if element_children == 1 {
        node_id = parent.id();
      } else {
        break;
      }
    }

    node_id
  }
}
