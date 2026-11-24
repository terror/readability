set dotenv-load

export EDITOR := 'nvim'

alias f := fmt
alias t := test

default:
  just --list

ci: fmt-check clippy test

[group: 'dev']
bench:
  cargo bench

[group: 'check']
clippy:
  cargo clippy

[group: 'format']
fmt:
  cargo fmt

[group: 'check']
fmt-check:
  cargo fmt -- --check

[group: 'release']
publish:
  ./bin/publish

[group: 'test']
test:
  cargo test

[group: 'test']
test-release-workflow:
  -git tag -d test-release
  -git push origin :test-release
  git tag test-release
  git push origin test-release

[group: 'release']
update-changelog:
  echo >> CHANGELOG.md
  git log --pretty='format:- %s' >> CHANGELOG.md

[group: 'dev']
watch +COMMAND='test':
  cargo watch --clear --exec "{{ COMMAND }}"
