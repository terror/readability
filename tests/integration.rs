use {
  pretty_assertions::assert_eq,
  readability::{Readability, ReadabilityOptions},
  scraper::Html,
  serde::{Deserialize, Serialize},
  std::{fs, panic, path::PathBuf},
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
    .unwrap();

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

    let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
      assert_html_eq(
        &Html::parse_document(&self.expected_html),
        &Html::parse_document(&article.content),
      );
    }));

    if result.is_err() {
      eprintln!("Raw HTML Diff:");
      assert_eq!(article.content, self.expected_html, "HTML content mismatch");
    }
  }
}

fn assert_html_eq(actual: &Html, expected: &Html) {
  use {
    ego_tree::NodeRef,
    scraper::{ElementRef, Node},
  };

  fn compare_elements(elem1: &ElementRef, elem2: &ElementRef) {
    assert_eq!(
      elem1.value().name(),
      elem2.value().name(),
      "Tag name mismatch"
    );

    let mut attrs1 = elem1.value().attrs().collect::<Vec<_>>();
    let mut attrs2 = elem2.value().attrs().collect::<Vec<_>>();

    attrs1.sort_by_key(|a| a.0);
    attrs2.sort_by_key(|a| a.0);

    assert_eq!(
      attrs1,
      attrs2,
      "Attributes mismatch on <{}>",
      elem1.value().name()
    );

    let children1 = elem1
      .children()
      .filter(|n| !is_whitespace_text(n))
      .collect::<Vec<_>>();

    let children2 = elem2
      .children()
      .filter(|n| !is_whitespace_text(n))
      .collect::<Vec<_>>();

    assert_eq!(
      children1.len(),
      children2.len(),
      "Different number of children in <{}>",
      elem1.value().name()
    );

    for (child1, child2) in children1.iter().zip(children2.iter()) {
      match (child1.value(), child2.value()) {
        (Node::Element(_), Node::Element(_)) => {
          compare_elements(
            &ElementRef::wrap(*child1).unwrap(),
            &ElementRef::wrap(*child2).unwrap(),
          );
        }
        (Node::Text(t1), Node::Text(t2)) => {
          assert_eq!(
            normalize_whitespace(t1),
            normalize_whitespace(t2),
            "Text content mismatch in <{}>",
            elem1.value().name()
          );
        }
        _ => {
          panic!(
            "Node type mismatch in <{}>: {:?} vs {:?}",
            elem1.value().name(),
            child1.value(),
            child2.value()
          );
        }
      }
    }
  }

  fn is_whitespace_text(node: &NodeRef<Node>) -> bool {
    matches!(node.value(), Node::Text(t) if t.trim().is_empty())
  }

  fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
  }

  compare_elements(&actual.root_element(), &expected.root_element());
}

test!("001");
test!("002");
test!("003-metadata-preferred");
test!("004-metadata-space-separated-properties");
test!("005-unescape-html-entities");
test!("aclu");
test!("base-url");
test!("basic-tags-cleaning");
test!("clean-links");
test!("comment-inside-script-parsing");
test!("js-link-replacement");
test!("lazy-image-3");
test!("metadata-content-missing");
test!("normalize-spaces");
test!("ol");
test!("remove-script-tags");
test!("rtl-1");
test!("rtl-2");
test!("rtl-3");
test!("rtl-4");
test!("salon-1");
test!("style-tags-removal");
test!("title-and-h1-discrepancy");
test!("title-en-dash");
