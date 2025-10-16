use {
  assert_html_eq::assert_html_eq,
  pretty_assertions::assert_eq,
  readability::{Readability, ReadabilityOptions},
  serde::{Deserialize, Serialize},
  std::{fs, path::PathBuf},
};

macro_rules! test_metadata {
  ($name:expr) => {
    paste::paste! {
      #[test]
      fn [<test_metadata_ $name>]() {
        TestFixture::load($name).test_metadata();
      }
    }
  };
}

macro_rules! test_output {
  ($name:expr) => {
    paste::paste! {
      #[test]
      fn [<test_output_ $name>]() {
        TestFixture::load($name).test_output();
      }
    }
  };
}

macro_rules! test {
  ($name:expr) => {
    test_metadata!($name);
    test_output!($name);
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

  fn parse_article(&self) -> readability::Article {
    let mut readability = Readability::new(
      &self.source_html,
      Some("http://fakehost/test/page.html"),
      ReadabilityOptions::default(),
    )
    .unwrap();

    readability.parse().expect("Failed to parse article")
  }

  fn test_metadata(&self) {
    let article = self.parse_article();

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
  }

  fn test_output(&self) {
    assert_html_eq!(
      self.parse_article().content,
      self.expected_html.to_string()
    );
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

test_metadata!("aktualne");
test_metadata!("archive-of-our-own");
test_metadata!("ars-1");
test_metadata!("article-author-tag");
test_metadata!("base-url-base-element");
test_metadata!("base-url-base-element-relative");
test_metadata!("bug-1255978");
test_metadata!("citylab-1");
test_metadata!("cnet");
test_metadata!("cnet-svg-classes");
test_metadata!("cnn");
test_metadata!("daringfireball-1");
test_metadata!("data-url-image");
test_metadata!("dev418");
test_metadata!("dropbox-blog");
test_metadata!("ebb-org");
test_metadata!("ehow-1");
test_metadata!("ehow-2");
test_metadata!("embedded-videos");
test_metadata!("engadget");
test_metadata!("firefox-nightly-blog");
test_metadata!("folha");
test_metadata!("gitlab-blog");
test_metadata!("gmw");
test_metadata!("heise");
test_metadata!("hidden-nodes");
test_metadata!("ietf-1");
test_metadata!("invalid-attributes");
test_metadata!("keep-images");
test_metadata!("lazy-image-1");
test_metadata!("lazy-image-2");
test_metadata!("lemonde-1");
test_metadata!("lifehacker-post-comment-load");
test_metadata!("lifehacker-working");
test_metadata!("mathjax");
test_metadata!("medicalnewstoday");
test_metadata!("medium-1");
test_metadata!("medium-2");
test_metadata!("medium-3");
test_metadata!("mercurial");
test_metadata!("missing-paragraphs");
test_metadata!("mozilla-2");
test_metadata!("msn");
test_metadata!("nytimes-1");
test_metadata!("nytimes-2");
test_metadata!("nytimes-3");
test_metadata!("nytimes-4");
test_metadata!("nytimes-5");
test_metadata!("pixnet");
test_metadata!("qq");
test_metadata!("remove-extra-brs");
test_metadata!("remove-extra-paragraphs");
test_metadata!("reordering-paragraphs");
test_metadata!("royal-road");
test_metadata!("schema-org-context-object");
test_metadata!("seattletimes-1");
test_metadata!("social-buttons");
test_metadata!("spiceworks");
test_metadata!("svg-parsing");
test_metadata!("theverge");
test_metadata!("tmz-1");
test_metadata!("toc-missing");
test_metadata!("topicseed-1");
test_metadata!("v8-blog");
test_metadata!("videos-1");
test_metadata!("videos-2");
test_metadata!("wapo-1");
test_metadata!("wapo-2");
test_metadata!("yahoo-4");
test_metadata!("youth");

test_output!("parsely-metadata");
test_output!("replace-font-tags");
test_output!("wikia");
