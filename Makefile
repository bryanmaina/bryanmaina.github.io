# Makefile for Bryan Maina's Developer Blog

# Variables
TAILWIND_INPUT = ./style/tailwind.css
TAILWIND_OUTPUT = ./style.css
TRUNK_VERSION = 0.21.13 # Or specify the version you need

.PHONY: help install dev watch-tailwind serve build build-tailwind test clean

# Default target: Show help
help:
	@echo "Available commands:"
	@echo "  make install        Install necessary tools (Trunk) and dependencies (npm, concurrently)"
	@echo "  make dev            Run Tailwind watch and Trunk serve concurrently for development (requires concurrently)"
	@echo "  make watch-tailwind Run Tailwind CSS in watch mode"
	@echo "  make serve          Run the Trunk development server"
	@echo "  make build          Build the project for production (includes Tailwind)"
	@echo "  make build-tailwind Build Tailwind CSS for production"
	@echo "  make test           Run tests"
	@echo "  make clean          Remove build artifacts"

# Install dependencies
install:
	@echo "Installing Trunk..."
	cargo install trunk --locked --version $(TRUNK_VERSION)
	@echo "Installing Node.js packages (including concurrently)..."
	npm install
	@echo "Installation complete."

# Run development environment (Tailwind watch + Trunk serve) concurrently
# Requires 'concurrently' to be installed (npm install --save-dev concurrently)
dev:
	@echo "Starting development environment (Tailwind watch + Trunk serve)..."
	npx concurrently --kill-others "make watch-tailwind" "make serve"

# Run Tailwind CSS in watch mode
watch-tailwind:
	@echo "Starting Tailwind CSS in watch mode..."
	npx @tailwindcss/cli@latest -i $(TAILWIND_INPUT) -o $(TAILWIND_OUTPUT) --watch

# Run Trunk development server
serve:
	@echo "Starting Trunk development server..."
	trunk serve

# Build the project for production
build: build-tailwind
	@echo "Building project with Trunk for production..."
	trunk build --release
	@echo "Production build complete in ./dist directory."

# Build Tailwind CSS for production
build-tailwind:
	@echo "Building Tailwind CSS for production..."
	npx @tailwindcss/cli@latest -i $(TAILWIND_INPUT) -o $(TAILWIND_OUTPUT) --minify

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	rm -rf ./dist
	rm -f $(TAILWIND_OUTPUT)
	cargo clean
	@echo "Clean complete."

