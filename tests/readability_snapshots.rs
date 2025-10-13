use {
  readability::{Readability, ReadabilityOptions},
  std::{
    fs,
    path::{Path, PathBuf},
  },
};

fn load_fixture(path: &Path) -> anyhow::Result<String> {
  Ok(fs::read_to_string(path.join("source.html"))?)
}

fn run_fixture(name: &str, path: PathBuf) -> anyhow::Result<()> {
  let html = load_fixture(&path)?;

  let mut parser =
    Readability::new(&html, None, ReadabilityOptions::default())?;

  let article = parser.parse()?;

  insta::with_settings!({ snapshot_suffix => name }, {
    insta::assert_json_snapshot!("article", &article);
  });

  Ok(())
}

fn fixture_dirs() -> Vec<(String, PathBuf)> {
  let mut fixtures = Vec::new();

  let root = PathBuf::from("tests/fixtures");

  if root.exists() {
    for entry in fs::read_dir(&root).expect("fixtures directory") {
      let entry = entry.expect("fixture entry");

      if entry.file_type().map(|ty| ty.is_dir()).unwrap_or(false) {
        let path = entry.path();

        if path.join("source.html").exists() {
          let name = entry.file_name().to_string_lossy().to_string();
          fixtures.push((name, path));
        }
      }
    }
  }

  if let Ok(env_root) = std::env::var("READABILITY_TEST_ROOT") {
    let env_path = PathBuf::from(env_root);

    if env_path.exists() {
      for entry in walkdir::WalkDir::new(env_path)
        .max_depth(2)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
      {
        let dir = entry.path().to_path_buf();

        if dir.join("source.html").exists() {
          let name = dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

          fixtures.push((name, dir));
        }
      }
    }
  }

  fixtures
}

#[test]
fn readability_snapshots() -> anyhow::Result<()> {
  let fixtures = fixture_dirs();

  if fixtures.is_empty() {
    return Ok(());
  }

  for (name, path) in fixtures {
    run_fixture(&name, path)?;
  }

  Ok(())
}
