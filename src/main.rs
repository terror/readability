use {
  anyhow::anyhow,
  arguments::Arguments,
  clap::Parser,
  decode::decode,
  encoding_rs::{Encoding, WINDOWS_1252},
  regex::bytes::Regex,
  std::{
    backtrace::BacktraceStatus,
    borrow::Cow,
    fs,
    io::{self, Read, Write},
    path::PathBuf,
    process,
    sync::LazyLock,
  },
  unclutter::ParserOptions,
};

mod arguments;
mod decode;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");

    for (i, error) in error.chain().skip(1).enumerate() {
      if i == 0 {
        eprintln!();
        eprintln!("because:");
      }

      eprintln!("- {error}");
    }

    let backtrace = error.backtrace();

    if backtrace.status() == BacktraceStatus::Captured {
      eprintln!("backtrace:");
      eprintln!("{backtrace}");
    }

    process::exit(1);
  }
}
