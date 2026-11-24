use super::*;

#[derive(Parser)]
#[command(name = "unclutter", version)]
#[command(about = "Extract readable content from HTML files", long_about = None)]
pub(crate) struct Arguments {
  #[arg(
    value_name = "FILE",
    help = "Path to the HTML file to parse, read from standard input if omitted"
  )]
  input: Option<PathBuf>,
  #[arg(long, help = "Print the whole article as JSON")]
  json: bool,
  #[arg(long, help = "URL of the page, used to resolve relative links")]
  url: Option<String>,
}

impl Arguments {
  fn execute(self, mut stdin: impl Read, mut stdout: impl Write) -> Result {
    let html = if let Some(path) = &self.input {
      fs::read_to_string(path)?
    } else {
      let mut html = String::new();
      stdin.read_to_string(&mut html)?;
      html
    };

    let parser = unclutter::Parser::new(
      &html,
      self.url.as_deref(),
      ParserOptions::default(),
    )?;

    let article = parser.parse()?;

    if self.json {
      serde_json::to_writer(&mut stdout, &article)?;
      writeln!(stdout)?;
    } else {
      writeln!(stdout, "{}", article.content)?;
    }

    Ok(())
  }

  pub(crate) fn run(self) -> Result {
    self.execute(io::stdin().lock(), io::stdout().lock())
  }
}

#[cfg(test)]
mod tests {
  use {super::*, clap::error::ErrorKind};

  #[track_caller]
  fn run(arguments: &[&str], stdin: &str) -> String {
    let mut stdout = Vec::new();

    Arguments::try_parse_from(arguments)
      .unwrap()
      .execute(stdin.as_bytes(), &mut stdout)
      .unwrap();

    String::from_utf8(stdout).unwrap()
  }

  #[test]
  fn prints_json() {
    assert_eq!(
      run(&["unclutter", "--json"], "<p>foo</p>"),
      concat!(
        r#"{"byline":null,"content":"<div id=\"readability-page-1\" class=\"page\"><p>foo</p></div>","#,
        r#""dir":null,"excerpt":"foo","lang":null,"length":3,"published_time":null,"#,
        r#""site_name":null,"text_content":"foo","title":""}"#,
        "\n",
      ),
    );
  }

  #[test]
  fn reads_stdin() {
    assert_eq!(
      run(&["unclutter"], "<p>foo</p>"),
      "<div id=\"readability-page-1\" class=\"page\"><p>foo</p></div>\n",
    );
  }

  #[test]
  fn resolves_relative_links() {
    assert_eq!(
      run(
        &["unclutter", "--url", "http://foo.com/bar/"],
        r#"<p>foo<img src="baz.png"></p>"#,
      ),
      "<div id=\"readability-page-1\" class=\"page\"><p>foo<img src=\"http://foo.com/bar/baz.png\"></p></div>\n",
    );
  }

  #[test]
  fn version() {
    let Err(error) = Arguments::try_parse_from(["unclutter", "--version"])
    else {
      panic!("expected version error");
    };

    assert_eq!(error.kind(), ErrorKind::DisplayVersion);

    assert_eq!(
      error.to_string(),
      format!("unclutter {}\n", env!("CARGO_PKG_VERSION")),
    );
  }
}
