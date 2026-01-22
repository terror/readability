set dotenv-load

export EDITOR := 'nvim'

alias f := fmt
alias t := test

default:
  just --list

ci: fmt-check clippy test

[group: 'check']
clippy:
  cargo clippy

[group: 'format']
fmt:
  cargo fmt

[group: 'check']
fmt-check:
  cargo fmt -- --check

[group: 'test']
test:
  cargo test

[group: 'dev']
watch +COMMAND='test':
  cargo watch --clear --exec "{{ COMMAND }}"
