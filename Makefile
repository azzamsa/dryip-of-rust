SHELL := /bin/bash

help:
    # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository
	cargo install git-cliff cargo-tarpaulin

verify_light: ## Run most important repository requirement
	cargo fmt
	cargo clippy

verify: ## Run all the repository requirement before making a commit
	cargo fmt
	cargo clippy
	cargo test
	cargo tarpaulin --ignore-tests

is_verified:  ## Check if the repository complies with the requirement in CI.
	cargo fmt -- --check
	cargo clippy
	cargo test

release:  ## Create a release
	bash scripts/release.sh $(version)


.PHONY: help verify_light verify is_verified release setup
.DEFAULT_GOAL := help
