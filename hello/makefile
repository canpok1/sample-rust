SOURCE = main.rs
BINARY = main

.PHONY: build
build: ## Build binary from source
	@rustc $(SOURCE) -o $(BINARY)

.PHONY: build_on_docker
build_by_docker: ## Build binary from source by docker container
	@docker run --rm --user "$$(id -u)":"$$(id -g)" -v "$$PWD":/usr/src/myapp -w /usr/src/myapp rust:latest make build

.PHONY: run
run: build ## Run binary
	@date "+%Y-%m-%d %H:%M:%S"
	@./$(BINARY)
	@date "+%Y-%m-%d %H:%M:%S"

.PHONY: run_on_docker
run_by_docker: ## Run binary by docker container
	@docker run --rm --user "$$(id -u)":"$$(id -g)" -v "$$PWD":/usr/src/myapp -w /usr/src/myapp rust:latest make run

.PHONY: clean
clean: ## Remote all binary
	@rm -f $(BINARY)

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s \033[0m%s\n", $$1, $$2}'
