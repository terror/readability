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

    let parser = unclutter::Parser::new(&html, None, ParserOptions::default())?;

    writeln!(stdout, "{}", parser.parse()?.content)?;

    Ok(())
  }

  pub(crate) fn run(self) -> Result {
    self.execute(io::stdin().lock(), io::stdout().lock())
  }
}

#[cfg(test)]
mod tests {
  use {super::*, clap::error::ErrorKind};

  #[test]
  fn reads_stdin() {
    let mut stdout = Vec::new();

    Arguments::try_parse_from(["unclutter"])
      .unwrap()
      .execute("<p>foo</p>".as_bytes(), &mut stdout)
      .unwrap();

    assert_eq!(
      String::from_utf8(stdout).unwrap(),
      "<div id=\"readability-page-1\" class=\"page\"><p>foo</p></div>\n",
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
