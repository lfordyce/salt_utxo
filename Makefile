VERSION=`git rev-parse HEAD`
BUILD=`date +%FT%T%z`

.PHONY: help
help: ## - Show help message
	@printf "\033[32m\xE2\x9c\x93 usage: make [target]\n\n\033[0m"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: docker-pull
docker-pull:	## - docker pull latest images
	@printf "\033[32m\xE2\x9c\x93 docker pull latest images\n\033[0m"
	@docker pull rust:latest

.PHONY: build
build:docker-pull	## - Build the rust based utxo docker image
	@printf "\033[32m\xE2\x9c\x93 Build the rust based utxo docker image image\n\033[0m"
	$(eval BUILDER_IMAGE=$(shell docker inspect --format='{{index .RepoDigests 0}}' rust:latest))
	@export DOCKER_CONTENT_TRUST=1
	@docker build -f Dockerfile --build-arg "BUILDER_IMAGE=$(BUILDER_IMAGE)" -t salt/utxo_rust:latest .

.PHONY: compose
compose:build	## - Run docker compose up
	@docker-compose -f docker-compose.yaml up

.PHONY: style-check
style-check:	## - Run Rust formatter on project
	@printf "\033[32m\xE2\x9c\x93 Run Rust formatter on project\n\033[0m"
	@rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

.PHONY: lint
lint:	## - Run Rust clippy for linting
	@printf "\033[32m\xE2\x9c\x93 Run Rust clippy for linting\n\033[0m"
	@rustup component add clippy 2> /dev/null
	@cargo clippy --all-targets --all-features -- -D warnings

.PHONY: scan
scan:	## - Scan for known vulnerabilities
	@printf "\033[32m\xE2\x9c\x93 Scan for known vulnerabilities \n\033[0m"
	@docker scan -f Dockerfile salt/utxo_rust:latest