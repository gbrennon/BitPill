# BitPill — task runner
# Requires: cargo, cargo-llvm-cov, rustfmt, clippy

# Default: check formatting, lint, then run tests with coverage
default: fmt-check lint coverage

# Build the project
build:
    cargo build

# Run the application
run:
    cargo run

# Run all tests
test:
    cargo test

# Run tests with HTML coverage report (opens in browser)
coverage:
    cargo llvm-cov --html
    @echo "Coverage report: target/llvm-cov/html/index.html"

# Run tests with coverage summary in terminal
coverage-summary:
    cargo llvm-cov

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

# Remove build artifacts
clean:
    cargo clean
