use super::*;

pub(crate) struct Context<'a> {
  pub(crate) article: Option<NodeId>,
  pub(crate) candidates: Vec<NodeId>,
  pub(crate) created_top_candidate: bool,
  pub(crate) data_tables: HashSet<NodeId>,
  pub(crate) dir: Option<String>,
  pub(crate) document: &'a mut dom_query::Document,
  pub(crate) flags: Flags,
  pub(crate) lang: Option<String>,
  pub(crate) metadata: Metadata,
  pub(crate) options: &'a ReadabilityOptions,
  pub(crate) scores: HashMap<NodeId, f64>,
  pub(crate) top_candidate: Option<NodeId>,
  pub(crate) top_candidate_parent: Option<NodeId>,
  pub(crate) url: Option<Url>,
}

impl<'a> Context<'a> {
  pub(crate) fn document(&mut self) -> Document<'_> {
    Document::new(&mut *self.document)
  }

  pub(crate) fn new(
    html: &'a mut dom_query::Document,
    options: &'a ReadabilityOptions,
  ) -> Self {
    Self {
      article: None,
      candidates: Vec::new(),
      created_top_candidate: false,
      data_tables: HashSet::new(),
      dir: None,
      document: html,
      flags: Flags::default(),
      lang: None,
      metadata: Metadata::default(),
      options,
      scores: HashMap::new(),
      top_candidate: None,
      top_candidate_parent: None,
      url: None,
    }
  }

  pub(crate) fn node(&self, id: NodeId) -> NodeRef<'_> {
    self.document.tree.get_unchecked(&id)
  }

  pub(crate) fn options(&self) -> &ReadabilityOptions {
    self.options
  }
}
