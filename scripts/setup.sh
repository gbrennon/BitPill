#!/usr/bin/env bash
set -euo pipefail

# Ensure cargo bin dir is on PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Install rustup if missing
if ! command -v rustup >/dev/null 2>&1; then
  echo "Installing rustup..."
  tmp_dir="$(mktemp -d)"
  trap 'rm -rf "$tmp_dir"' EXIT
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o "$tmp_dir/rustup-init.sh"
  sh "$tmp_dir/rustup-init.sh" -y
fi

# Ensure stable toolchain and developer components
# Source cargo env if present (rustup installer writes this file).
if [[ -f "$HOME/.cargo/env" ]]; then
  # shellcheck disable=SC1090
  source "$HOME/.cargo/env" || true
fi

if ! command -v rustup >/dev/null 2>&1; then
  echo "rustup not found after install. Please check output above." >&2
  exit 1
fi

# Use non-interactive default stable toolchain and add components
rustup default stable --no-modify-path || rustup default stable
rustup component add rustfmt clippy || true

# Install just (task runner) if missing
# Always install in CI to avoid stale cache issues
if [[ -n "${CI:-}" ]] || ! command -v just >/dev/null 2>&1; then
  echo "Installing just..."
  cargo install just --locked || {
    echo "cargo install just failed" >&2
    exit 1
  }
fi

# Install cargo-llvm-cov for coverage reporting
# Always install in CI to avoid stale cache issues
if [[ -n "${CI:-}" ]] || ! command -v cargo-llvm-cov >/dev/null 2>&1; then
  echo "Installing cargo-llvm-cov..."
  cargo install cargo-llvm-cov --locked || {
    echo "cargo install cargo-llvm-cov failed" >&2
    exit 1
  }
fi

# Display versions for debugging
if command -v rustc >/dev/null 2>&1; then
  rustc --version
else
  echo "rustc not found"
fi
if command -v cargo >/dev/null 2>&1; then
  cargo --version
else
  echo "cargo not found"
fi
if command -v just >/dev/null 2>&1; then
  just --version
  just tools
else
  echo "just not found"
fi
if command -v cargo-llvm-cov >/dev/null 2>&1; then
  cargo-llvm-cov --version
else
  echo "cargo-llvm-cov not found"
fi
