# BitPill — task runner
# Requires: cargo, cargo-llvm-cov, rustfmt, clippy

# Default: check formatting, lint, then run tests with coverage
default: fmt-check lint test

# Build the project
build:
    cargo build

# Run the terminal UI
run:
    cargo run --release

# Run all tests with coverage
test:
    ./scripts/check_coverage.sh

# Run tests matching a specific path or name filter
# Examples:
#   just test-path src/application/services/create_medication_service.rs
#   just test-path services::create_medication
#   just test-path create_medication
test-path filter:
    ./scripts/test_path.sh "{{filter}}"

# Lint (zero warnings enforced)
lint:
    cargo clippy -- -D warnings

# Lint fixes (optionally specify files)
lint-fix +files='':
    cargo clippy --fix --allow-dirty --allow-staged {{files}}

# Format source files (optionally specify files)
fmt *files:
    cargo +nightly fmt {{ files }}

# Check formatting without modifying files
fmt-check:
    cargo +nightly fmt --check

# Install required dev tools
tools:
    rustup component add rustfmt clippy
    cargo install cargo-llvm-cov --locked --force

# Install lefthook for pre-commit hooks
install-hooks:
    pipx install lefthook
    lefthook install

# Validate workflow files statically (requires actionlint)
lint-workflows:
    actionlint -config-file .actionlint.yaml .forgejo/workflows/*.yml

# Remove build artifacts
clean:
    cargo clean

# Install the CLI tool globally
install:
    cargo install --path . --locked

set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

release *args:
    ./scripts/release.sh {{args}}

release-dry *args:
    ./scripts/release.sh --dry-run {{args}}

release-alpha *args:
    ./scripts/release.sh --pre-release alpha {{args}}

release-beta *args:
    ./scripts/release.sh --pre-release beta {{args}}

release-rc *args:
    ./scripts/release.sh --pre-release rc {{args}}
