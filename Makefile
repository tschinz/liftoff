.PHONY: info

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

run-egui: ## build and run the liftoff-egui debug application
	@cargo run --package liftoff-egui

run-terminal: ## build and run the liftoff-terminal debug application
	@cargo run --package liftoff-terminal

run-ratatui: ## build and run the liftoff-ratatui debug application
	@cargo run --package liftoff-ratatui

build-release: ## build release versions of all applications
	@cargo build --release --package liftoff-egui
	@cargo build --release --package liftoff-terminal
	@cargo build --release --package liftoff-ratatui
	@mkdir -p bin && cp target/release/liftoff-egui bin/
	@mkdir -p bin && cp target/release/liftoff-terminal bin/
	@mkdir -p bin && cp target/release/liftoff-ratatui bin/

help: ## Show this help
	@grep -E '^[a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; \
	{printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.DEFAULT_GOAL := help