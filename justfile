#!/usr/bin/env -S just --justfile

set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

_default:
  @just --list -u

alias r := ready

[unix]
_clean_dist:
  rm -rf packages/*/dist

[windows]
_clean_dist:
  Remove-Item -Path 'packages/*/dist' -Recurse -Force -ErrorAction SilentlyContinue

# Install bun if not already installed (Unix)
[unix]
_bun_install:
  #!/usr/bin/env bash
  set -e
  if ! command -v bun &> /dev/null; then
    curl -fsSL https://bun.sh/install | bash
    export PATH="$HOME/.bun/bin:$PATH"
  fi

# Install bun if not already installed (Windows)
[windows]
_bun_install:
  #!/usr/bin/env bash
  set -e
  if ! command -v bun &> /dev/null; then
    powershell -ExecutionPolicy Bypass -Command "irm bun.sh/install.win | iex"
  fi

# Install bun package dependencies (Unix)
[unix]
_bun_deps:
  #!/usr/bin/env bash
  set -e
  export PATH="$HOME/.bun/bin:$PATH"
  bun install --dir packages/bun

# Install bun package dependencies (Windows)
[windows]
_bun_deps:
  $env:Path = \"$env:USERPROFILE\\.bun\\bin;$env:Path\"
  bun install --dir packages/bun

init: _clean_dist _bun_install
  cargo binstall watchexec-cli cargo-insta typos-cli cargo-shear dprint taplo-cli -y
  node packages/tools/src/index.ts sync-remote
  pnpm install
  pnpm -C docs install
  just _bun_deps

# Build bun (requires bun runtime - takes significant time)
[unix]
build-bun:
  #!/usr/bin/env bash
  set -e
  export PATH="$HOME/.bun/bin:$PATH"
  cd packages/bun && bun bd

# Build bun (Windows)
[windows]
build-bun:
  $env:Path = \"$env:USERPROFILE\\.bun\\bin;$env:Path\"
  cd packages/bun
  bun bd

build:
  pnpm install
  pnpm --filter @rolldown/pluginutils build
  pnpm --filter rolldown build-binding:release
  pnpm --filter rolldown build-node
  pnpm --filter vite build-types
  pnpm --filter=@voidzero-dev/vite-plus-core build
  pnpm --filter=@voidzero-dev/vite-plus-test build
  pnpm --filter=@voidzero-dev/vite-plus-prompts build
  pnpm --filter=vite-plus build

ready:
  git diff --exit-code --quiet
  typos
  just fmt
  just check
  just test
  just lint
  just doc

watch *args='':
  watchexec --no-vcs-ignore {{args}}

fmt:
  cargo shear --fix
  cargo fmt --all
  pnpm fmt

check:
  cargo check --workspace --all-features --all-targets --locked

watch-check:
  just watch "'cargo check; cargo clippy'"

test:
  cargo test

lint:
  cargo clippy --workspace --all-targets --all-features -- --deny warnings

[unix]
doc:
  RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --document-private-items

[windows]
doc:
  $Env:RUSTDOCFLAGS='-D warnings'; cargo doc --no-deps --document-private-items
