use {
  pretty_assertions::assert_eq,
  readability::{Readability, ReadabilityOptions},
  serde::{Deserialize, Serialize},
  std::{fs, path::PathBuf},
};

macro_rules! test {
  ($name:expr) => {
    paste::paste! {
      #[test]
      fn [<test_ $name>]() {
        let fixture = TestFixture::load($name);
        fixture.run_test();
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

  fn normalize_html(html: &str) -> String {
    html
      .lines()
      .map(str::trim)
      .filter(|line| !line.is_empty())
      .collect::<Vec<_>>()
      .join("\n")
  }

  fn run_test(&self) {
    let mut readability = Readability::new(
      &self.source_html,
      Some("http://fakehost/"),
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

    let (expected_normalized, actual_normalized) = (
      Self::normalize_html(&self.expected_html),
      Self::normalize_html(&article.content),
    );

    assert_eq!(
      actual_normalized, expected_normalized,
      "HTML content mismatch"
    );
  }
}

test!("001");
test!("002");
test!("003-metadata-preferred");
test!("004-metadata-space-separated-properties");
test!("005-unescape-html-entities");
