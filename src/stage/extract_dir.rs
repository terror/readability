use super::*;

pub(crate) struct ExtractDir;

impl Stage for ExtractDir {
  fn run(&mut self, context: &mut Context<'_>) -> Result {
    let (Some(parent), Some(top_candidate)) = (
      context.extraction.top_candidate_parent,
      context.extraction.top_candidate,
    ) else {
      return Ok(());
    };

    let parent = context.node(parent);

    let dir = [parent.clone(), context.node(top_candidate)]
      .into_iter()
      .chain(parent.ancestors(None))
      .filter(NodeRef::is_element)
      .find_map(|node| node.attr("dir").filter(|dir| !dir.is_empty()))
      .map(|dir| dir.to_string());

    context.dir = dir;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[track_caller]
  fn dir(html: &str) -> Option<String> {
    let mut document = dom_query::Document::from(html);

    let options = ReadabilityOptions::default();

    let mut context = Context::new(&mut document, &options);

    ScoreCandidates.run(&mut context).unwrap();
    SelectTopCandidate.run(&mut context).unwrap();
    ExtractArticle.run(&mut context).unwrap();
    ExtractDir.run(&mut context).unwrap();

    context.dir
  }

  #[test]
  fn extracts_dir_from_ancestors() {
    #[track_caller]
    fn case(html: &str, expected: Option<&str>) {
      assert_eq!(dir(html).as_deref(), expected);
    }

    let paragraph = format!("<p>{}</p>", "foo, ".repeat(10));

    case(
      &format!(
        r#"<html dir="rtl"><body><div>{paragraph}</div><span></span></body></html>"#
      ),
      Some("rtl"),
    );

    case(
      &format!(
        r#"<html dir="ltr"><body dir="rtl"><div>{paragraph}</div><span></span></body></html>"#
      ),
      Some("rtl"),
    );

    case(
      &format!(
        r#"<html><body><div dir="rtl">{paragraph}</div><span></span></body></html>"#
      ),
      Some("rtl"),
    );

    case(
      &format!(
        r#"<html dir=""><body><div>{paragraph}</div><span></span></body></html>"#
      ),
      None,
    );
  }
}
