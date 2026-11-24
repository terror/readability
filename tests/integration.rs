#![allow(unused)]

use {
  assert_html_eq::assert_html_eq,
  pretty_assertions::assert_eq,
  serde::{Deserialize, Serialize},
  std::{fs, path::PathBuf},
  unclutter::{Parser, ParserOptions},
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

  fn parse_article(&self) -> unclutter::Article {
    self.parser().parse().expect("Failed to parse article")
  }

  fn parser(&self) -> Parser {
    Parser::new(
      &self.source_html,
      Some("http://fakehost/test/page.html"),
      ParserOptions::builder()
        .classes_to_preserve(["caption"])
        .build(),
    )
    .unwrap()
  }

  fn test_metadata(&self) {
    assert_eq!(
      self.parser().is_readable(),
      self.expected_metadata.readerable,
      "Readerable mismatch"
    );

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
    assert_html_eq!(self.parse_article().content, self.expected_html.clone());
  }
}

test!("001");
test!("002");
test!("003-metadata-preferred");
test!("004-metadata-space-separated-properties");
test!("005-unescape-html-entities");
test!("aclu");
test!("aktualne");
test!("archive-of-our-own");
test!("ars-1");
test!("article-author-tag");
test!("base-url");
test!("base-url-base-element");
test!("base-url-base-element-relative");
test!("basic-tags-cleaning");
test!("bbc-1");
test!("blogger");
test!("breitbart");
test!("bug-1255978");
test!("buzzfeed-1");
test!("citylab-1");
test!("clean-links");
test!("cnet");
test!("cnet-svg-classes");
test!("cnn");
test!("comment-inside-script-parsing");
test!("daringfireball-1");
test!("data-url-image");
test!("dev418");
test!("dropbox-blog");
test!("ebb-org");
test!("ehow-1");
test!("ehow-2");
test!("embedded-videos");
test!("engadget");
test!("firefox-nightly-blog");
test!("folha");
test!("gitlab-blog");
test!("gmw");
test!("google-sre-book-1");
test!("guardian-1");
test!("heise");
test!("herald-sun-1");
test!("hidden-nodes");
test!("hukumusume");
test!("iab-1");
test!("ietf-1");
test!("invalid-attributes");
test!("js-link-replacement");
test!("keep-images");
test!("keep-tabular-data");
test!("la-nacion");
test!("lazy-image-1");
test!("lazy-image-2");
test!("lazy-image-3");
test!("lemonde-1");
test!("liberation-1");
test!("lifehacker-post-comment-load");
test!("lifehacker-working");
test!("links-in-tables");
test!("lwn-1");
test!("mathjax");
test!("medicalnewstoday");
test!("medium-1");
test!("medium-2");
test!("medium-3");
test!("mercurial");
test!("metadata-content-missing");
test!("missing-paragraphs");
test!("mozilla-1");
test!("mozilla-2");
test!("msn");
test!("normalize-spaces");
test!("nytimes-1");
test!("nytimes-2");
test!("nytimes-3");
test!("nytimes-4");
test!("nytimes-5");
test!("ol");
test!("parsely-metadata");
test!("pixnet");
test!("qq");
test!("quanta-1");
test!("remove-aria-hidden");
test!("remove-extra-brs");
test!("remove-extra-paragraphs");
test!("remove-script-tags");
test!("reordering-paragraphs");
test!("replace-brs");
test!("replace-font-tags");
test!("royal-road");
test!("rtl-1");
test!("rtl-2");
test!("rtl-3");
test!("rtl-4");
test!("salon-1");
test!("schema-org-context-object");
test!("seattletimes-1");
test!("simplyfound-1");
test!("social-buttons");
test!("spiceworks");
test!("style-tags-removal");
test!("svg-parsing");
test!("table-style-attributes");
test!("telegraph");
test!("theverge");
test!("title-and-h1-discrepancy");
test!("title-en-dash");
test!("tmz-1");
test!("toc-missing");
test!("topicseed-1");
test!("tumblr");
test!("v8-blog");
test!("videos-1");
test!("videos-2");
test!("visibility-hidden");
test!("wapo-1");
test!("wapo-2");
test!("webmd-1");
test!("webmd-2");
test!("wikia");
test!("wikipedia");
test!("wikipedia-2");
test!("wikipedia-3");
test!("wikipedia-4");
test!("wordpress");
test!("yahoo-1");
test!("yahoo-2");
test!("yahoo-3");
test!("yahoo-4");
test!("youth");
