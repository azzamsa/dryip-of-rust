#!/usr/bin/env -S just --justfile

alias d := dev
alias t := test

# List available commands.
default:
    just --list --unsorted

# Setup the repository.
setup:
    git cliff --version || cargo install --locked git-cliff
    cargo nextest --version || cargo install --locked cargo-nextest
    cargo-set-version --help || cargo install --locked cargo-edit
    cargo watch --version || cargo install --locked cargo-watch
    cargo outdated --version || cargo install --locked cargo-outdated
    cargo tarpaulin --version || cargo install --locked cargo-tarpaulin
    cargo udeps --version || cargo install --locked cargo-udeps
    dprint --version || cargo install --locked dprint

# Develop the app.
dev pattern:
    cargo watch -x 'test -- --nocapture --test {{ pattern }}'

# Format the codebase.
fmt:
    cargo fmt --all

# Check is the codebase properly formatted.
fmt_check:
    cargo fmt --all -- --check

# Lint the docstring.
lint_doc:
    cargo doc --all-features --no-deps

# Lint the codebase.
lint: lint_doc
    cargo clippy

# Test the codebase.
test:
    cargo test --doc
    cargo nextest run

# Tasks to make the code-base comply with the rules. Mostly used in git hooks.
comply: fmt lint test

# Check if the repository comply with the rules and ready to be pushed.
check: fmt_check lint test

# Open documentation.
doc:
    cargo doc --open

# Check dependencies health.
up:
    cargo +nightly udeps
    cargo outdated --root-deps-only

# Local Variables:
# mode: makefile
# End:
# vim: set ft=make :
