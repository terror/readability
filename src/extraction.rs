use super::*;

#[derive(Default)]
pub(crate) struct Extraction {
  pub(crate) article: Option<NodeId>,
  pub(crate) candidates: Vec<NodeId>,
  pub(crate) created_top_candidate: bool,
  pub(crate) data_tables: HashSet<NodeId>,
  pub(crate) scores: HashMap<NodeId, f64>,
  pub(crate) top_candidate: Option<NodeId>,
  pub(crate) top_candidate_parent: Option<NodeId>,
}
