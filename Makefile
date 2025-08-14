# ThreatFlux Hashing Library Makefile
# Provides consistent build, test, and quality commands across all ThreatFlux libraries

.PHONY: help build test check fmt clippy clean doc bench examples install release pre-commit all

# Default target
all: fmt clippy test build

# Help target
help:
	@echo "ThreatFlux Hashing Library - Available targets:"
	@echo ""
	@echo "  Build targets:"
	@echo "    build          - Build the library in debug mode"
	@echo "    release        - Build the library in release mode"
	@echo "    check          - Fast compilation check without optimization"
	@echo ""
	@echo "  Quality targets:"
	@echo "    fmt            - Format code with rustfmt"
	@echo "    clippy         - Run clippy lints"
	@echo "    test           - Run all tests"
	@echo "    bench          - Run benchmarks"
	@echo "    doc            - Generate documentation"
	@echo ""
	@echo "  Maintenance targets:"
	@echo "    clean          - Clean build artifacts"
	@echo "    install        - Install from source"
	@echo "    examples       - Run all examples"
	@echo "    pre-commit     - Run pre-commit checks (fmt + clippy + test)"
	@echo ""
	@echo "  Meta targets:"
	@echo "    all            - Run fmt + clippy + test + build"
	@echo "    help           - Show this help message"

# Build targets
build:
	@echo "🔨 Building threatflux-hashing..."
	cargo build

release:
	@echo "🚀 Building threatflux-hashing in release mode..."
	cargo build --release

check:
	@echo "✅ Checking threatflux-hashing compilation..."
	cargo check

# Quality targets
fmt:
	@echo "🎨 Formatting threatflux-hashing code..."
	cargo fmt

clippy:
	@echo "📎 Running clippy on threatflux-hashing..."
	cargo clippy -- -D warnings

test:
	@echo "🧪 Running threatflux-hashing tests..."
	cargo test

bench:
	@echo "⚡ Running threatflux-hashing benchmarks..."
	cargo bench

doc:
	@echo "📚 Generating threatflux-hashing documentation..."
	cargo doc --no-deps --open

# Maintenance targets
clean:
	@echo "🧹 Cleaning threatflux-hashing build artifacts..."
	cargo clean

install:
	@echo "📦 Installing threatflux-hashing..."
	cargo install --path .

examples:
	@echo "💡 Running threatflux-hashing examples..."
	@for example in $$(cargo run --example 2>&1 | grep -E "^\s+" | awk '{print $$1}'); do \
		echo "Running example: $$example"; \
		cargo run --example $$example; \
	done

# Pre-commit checks
pre-commit: fmt clippy test
	@echo "✅ All pre-commit checks passed for threatflux-hashing!"

# Development workflow
dev: fmt clippy test build
	@echo "🎯 Development cycle complete for threatflux-hashing!"