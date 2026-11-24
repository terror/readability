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

[group: 'dev']
build-wasm:
  ./bin/build-wasm

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

[group: 'web']
[working-directory: 'www']
web-build: build-wasm
  bun run build

[group: 'web']
[working-directory: 'www']
web-dev: build-wasm
  bun run dev

[group: 'web']
[working-directory: 'www']
web-fmt:
  bun run format

[group: 'web']
[working-directory: 'www']
web-install:
  bun install

[group: 'web']
[working-directory: 'www']
web-storybook:
  bun run storybook
