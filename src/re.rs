use super::*;

macro_rules! re {
  ($pattern:literal) => {
    LazyLock::new(|| Regex::new($pattern).unwrap())
  };
}

pub(crate) static BYLINE: LazyLock<Regex> =
  re!(r"(?i)byline|author|dateline|writtenby|p-author");

pub(crate) static MAYBE_CANDIDATE: LazyLock<Regex> =
  re!(r"(?i)and|article|body|column|content|main|mathjax|shadow");

pub(crate) static META_PROPERTY: LazyLock<Regex> = re!(
  r"(?i)\s*(article|dc|dcterm|og|twitter)\s*:\s*(author|creator|description|published_time|title|site_name)\s*"
);

pub(crate) static NUMERIC_HTML_ENTITY: LazyLock<Regex> =
  re!(r"(?i)&#(?:x([0-9a-f]+)|([0-9]+));");

pub(crate) static TITLE_HIERARCHICAL_SEPARATOR: LazyLock<Regex> =
  re!(r"\s[\\/>»]\s");

pub(crate) static TITLE_LEADING_JUNK: LazyLock<Regex> =
  re!(r"(?i)^[^|\-–—\/>»]*[|\-–—\/>»]");

pub(crate) static TITLE_NORMALIZE_WHITESPACE: LazyLock<Regex> = re!(r"\s{2,}");

pub(crate) static TITLE_SEPARATOR: LazyLock<Regex> = re!(r"\s[|\-–—\/>»]\s");

pub(crate) static UNLIKELY_CANDIDATE: LazyLock<Regex> = re!(
  r"(?i)-ad-|ai2html|banner|breadcrumbs|combx|comment|community|cover-wrap|disqus|extra|footer|gdpr|header|legends|menu|related|remark|replies|rss|shoutbox|sidebar|skyscraper|social|sponsor|supplemental|ad-break|agegate|pagination|pager|popup|yom-remote"
);
