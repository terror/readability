use super::*;

fn re(pattern: &str) -> Regex {
  Regex::new(pattern).unwrap()
}

lazy_static! {
  pub(crate) static ref BASE64_DATA_URL: Regex =
    re(r"^data:\s*([^\s;,]+)\s*;\s*base64\s*,");
  pub(crate) static ref BYLINE_HINTS: Regex =
    re(r"(?i)byline|author|dateline|writtenby|p-author");
  pub(crate) static ref COMMENT_SECTION_HINT: Regex =
    re(r"(?i)comment|comments|discussion|discuss|respond|reply|talkback");
  pub(crate) static ref COMMA_VARIANTS: Regex = re(r"[,،﹐︐﹑⹀⸲，]");
  pub(crate) static ref FRAGMENT_URL: Regex = re(r"^#.+");
  pub(crate) static ref IMAGE_EXTENSION_SUFFIX: Regex =
    re(r"(?i)\.(?:jpg|jpeg|png|webp)");
  pub(crate) static ref IMG_MISSING_SELF_CLOSING: Regex =
    re(r"<img([^>]*[^/])>");
  pub(crate) static ref LAZY_IMAGE_SRC_VALUE: Regex =
    re(r"(?i)^\s*\S+\.(?:jpg|jpeg|png|webp)\S*\s*$");
  pub(crate) static ref NAMED_HTML_ENTITIES: Regex =
    re(r"&(?P<name>quot|amp|apos|lt|gt);");
  pub(crate) static ref NEGATIVE_CONTENT_HINTS: Regex = re(concat!(
    r"(?i)-ad-|hidden|^hid$| hid$| hid |^hid |banner|combx|comment|com-|",
    r"contact|footer|gdpr|masthead|media|meta|outbrain|promo|related|scroll|",
    r"share|shoutbox|sidebar|skyscraper|sponsor|shopping|tags|widget"
  ));
  pub(crate) static ref NUMERIC_HTML_ENTITY: Regex =
    re(r"&#(?:x([0-9a-fA-F]+)|([0-9]+));");
  pub(crate) static ref POSSIBLE_CONTENT_CANDIDATE: Regex =
    re(r"(?i)and|article|body|column|content|main|mathjax|shadow");
  pub(crate) static ref POSITIVE_CONTENT_HINTS: Regex = re(concat!(
    r"(?i)article|body|content|entry|hentry|h-entry|main|page|pagination|",
    r"post|text|blog|story"
  ));
  pub(crate) static ref SRCSET_CANDIDATE_VALUE: Regex =
    re(r"(?i)\.(?:jpg|jpeg|png|webp)\s+\d");
  pub(crate) static ref TITLE_HIERARCHICAL_SEPARATOR: Regex =
    re(r"\s[\\\/>»]\s");
  pub(crate) static ref TITLE_LEADING_SEPARATOR: Regex =
    re(r"^[^\|\-–—\\\/>»]*[\|\-–—\\\/>»]");
  pub(crate) static ref TITLE_SEPARATOR_RUN: Regex = re(r"\s[\|\-–—\\\/>»]\s");
  pub(crate) static ref TOKEN_BOUNDARY: Regex = re(r"\W+");
  pub(crate) static ref UNLIKELY_CONTENT_CANDIDATES: Regex = re(concat!(
    r"(?i)-ad-|ai2html|banner|breadcrumbs|combx|comment|community|cover-wrap|",
    r"disqus|extra|footer|gdpr|header|legends|menu|related|remark|replies|rss|",
    r"shoutbox|sidebar|skyscraper|social|sponsor|supplemental|ad-break|agegate|",
    r"pagination|pager|popup|yom-remote"
  ));
  pub(crate) static ref WHITESPACE_RUNS: Regex = re(r"\s{2,}");
}
