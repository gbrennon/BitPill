#!/usr/bin/env bash
set -euo pipefail

# Ensure cargo bin dir is on PATH
export PATH="$HOME/.cargo/bin:$PATH"

# Install rustup if missing
if ! command -v rustup >/dev/null 2>&1; then
  echo "Installing rustup..."
  curl https://sh.rustup.rs -sSf | sh -s -- -y
fi

export PATH="$HOME/.cargo/bin:$PATH"

# Ensure stable toolchain and developer components
rustup default stable || true
rustup component add rustfmt clippy || true

# Install just (task runner) if missing
if ! command -v just >/dev/null 2>&1; then
  echo "Installing just..."
  cargo install just --locked || true
fi

# Install cargo-llvm-cov for coverage reporting
if ! command -v cargo-llvm-cov >/dev/null 2>&1; then
  echo "Installing cargo-llvm-cov..."
  cargo install cargo-llvm-cov --locked || true
fi

# Display versions for debugging
rustc --version || true
cargo --version || true
just --version || true
cargo-llvm-cov --version || true
