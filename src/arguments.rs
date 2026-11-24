use super::*;

#[derive(Parser)]
#[command(name = "unclutter", version)]
#[command(about = "Extract readable content from HTML files", long_about = None)]
pub(crate) struct Arguments {
  #[arg(value_name = "FILE", help = "Path to the HTML file to parse")]
  input: PathBuf,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    let html = fs::read_to_string(&self.input)?;

    let parser = unclutter::Parser::new(&html, None, ParserOptions::default())?;

    println!("{}", parser.parse()?.content);

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use {super::*, clap::error::ErrorKind};

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
