SHELL := /bin/bash
.DEFAULT_GOAL := help

help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository.
	git cliff --version || cargo install --locked git-cliff
	cargo nextest --version || cargo install --locked cargo-nextest
	cargo-set-version --help || cargo install --locked cargo-edit
	cargo watch --version || cargo install --locked cargo-watch
	cargo outdated --version || cargo install --locked cargo-outdated
	cargo tarpaulin --version || cargo install --locked cargo-tarpaulin
	cargo udeps --version || cargo install --locked cargo-udeps
	dprint --version || cargo install --locked dprint

dev: # Develop the app.
	cargo watch -x clippy

fmt: ## Format the codebase.
	cargo fmt --all
	dprint fmt --config configs/dprint.json

fmt_check: ## Check is the codebase properly formatted.
	cargo fmt --all -- --check
	dprint check --config configs/dprint.json

lint_doc: ## Lint the docstring.
	cargo doc --all-features --no-deps

lint: lint_doc ## Lint the codebase.
	cargo clippy

test: ## Test the codebase.
	cargo test --doc
	cargo nextest run

comply: fmt lint test ## Tasks to make the code-base comply with the rules. Mostly used in git hooks.

check: fmt_check lint test ## Check if the repository comply with the rules and ready to be pushed.

release: ## Create a new release. Example `make release version=v2.2.0`
	bash scripts/release.sh $(version)

#
# Misc
#

check_dependencies: ## Check dependencies health.
	cargo +nightly udeps
	cargo outdated --root-deps-only
