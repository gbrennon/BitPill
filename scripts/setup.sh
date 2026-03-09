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

export PATH="$HOME/.cargo/bin:$PATH"

# Ensure stable toolchain and developer components
rustup default stable
rustup component add rustfmt clippy

# Install just (task runner) if missing
if ! command -v just >/dev/null 2>&1; then
  echo "Installing just..."
  cargo install just --locked
fi

# Install cargo-llvm-cov for coverage reporting
if ! command -v cargo-llvm-cov >/dev/null 2>&1; then
  echo "Installing cargo-llvm-cov..."
  cargo install cargo-llvm-cov --locked
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
else
  echo "just not found"
fi
if command -v cargo-llvm-cov >/dev/null 2>&1; then
  cargo-llvm-cov --version
else
  echo "cargo-llvm-cov not found"
fi
