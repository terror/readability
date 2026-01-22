use super::*;

#[derive(Parser)]
#[command(name = "readability")]
#[command(about = "Extract readable content from HTML files", long_about = None)]
pub(crate) struct Arguments {
  #[arg(value_name = "FILE", help = "Path to the HTML file to parse")]
  input: PathBuf,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    let html = fs::read_to_string(&self.input)?;

    let mut readability =
      Readability::new(&html, None, ReadabilityOptions::default())?;

    println!("{}", readability.parse()?.content);

    Ok(())
  }
}
