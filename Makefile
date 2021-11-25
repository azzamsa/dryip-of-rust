help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

verify_light: ## Run most important repository requirement
	cargo fmt
	cargo clippy --locked --all-targets

verify: ## Run all the repository requirement before making a commit
	cargo fmt
	cargo clippy --locked --all-targets
	cargo test --all-targets

is_verified:  ## Check if the repository complies with the requirement in CI.
	cargo fmt -- --check
	cargo clippy  --locked --all-targets
	cargo test --workspace --all-targets


.PHONY: help verify_light verify  is_verified
.DEFAULT_GOAL := help
