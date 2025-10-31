use super::*;

macro_rules! re {
  ($pat:expr) => {
    LazyLock::new(|| Regex::new($pat).unwrap())
  };
}

pub(crate) static BASE64_DATA_URL: LazyLock<Regex> =
  re!(r"^data:\s*([^\s;,]+)\s*;\s*base64\s*,");

pub(crate) static BYLINE_HINTS: LazyLock<Regex> =
  re!(r"(?i)byline|author|dateline|writtenby|p-author");

pub(crate) static COMMENT_SECTION_HINT: LazyLock<Regex> =
  re!(r"(?i)comment|comments|discussion|discuss|respond|reply|talkback");

pub(crate) static COMMA_VARIANTS: LazyLock<Regex> = re!(r"[,،﹐︐﹑⹀⸲，]");

pub(crate) static FRAGMENT_URL: LazyLock<Regex> = re!(r"^#.+");

pub(crate) static IMAGE_EXTENSION_SUFFIX: LazyLock<Regex> =
  re!(r"(?i)\.(?:jpg|jpeg|png|webp)");

pub(crate) static IMG_MISSING_SELF_CLOSING: LazyLock<Regex> =
  re!(r"<img([^>]*[^/])>");

pub(crate) static LAZY_IMAGE_SRC_VALUE: LazyLock<Regex> =
  re!(r"(?i)^\s*\S+\.(?:jpg|jpeg|png|webp)\S*\s*$");

pub(crate) static NAMED_HTML_ENTITIES: LazyLock<Regex> =
  re!(r"&(?P<name>quot|amp|apos|lt|gt);");

pub(crate) static NEGATIVE_CONTENT_HINTS: LazyLock<Regex> = re!(concat!(
  r"(?i)-ad-|hidden|^hid$| hid$| hid |^hid |banner|combx|comment|com-|",
  r"contact|footer|gdpr|masthead|media|meta|outbrain|promo|related|scroll|",
  r"share|shoutbox|sidebar|skyscraper|sponsor|shopping|tags|widget"
));

pub(crate) static NUMERIC_HTML_ENTITY: LazyLock<Regex> =
  re!(r"&#(?:x([0-9a-fA-F]+)|([0-9]+));");

pub(crate) static POSSIBLE_CONTENT_CANDIDATE: LazyLock<Regex> =
  re!(r"(?i)and|article|body|column|content|main|mathjax|shadow");

pub(crate) static POSITIVE_CONTENT_HINTS: LazyLock<Regex> = re!(concat!(
  r"(?i)article|body|content|entry|hentry|h-entry|main|page|pagination|",
  r"post|text|blog|story"
));

pub(crate) static SRCSET_CANDIDATE_VALUE: LazyLock<Regex> =
  re!(r"(?i)\.(?:jpg|jpeg|png|webp)\s+\d");

pub(crate) static TITLE_HIERARCHICAL_SEPARATOR: LazyLock<Regex> =
  re!(r"\s[\\\/>»]\s");

pub(crate) static TITLE_LEADING_SEPARATOR: LazyLock<Regex> =
  re!(r"^[^\|\-–—\\\/>»]*[\|\-–—\\\/>»]");

pub(crate) static TITLE_SEPARATOR_RUN: LazyLock<Regex> =
  re!(r"\s[\|\-–—\\\/>»]\s");

pub(crate) static TOKEN_BOUNDARY: LazyLock<Regex> = re!(r"\W+");

pub(crate) static UNLIKELY_CONTENT_CANDIDATES: LazyLock<Regex> = re!(concat!(
  r"(?i)-ad-|ai2html|banner|breadcrumbs|combx|comment|community|cover-wrap|",
  r"disqus|extra|footer|gdpr|header|legends|menu|related|remark|replies|rss|",
  r"shoutbox|sidebar|skyscraper|social|sponsor|supplemental|ad-break|agegate|",
  r"pagination|pager|popup|yom-remote"
));

pub(crate) static WHITESPACE_RUNS: LazyLock<Regex> = re!(r"\s{2,}");
