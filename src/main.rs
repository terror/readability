use {
  anyhow::Context,
  clap::Parser,
  readability::{Readability, ReadabilityOptions},
  std::{fs, path::PathBuf, process},
};

#[derive(Parser)]
#[command(name = "readability")]
#[command(about = "Extract readable content from HTML files", long_about = None)]
struct Arguments {
  /// Path to the HTML file to parse
  #[arg(value_name = "FILE")]
  input: PathBuf,
}

impl Arguments {
  fn run(self) -> Result {
    let html = fs::read_to_string(&self.input).with_context(|| {
      format!("failed to read file from `{}`", self.input.display())
    })?;

    let mut readability =
      Readability::new(&html, None, ReadabilityOptions::default())
        .context("failed to create readability parser")?;

    let article = readability
      .parse()
      .context("failed to parse article content")?;

    println!("{}", article.content);

    Ok(())
  }
}

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
