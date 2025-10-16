use super::*;

const DEFAULT_TAGS_TO_SCORE: &[&str] =
  &["section", "h2", "h3", "h4", "h5", "h6", "p", "td", "pre"];

static REGEX_COMMAS: LazyLock<Regex> =
  LazyLock::new(|| Regex::new(r"[,،﹐︐﹑⹀⸲，]").unwrap());

static REGEX_POSITIVE: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(concat!(
    r"(?i)article|body|content|entry|hentry|h-entry|main|page|pagination|",
    r"post|text|blog|story"
  ))
  .unwrap()
});

static REGEX_NEGATIVE: LazyLock<Regex> = LazyLock::new(|| {
  Regex::new(concat!(
    r"(?i)-ad-|hidden|^hid$| hid$| hid |^hid |banner|combx|comment|com-|",
    r"contact|footer|gdpr|masthead|media|meta|outbrain|promo|related|scroll|",
    r"share|shoutbox|sidebar|skyscraper|sponsor|shopping|tags|widget"
  ))
  .unwrap()
});

/// Minimum amount of trimmed text a node must contain to be scored.
const MIN_TEXT_LENGTH: usize = 25;

/// Ratio of the top candidate score used to decide if a sibling is included.
const SIBLING_SCORE_RATIO: f64 = 0.2;

/// Absolute sibling score floor to prevent including very weak candidates.
const MIN_SIBLING_SCORE: f64 = 10.0;

/// Maximum number of high scoring candidates kept for ancestor evaluation.
const DEFAULT_TOP_CANDIDATES: usize = 5;

/// Minimum number of strong candidates that must agree on an ancestor.
const MINIMUM_TOP_CANDIDATE_SUPPORT: usize = 3;

/// Score ratio threshold when considering alternative top candidates.
const TOP_CANDIDATE_SCORE_RATIO: f64 = 0.75;

/// Additional score bonus for siblings sharing the top candidate's class.
const CLASS_BONUS_RATIO: f64 = 0.2;

/// Maximum depth when propagating scores to ancestor nodes.
const MAX_PARENT_DEPTH: usize = 5;

struct ArticleContent {
  /// Language code taken from the document's `<body lang>` attribute.
  body_lang: Option<String>,
  /// Text direction derived from the article container hierarchy.
  dir: Option<String>,
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
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let ArticleContent {
      body_lang,
      dir,
      fragment,
    } =
      Self::extract(context.document()).ok_or(Error::MissingArticleContent)?;

    context.set_body_lang(body_lang);
    context.set_article_dir(dir);

    if context.metadata().excerpt.is_none() {
      let selector = Selector::parse("p")
        .map_err(|error| Error::InvalidSelector(error.to_string()))?;

      let first_paragraph = fragment.html.select(&selector);

      let excerpt = first_paragraph
        .map(|element| {
          element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string()
        })
        .find(|text| !text.is_empty());

      if let Some(excerpt) = excerpt {
        context.set_metadata(Metadata {
          excerpt: Some(excerpt),
          ..context.metadata().clone()
        });
      }
    }

    context.set_article_fragment(fragment);

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

  fn class_weight(element: ElementRef<'_>) -> f64 {
    let mut weight = 0.0;

    if let Some(class_attr) = element.value().attr("class") {
      if REGEX_NEGATIVE.is_match(class_attr) {
        weight -= 25.0;
      }

      if REGEX_POSITIVE.is_match(class_attr) {
        weight += 25.0;
      }
    }

    if let Some(id_attr) = element.value().attr("id") {
      if REGEX_NEGATIVE.is_match(id_attr) {
        weight -= 25.0;
      }

      if REGEX_POSITIVE.is_match(id_attr) {
        weight += 25.0;
      }
    }

    weight
  }

  /// Assembles the HTML representing the main article by merging the top
  /// candidate with qualifying sibling nodes.
  fn collect_article_parts(
    document: Document<'_>,
    top_candidate: NodeId,
    candidates: &HashMap<NodeId, Candidate>,
    top_score: f64,
    top_class: Option<&str>,
  ) -> Option<String> {
    let top_node = document.node(top_candidate)?;
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
              top_score,
              top_class,
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
    let body = document.body_element()?;

    let candidates = Self::score_candidates(document, body.id());

    let top_candidates = Self::top_candidates(&candidates);

    let Some(first_candidate) = top_candidates.first().copied() else {
      return Self::fallback_article(document, body);
    };

    let mut top_candidate = Self::select_top_candidate(
      document,
      &candidates,
      &top_candidates,
      body.id(),
    )
    .unwrap_or(first_candidate);

    top_candidate = Self::promote_single_child_parent(document, top_candidate);

    let top_candidate_score = candidates
      .get(&top_candidate)
      .map_or(0.0, |candidate| candidate.score);

    let top_candidate_class = document
      .node(top_candidate)
      .and_then(ElementRef::wrap)
      .and_then(|el| el.value().attr("class"))
      .map(str::to_string);

    let article_html = match Self::collect_article_parts(
      document,
      top_candidate,
      &candidates,
      top_candidate_score,
      top_candidate_class.as_deref(),
    ) {
      Some(html) if !html.trim().is_empty() => html,
      _ => return Self::fallback_article(document, body),
    };

    Some(ArticleContent {
      body_lang: Self::extract_body_lang(document, body.id()),
      dir: Self::find_article_dir(document, top_candidate),
      fragment: ArticleFragment::from(article_html.as_str()),
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

  /// Provides a fallback article fragment using the `<body>` contents when no
  /// suitable candidate could be scored.
  fn fallback_article(
    document: Document<'_>,
    body: NodeRef<'_, Node>,
  ) -> Option<ArticleContent> {
    let mut buffer = Vec::new();

    let serializer = SerializableNode { node: body };

    serialize(
      &mut buffer,
      &serializer,
      SerializeOpts {
        scripting_enabled: false,
        traversal_scope: TraversalScope::ChildrenOnly(None),
        create_missing_parent: false,
      },
    )
    .ok()?;

    let markup = String::from_utf8(buffer).ok()?;

    if markup.trim().is_empty() {
      return None;
    }

    let body_id = body.id();

    Some(ArticleContent {
      body_lang: Self::extract_body_lang(document, body_id),
      dir: Self::find_article_dir(document, body_id),
      fragment: ArticleFragment::from(markup.as_str()),
    })
  }

  /// Traverses the candidate's ancestor chain to find a `dir` attribute hint.
  fn find_article_dir(
    document: Document<'_>,
    node_id: NodeId,
  ) -> Option<String> {
    let node = document.node(node_id)?;

    if let Some(parent) = node.parent() {
      if let Some(dir) = Self::node_dir(parent) {
        return Some(dir);
      }

      if let Some(dir) = Self::node_dir(node) {
        return Some(dir);
      }

      let mut ancestor = parent.parent();

      while let Some(current) = ancestor {
        if let Some(dir) = Self::node_dir(current) {
          return Some(dir);
        }

        ancestor = current.parent();
      }
    } else if let Some(dir) = Self::node_dir(node) {
      return Some(dir);
    }

    None
  }

  fn initial_score(document: Document<'_>, node_id: NodeId) -> f64 {
    document
      .node(node_id)
      .and_then(ElementRef::wrap)
      .map_or(0.0, |element| {
        Self::node_base_score(element) + Self::class_weight(element)
      })
  }

  fn is_body_node(node: &Node) -> bool {
    matches!(node, Node::Element(element) if element.name() == "body")
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

  fn node_ancestors(document: Document<'_>, node_id: NodeId) -> Vec<NodeId> {
    let mut ancestors = Vec::new();
    let mut current = document.node(node_id).and_then(|node| node.parent());

    while let Some(node) = current {
      ancestors.push(node.id());
      current = node.parent();
    }

    ancestors
  }

  fn node_base_score(element: ElementRef<'_>) -> f64 {
    match element.value().name() {
      "div" => 5.0,
      "pre" | "td" | "blockquote" => 3.0,
      "address" | "ol" | "ul" | "dl" | "dd" | "dt" | "li" | "form" => -3.0,
      "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "th" => -5.0,
      _ => 0.0,
    }
  }

  fn node_dir(node: NodeRef<'_, Node>) -> Option<String> {
    match node.value() {
      Node::Element(element) => element.attr("dir").map(str::to_string),
      _ => None,
    }
  }

  /// Produces HTML for a sibling node when it meets the inclusion heuristics.
  fn process_sibling(
    document: Document<'_>,
    child: ego_tree::NodeRef<'_, Node>,
    top_candidate: NodeId,
    candidates: &HashMap<NodeId, Candidate>,
    threshold: f64,
    top_score: f64,
    top_class: Option<&str>,
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
          top_score,
          top_class,
        ) {
          Some(element.html())
        } else {
          None
        }
      }
      _ => None,
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

    let mut candidates = body
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
          .and_modify(|c: &mut Candidate| c.score += score)
          .or_insert_with(|| Candidate {
            node: node_id,
            score: score + Self::initial_score(document, node_id),
          });
        acc
      });

    for candidate in candidates.values_mut() {
      let link_density = document.link_density(candidate.node);
      candidate.score *= 1.0 - link_density;
    }

    candidates
  }

  fn select_top_candidate(
    document: Document<'_>,
    candidates: &HashMap<NodeId, Candidate>,
    top_candidates: &[NodeId],
    body_id: NodeId,
  ) -> Option<NodeId> {
    let mut top_candidate = *top_candidates.first()?;

    if top_candidate == body_id {
      return Some(top_candidate);
    }

    let top_score = candidates.get(&top_candidate)?.score;

    let alternative_ancestors = top_candidates
      .iter()
      .skip(1)
      .filter_map(|candidate_id| {
        candidates.get(candidate_id).and_then(|candidate| {
          if candidate.score / top_score >= TOP_CANDIDATE_SCORE_RATIO {
            Some(Self::node_ancestors(document, *candidate_id))
          } else {
            None
          }
        })
      })
      .collect::<Vec<_>>();

    if alternative_ancestors.len() >= MINIMUM_TOP_CANDIDATE_SUPPORT {
      let mut parent =
        document.node(top_candidate).and_then(|node| node.parent());

      while let Some(current) = parent {
        if Self::is_body_node(current.value()) {
          break;
        }

        let current_id = current.id();

        let support = alternative_ancestors
          .iter()
          .filter(|ancestors| ancestors.contains(&current_id))
          .count();

        if support >= MINIMUM_TOP_CANDIDATE_SUPPORT {
          top_candidate = current_id;
          break;
        }

        parent = current.parent();
      }
    }

    let mut parent =
      document.node(top_candidate).and_then(|node| node.parent());

    let mut last_score = top_score;

    let score_threshold = top_score / 3.0;

    while let Some(current) = parent {
      if Self::is_body_node(current.value()) {
        break;
      }

      let parent_id = current.id();

      let Some(parent_candidate) = candidates.get(&parent_id) else {
        parent = current.parent();
        continue;
      };

      let parent_score = parent_candidate.score;

      if parent_score < score_threshold {
        break;
      }

      if parent_score > last_score {
        top_candidate = parent_id;
        break;
      }

      last_score = parent_score;
      parent = current.parent();
    }

    if let Some(node) = document.node(top_candidate)
      && let Some(element) = ElementRef::wrap(node)
      && element.value().name() == "article"
      && let Some(parent) = node.parent()
      && let Some(parent_element) = ElementRef::wrap(parent)
    {
      let parent_score = candidates
        .get(&parent.id())
        .map_or(0.0, |candidate| candidate.score);

      if parent_score >= MIN_SIBLING_SCORE
        && matches!(parent_element.value().name(), "div" | "section" | "main")
      {
        top_candidate = parent.id();
      }
    }

    Some(top_candidate)
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
    top_score: f64,
    top_class: Option<&str>,
  ) -> bool {
    if child_id == top_candidate {
      return true;
    }

    let mut candidate_score =
      candidates.get(&child_id).map_or(0.0, |c| c.score);

    if candidate_score > 0.0
      && let Some(top_class) = top_class.filter(|cls| !cls.is_empty())
      && let Some(sibling_class) = element.value().attr("class")
      && sibling_class == top_class
    {
      candidate_score += top_score * CLASS_BONUS_RATIO;
    }

    if candidate_score >= threshold {
      return true;
    }

    if element.value().name() == "p" {
      Self::is_valid_paragraph(document, child_id)
    } else {
      false
    }
  }

  fn top_candidates(candidates: &HashMap<NodeId, Candidate>) -> Vec<NodeId> {
    let mut ranked = candidates
      .values()
      .map(|candidate| (candidate.node, candidate.score))
      .collect::<Vec<_>>();

    ranked.sort_by(|a, b| {
      b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
    });

    ranked
      .into_iter()
      .take(DEFAULT_TOP_CANDIDATES)
      .map(|(node, _)| node)
      .collect()
  }
}
