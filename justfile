# BitPill — task runner
# Requires: cargo, cargo-llvm-cov, rustfmt, clippy

# Default: check formatting, lint, then run tests with coverage
default: fmt-check lint test

# Build the project
build:
    cargo build

# Run the REST API server (default)
run:
    cargo run --bin rest

# Run the terminal UI
run-tui:
    cargo run --bin tui

# Run both the REST server (background thread) and the TUI (foreground)
run-both:
    cargo run --bin app

# Run all tests with coverage
test:
    cargo llvm-cov --ignore-filename-regex "ports/fakes"

# Run a single test by name substring
test-one NAME:
    cargo test {{NAME}}

# Lint (zero warnings enforced)
lint:
    cargo clippy -- -D warnings

# Format all source files
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt --check

# Install required dev tools
tools:
    rustup component add rustfmt clippy
    cargo install cargo-llvm-cov --locked

# Remove build artifacts
clean:
    cargo clean
