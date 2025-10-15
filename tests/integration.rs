use {
  pretty_assertions::assert_eq,
  readability::{Readability, ReadabilityOptions},
  scraper::Html,
  serde::{Deserialize, Serialize},
  std::{fs, path::PathBuf},
};

macro_rules! test {
  ($name:expr) => {
    paste::paste! {
      #[test]
      fn [<test_ $name>]() {
        TestFixture::load($name).run();
      }
    }
  };
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ExpectedMetadata {
  byline: Option<String>,
  dir: Option<String>,
  excerpt: Option<String>,
  lang: Option<String>,
  published_time: Option<String>,
  #[serde(default)]
  readerable: bool,
  site_name: Option<String>,
  title: String,
}

struct TestFixture {
  expected_html: String,
  expected_metadata: ExpectedMetadata,
  source_html: String,
}

impl TestFixture {
  fn load(name: &str) -> Self {
    let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
      .join("submodules/readability/test/test-pages")
      .join(name);
    let source_html = fs::read_to_string(base_path.join("source.html"))
      .expect("Failed to read source.html");
    let expected_html = fs::read_to_string(base_path.join("expected.html"))
      .expect("Failed to read expected.html");
    let expected_metadata_str =
      fs::read_to_string(base_path.join("expected-metadata.json"))
        .expect("Failed to read expected-metadata.json");
    let expected_metadata: ExpectedMetadata =
      serde_json::from_str(&expected_metadata_str)
        .expect("Failed to parse metadata JSON");
    Self {
      expected_html,
      expected_metadata,
      source_html,
    }
  }

  fn run(&self) {
    let mut readability = Readability::new(
      &self.source_html,
      Some("http://fakehost/test/page.html"),
      ReadabilityOptions::default(),
    )
    .expect("Failed to create Readability instance");
    let article = readability.parse().expect("Failed to parse article");

    assert_eq!(
      article.title, self.expected_metadata.title,
      "Title mismatch"
    );
    assert_eq!(
      article.byline, self.expected_metadata.byline,
      "Byline mismatch"
    );
    assert_eq!(article.dir, self.expected_metadata.dir, "Dir mismatch");
    assert_eq!(article.lang, self.expected_metadata.lang, "Lang mismatch");
    assert_eq!(
      article.excerpt, self.expected_metadata.excerpt,
      "Excerpt mismatch"
    );
    assert_eq!(
      article.site_name, self.expected_metadata.site_name,
      "Site name mismatch"
    );
    assert_eq!(
      article.published_time, self.expected_metadata.published_time,
      "Published time mismatch"
    );

    let expected_html = Html::parse_document(&self.expected_html);
    let actual_html = Html::parse_document(&article.content);

    assert_html_eq(&actual_html, &expected_html, "HTML content mismatch");
  }
}

fn assert_html_eq(actual: &Html, expected: &Html, message: &str) {
  use scraper::{ElementRef, Node};

  fn compare_elements(
    elem1: &ElementRef,
    elem2: &ElementRef,
  ) -> Result<(), String> {
    // Compare tag names
    if elem1.value().name() != elem2.value().name() {
      return Err(format!(
        "Tag name mismatch: '{}' vs '{}'",
        elem1.value().name(),
        elem2.value().name()
      ));
    }

    // Compare attributes (order-independent)
    let mut attrs1: Vec<_> = elem1.value().attrs().collect();
    let mut attrs2: Vec<_> = elem2.value().attrs().collect();
    attrs1.sort_by_key(|a| a.0);
    attrs2.sort_by_key(|a| a.0);

    if attrs1 != attrs2 {
      return Err(format!(
        "Attributes mismatch on <{}>: {:?} vs {:?}",
        elem1.value().name(),
        attrs1,
        attrs2
      ));
    }

    // Get non-whitespace children
    let children1: Vec<_> = elem1
      .children()
      .filter(|n| !is_whitespace_text(n))
      .collect();
    let children2: Vec<_> = elem2
      .children()
      .filter(|n| !is_whitespace_text(n))
      .collect();

    if children1.len() != children2.len() {
      return Err(format!(
        "Different number of children in <{}>: {} vs {}",
        elem1.value().name(),
        children1.len(),
        children2.len()
      ));
    }

    // Compare each child
    for (child1, child2) in children1.iter().zip(children2.iter()) {
      match (child1.value(), child2.value()) {
        (Node::Element(_), Node::Element(_)) => {
          let elem_ref1 = ElementRef::wrap(*child1).unwrap();
          let elem_ref2 = ElementRef::wrap(*child2).unwrap();
          compare_elements(&elem_ref1, &elem_ref2)?;
        }
        (Node::Text(t1), Node::Text(t2)) => {
          let text1 = normalize_whitespace(t1);
          let text2 = normalize_whitespace(t2);
          if text1 != text2 {
            return Err(format!(
              "Text content mismatch in <{}>: '{}' vs '{}'",
              elem1.value().name(),
              text1,
              text2
            ));
          }
        }
        _ => {
          return Err(format!(
            "Node type mismatch in <{}>",
            elem1.value().name()
          ));
        }
      }
    }

    Ok(())
  }

  fn is_whitespace_text(node: &ego_tree::NodeRef<Node>) -> bool {
    matches!(node.value(), Node::Text(t) if t.trim().is_empty())
  }

  fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
  }

  if let Err(diff) =
    compare_elements(&actual.root_element(), &expected.root_element())
  {
    panic!("{}: {}", message, diff);
  }
}

test!("001");
test!("002");
test!("003-metadata-preferred");
test!("004-metadata-space-separated-properties");
test!("005-unescape-html-entities");
test!("base-url");
test!("basic-tags-cleaning");
test!("clean-links");
test!("comment-inside-script-parsing");
test!("metadata-content-missing");
test!("normalize-spaces");
test!("remove-script-tags");
test!("rtl-1");
test!("rtl-2");
test!("rtl-3");
test!("rtl-4");
test!("salon-1");
test!("style-tags-removal");
test!("title-and-h1-discrepancy");
test!("title-en-dash");
