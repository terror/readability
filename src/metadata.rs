#[derive(Debug, Clone, Default)]
pub(crate) struct Metadata {
  pub(crate) byline: Option<String>,
  pub(crate) excerpt: Option<String>,
  pub(crate) published_time: Option<String>,
  pub(crate) site_name: Option<String>,
  pub(crate) title: Option<String>,
}
