use {
  assert_html_eq::assert_html_eq,
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

    assert_html_eq!(article.content, self.expected_html.to_string());
  }
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
test!("guardian-1");
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
