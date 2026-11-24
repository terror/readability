use super::*;

const ALTERNATIVE_SCORE_RATIO: f64 = 0.75;

const MIN_ALTERNATIVE_CANDIDATES: usize = 3;

pub(crate) struct SelectTopCandidate;

impl SelectTopCandidate {
  fn common_ancestor(
    context: &Context<'_>,
    top_candidate: NodeId,
    top_candidates: &[NodeId],
  ) -> NodeId {
    let score = context.extraction.scores[&top_candidate];

    let alternative_ancestors = top_candidates
      .iter()
      .skip(1)
      .filter(|id| {
        context.extraction.scores[id] / score >= ALTERNATIVE_SCORE_RATIO
      })
      .map(|id| {
        context
          .node(*id)
          .ancestors(None)
          .iter()
          .map(|node| node.id)
          .collect::<Vec<_>>()
      })
      .collect::<Vec<_>>();

    if alternative_ancestors.len() < MIN_ALTERNATIVE_CANDIDATES {
      return top_candidate;
    }

    let mut parent = context.node(top_candidate).parent();

    while let Some(node) = parent.filter(|node| node.tag() != "BODY") {
      let lists = alternative_ancestors
        .iter()
        .filter(|ancestors| ancestors.contains(&node.id))
        .count();

      if lists >= MIN_ALTERNATIVE_CANDIDATES {
        return node.id;
      }

      parent = node.parent();
    }

    top_candidate
  }

  fn create_top_candidate(context: &mut Context<'_>) {
    let Some(body) = context.document.select("body").nodes().first().cloned()
    else {
      return;
    };

    let div = create_element(&context.document.tree, "div");

    while let Some(child) = body.first_child() {
      div.append_child(&child);
    }

    body.append_child(&div);

    let score = div.initial_score(context.flags.weight_classes);

    let id = div.id;

    context.extraction.scores.insert(id, score);
    context.extraction.created_top_candidate = true;
    context.extraction.top_candidate = Some(id);
  }

  fn higher_scoring_ancestor(
    context: &Context<'_>,
    top_candidate: NodeId,
  ) -> NodeId {
    let mut last_score = context.extraction.scores[&top_candidate];

    let threshold = last_score / 3.0;

    let mut parent = context.node(top_candidate).parent();

    while let Some(node) = parent.filter(|node| node.tag() != "BODY") {
      parent = node.parent();

      let Some(score) = context.extraction.scores.get(&node.id).copied() else {
        continue;
      };

      if score < threshold {
        break;
      }

      if score > last_score {
        return node.id;
      }

      last_score = score;
    }

    top_candidate
  }

  fn initialize(context: &mut Context<'_>, id: NodeId) {
    if !context.extraction.scores.contains_key(&id) {
      let score = context.node(id).initial_score(context.flags.weight_classes);
      context.extraction.scores.insert(id, score);
    }
  }

  fn only_child_ancestor(
    context: &Context<'_>,
    top_candidate: NodeId,
  ) -> NodeId {
    let mut top_candidate = context.node(top_candidate);

    while let Some(parent) = top_candidate.parent().filter(|parent| {
      parent.tag() != "BODY" && parent.element_children().len() == 1
    }) {
      top_candidate = parent;
    }

    top_candidate.id
  }

  fn top_candidates(context: &mut Context<'_>) -> Vec<NodeId> {
    let limit = context.options.n_top_candidates;

    let mut top_candidates = Vec::<NodeId>::new();

    for id in context.extraction.candidates.clone() {
      let score = context.extraction.scores[&id]
        * (1.0 - context.node(id).link_density());

      context.extraction.scores.insert(id, score);

      if let Some(index) = (0..limit).find(|index| {
        top_candidates
          .get(*index)
          .is_none_or(|other| score > context.extraction.scores[other])
      }) {
        top_candidates.insert(index, id);
        top_candidates.truncate(limit);
      }
    }

    top_candidates
  }
}

impl Stage for SelectTopCandidate {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let top_candidates = Self::top_candidates(context);

    let top_candidate = top_candidates
      .first()
      .copied()
      .filter(|id| context.node(*id).tag() != "BODY");

    let Some(top_candidate) = top_candidate else {
      Self::create_top_candidate(context);
      return Ok(());
    };

    let top_candidate =
      Self::common_ancestor(context, top_candidate, &top_candidates);

    Self::initialize(context, top_candidate);

    let top_candidate = Self::higher_scoring_ancestor(context, top_candidate);

    let top_candidate = Self::only_child_ancestor(context, top_candidate);

    Self::initialize(context, top_candidate);

    context.extraction.top_candidate = Some(top_candidate);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn top_candidate(html: &str) -> String {
    let mut document = dom_query::Document::from(html);

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    ScoreCandidates.run(&mut context).unwrap();
    SelectTopCandidate.run(&mut context).unwrap();

    context
      .node(context.extraction.top_candidate.unwrap())
      .html()
      .to_string()
  }

  #[test]
  fn selects_highest_scoring_candidate() {
    let paragraph = format!("<p>{}</p>", "foo, ".repeat(10));

    assert_eq!(
      top_candidate(&format!(
        r#"<html><body><div id="foo">{paragraph}{paragraph}</div><div id="bar"><span></span>{paragraph}</div></body></html>"#
      )),
      format!(r#"<div id="foo">{paragraph}{paragraph}</div>"#),
    );
  }

  #[test]
  fn creates_top_candidate_without_candidates() {
    let mut document = dom_query::Document::from(
      "<html><body>foo<span>bar</span></body></html>",
    );

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    SelectTopCandidate.run(&mut context).unwrap();

    assert!(context.extraction.created_top_candidate);

    assert_eq!(
      context.document.html().to_string(),
      "<html><head></head><body><div>foo<span>bar</span></div></body></html>",
    );
  }

  #[test]
  fn climbs_to_only_child_parent() {
    let paragraph = format!("<p>{}</p>", "foo, ".repeat(10));

    assert_eq!(
      top_candidate(&format!(
        "<html><body><section><div>{paragraph}</div></section><span></span></body></html>"
      )),
      format!("<section><div>{paragraph}</div></section>"),
    );
  }
}
