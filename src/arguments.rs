use super::*;

#[derive(Parser)]
#[command(name = "unclutter")]
#[command(about = "Extract readable content from HTML files", long_about = None)]
pub(crate) struct Arguments {
  #[arg(value_name = "FILE", help = "Path to the HTML file to parse")]
  input: PathBuf,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    let html = fs::read_to_string(&self.input)?;

    let mut parser =
      unclutter::Parser::new(&html, None, ParserOptions::default())?;

    println!("{}", parser.parse()?.content);

    Ok(())
  }
}
