use super::*;

macro_rules! re {
  ($pattern:literal) => {
    LazyLock::new(|| Regex::new($pattern).unwrap())
  };
}

pub(crate) static AD_WORDS: LazyLock<Regex> = re!(
  r"(?i)^(ad(vertising|vertisement)?|pub(licité)?|werb(ung)?|广告|Реклама|Anuncio)$"
);

pub(crate) static B64_DATA_URL: LazyLock<Regex> =
  re!(r"(?i)^data:\s*([^\s;,]+)\s*;\s*base64\s*,");

pub(crate) static BYLINE: LazyLock<Regex> =
  re!(r"(?i)byline|author|dateline|writtenby|p-author");

pub(crate) static COMMAS: LazyLock<Regex> =
  re!(r"[,\u{060C}\u{FE50}\u{FE10}\u{FE11}\u{2E41}\u{2E34}\u{2E32}\u{FF0C}]");

pub(crate) static DISPLAY_NONE: LazyLock<Regex> =
  re!(r"(?i)(^|;)\s*display\s*:\s*none\s*(!important\s*)?(;|$)");

pub(crate) static HASH_URL: LazyLock<Regex> = re!(r"^#.+");

pub(crate) static HAS_CONTENT: LazyLock<Regex> = re!(r"\S$");

pub(crate) static IMAGE_EXTENSION: LazyLock<Regex> =
  re!(r"(?i)\.(jpg|jpeg|png|webp)");

pub(crate) static LAZY_IMAGE_SRC: LazyLock<Regex> =
  re!(r"^\s*\S+\.(jpg|jpeg|png|webp)\S*\s*$");

pub(crate) static LAZY_IMAGE_SRCSET: LazyLock<Regex> =
  re!(r"\.(jpg|jpeg|png|webp)\s+\d");

pub(crate) static LOADING_WORDS: LazyLock<Regex> =
  re!(r"(?i)^((loading|正在加载|Загрузка|chargement|cargando)(…|\.\.\.)?)$");

pub(crate) static MAYBE_CANDIDATE: LazyLock<Regex> =
  re!(r"(?i)and|article|body|column|content|main|mathjax|shadow");

pub(crate) static META_NAME: LazyLock<Regex> = re!(
  r"(?i)^\s*(?:(dc|dcterm|og|twitter|parsely|weibo:(article|webpage))\s*[-.:]\s*)?(author|creator|pub-date|description|title|site_name)\s*$"
);

pub(crate) static META_PROPERTY: LazyLock<Regex> = re!(
  r"(?i)\s*(article|dc|dcterm|og|twitter)\s*:\s*(author|creator|description|published_time|title|site_name)\s*"
);

pub(crate) static NAMED_HTML_ENTITY: LazyLock<Regex> =
  re!(r"&(quot|amp|apos|lt|gt);");

pub(crate) static NEGATIVE: LazyLock<Regex> = re!(
  r"(?i)-ad-|hidden|^hid$| hid$| hid |^hid |banner|combx|comment|com-|contact|footer|gdpr|masthead|media|meta|outbrain|promo|related|scroll|share|shoutbox|sidebar|skyscraper|sponsor|shopping|tags|widget"
);

pub(crate) static NORMALIZE_WHITESPACE: LazyLock<Regex> = re!(r"\s{2,}");

pub(crate) static NUMERIC_HTML_ENTITY: LazyLock<Regex> =
  re!(r"(?i)&#(?:x([0-9a-f]+)|([0-9]+));");

pub(crate) static POSITIVE: LazyLock<Regex> = re!(
  r"(?i)article|body|content|entry|hentry|h-entry|main|page|pagination|post|text|blog|story"
);

pub(crate) static SENTENCE_END: LazyLock<Regex> = re!(r"\.( |$)");

pub(crate) static SHARE_ELEMENTS: LazyLock<Regex> =
  re!(r"(?i)((?-u:\b)|_)(share|sharedaddy)((?-u:\b)|_)");

pub(crate) static SRCSET_URL: LazyLock<Regex> =
  re!(r"(\S+)(\s+[\d.]+[xw])?(\s*(?:,|$))");

pub(crate) static TITLE_HIERARCHICAL_SEPARATOR: LazyLock<Regex> =
  re!(r"\s[\\/>»]\s");

pub(crate) static TITLE_LEADING_JUNK: LazyLock<Regex> =
  re!(r"(?i)^[^|\-–—\\/>»]*[|\-–—\\/>»]");

pub(crate) static TITLE_SEPARATOR: LazyLock<Regex> = re!(r"\s[|\-–—\\/>»]\s");

pub(crate) static TITLE_WHITESPACE: LazyLock<Regex> = re!(r"[\t\n\x0C\r ]+");

pub(crate) static TITLE_WORD_SEPARATOR: LazyLock<Regex> = re!(r"\s+");

pub(crate) static TOKENIZE: LazyLock<Regex> = re!(r"[^A-Za-z0-9_]+");

pub(crate) static UNLIKELY_CANDIDATE: LazyLock<Regex> = re!(
  r"(?i)-ad-|ai2html|banner|breadcrumbs|combx|comment|community|cover-wrap|disqus|extra|footer|gdpr|header|legends|menu|related|remark|replies|rss|shoutbox|sidebar|skyscraper|social|sponsor|supplemental|ad-break|agegate|pagination|pager|popup|yom-remote"
);

pub(crate) static VIDEOS: LazyLock<Regex> = re!(
  r"(?i)//(www\.)?((dailymotion|youtube|youtube-nocookie|player\.vimeo|v\.qq|bilibili|live.bilibili)\.com|(archive|upload\.wikimedia)\.org|player\.twitch\.tv)"
);
