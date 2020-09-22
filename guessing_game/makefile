BINARY = target/debug/guessing_game

.PHONY: build
build: ## Build binary from source
	@cargo build

.PHONY: build_on_docker
build_by_docker: ## Build binary from source by docker container
	@docker run --rm --user "$$(id -u)":"$$(id -g)" -v "$$PWD":/usr/src/myapp -w /usr/src/myapp rust:latest make build

.PHONY: run
run: build ## Run binary
	@echo [`date "+%Y-%m-%d %H:%M:%S"`] start program
	@./$(BINARY)
	@echo [`date "+%Y-%m-%d %H:%M:%S"`] end program

.PHONY: run_on_docker
run_by_docker: ## Run binary by docker container
	@docker run --rm --user "$$(id -u)":"$$(id -g)" -v "$$PWD":/usr/src/myapp -w /usr/src/myapp -i rust:latest make run

.PHONY: clean
clean: ## Remote all binary
	@cargo clean

.PHONY: clean_by_docker
clean_by_docker: ## Remote all binary by docker container
	@docker run --rm --user "$$(id -u)":"$$(id -g)" -v "$$PWD":/usr/src/myapp -w /usr/src/myapp rust:latest make clean

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s \033[0m%s\n", $$1, $$2}'
