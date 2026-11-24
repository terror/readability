use {
  criterion::{Criterion, criterion_group, criterion_main},
  std::{fs, hint::black_box, path::PathBuf, time::Duration},
  unclutter::{Readability, ReadabilityOptions},
};

fn parse(criterion: &mut Criterion) {
  let mut paths = fs::read_dir("submodules/readability/test/test-pages")
    .unwrap()
    .map(|entry| entry.unwrap().path().join("source.html"))
    .collect::<Vec<PathBuf>>();

  paths.sort();

  let pages = paths
    .iter()
    .map(|path| fs::read_to_string(path).unwrap())
    .collect::<Vec<String>>();

  let mut group = criterion.benchmark_group("parse");

  group.measurement_time(Duration::from_secs(10));

  group.sample_size(10);

  group.bench_function("test-pages", |bencher| {
    bencher.iter(|| {
      for page in &pages {
        black_box(
          Readability::new(
            black_box(page),
            Some("http://fakehost/test/page.html"),
            ReadabilityOptions::default(),
          )
          .unwrap()
          .parse()
          .unwrap(),
        );
      }
    });
  });

  group.finish();
}

criterion_group!(benches, parse);
criterion_main!(benches);
