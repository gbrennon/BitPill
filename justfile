# BitPill — task runner
# Requires: cargo, cargo-llvm-cov, rustfmt, clippy

# Default: check formatting, lint, then run tests with coverage
default: fmt-check lint test

# Build the project
build:
    cargo build

# Run the REST API server
run-api:
    cargo run --bin bitpill -- api

# Run the terminal UI
run-tui:
    cargo run --bin bitpill -- tui

# Run both the REST server (background) and the TUI (foreground)
run:
    cargo run --bin bitpill

# Run all tests with coverage
test:
    ./scripts/check_coverage.sh

# Lint (zero warnings enforced)
lint:
    cargo clippy -- -D warnings

# Lint fixes (automatically apply suggestions)
lint-fix:
    cargo clippy --fix --allow-dirty --allow-staged

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

# Validate workflow files statically (requires actionlint)
lint-workflows:
    actionlint .forgejo/workflows/*.yml

# Remove build artifacts
clean:
    cargo clean

# Install the CLI tool globally
install:
    cargo install --path . --locked
