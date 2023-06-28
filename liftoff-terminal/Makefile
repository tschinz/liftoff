.PHONY: info

APP_NAME = liftoff-terminal

#################################################
# Detect OS
##
ifeq ($(OS),Windows_NT)
detected_OS := Windows
else
detected_OS := $(shell uname)
endif

RUST_ENV := $$( rustup show )

#################################################
# OS Specifics
##
ifeq ($(detected_OS),Windows)
	PDFVIEWER = 'start "" /max'
endif

ifeq ($(detected_OS),Darwin)
	PDFVIEWER = open
endif

ifeq ($(detected_OS),Linux)
	PDFVIEWER = xdg-open
endif

#################################################
# COMMANDS
##
info: ## Information about the environment
	@echo "Environment Informations"
	@echo "  * Detected OS: $(detected_OS)"
	@echo "  * Pdfviewer: $(PDFVIEWER)"
	@echo "  * Rust Enviroment: "
	@echo "$(RUST_ENV)"

install-wasm: ## install wasm tool trunk
	@cargo install --locked trunk
	@rustup target add wasm32-unknown-unknown
run: ## build and run the debug application
	@cargo run

run-wasm: ## build an run the wasm locally
	@trunk serve

build-wasm: ## build wasm version
	@trunk build --release

build-debug: ## build debug version
	@cargo build

build-release: ## build release version
	@cargo build --release
	@mkdir -p ../bin && cp ../target/release/$(APP_NAME) ../bin/

run-release: ## run builded version
	@make build-release
	@bin/$(APP_NAME)
clean: ## clean project
	@cargo clean

help: ## Show this help
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; \
	{printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help