#!/usr/bin/env bash
set -euo pipefail

export PATH="$HOME/.cargo/bin:$PATH"

install_os_dependencies() {
  apt-get upgrade && apt-get install -y jq
}

install_rustup_if_missing() {
  if command -v rustup >/dev/null 2>&1; then
    return
  fi
  echo "Installing rustup..."
  local tmp_dir
  tmp_dir="$(mktemp -d)"
  trap 'rm -rf "$tmp_dir"' EXIT
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o "$tmp_dir/rustup-init.sh"
  sh "$tmp_dir/rustup-init.sh" -y
}

source_cargo_env_if_present() {
  if [[ -f "$HOME/.cargo/env" ]]; then
    # shellcheck disable=SC1090
    source "$HOME/.cargo/env" || true
  fi
}

abort_if_rustup_unavailable() {
  if ! command -v rustup >/dev/null 2>&1; then
    echo "rustup not found after install. Please check output above." >&2
    exit 1
  fi
}

configure_rust_stable_toolchain_with_dev_components() {
  rustup default stable --no-modify-path || rustup default stable
  rustup component add rustfmt clippy || true
}

install_just_task_runner() {
  if [[ -z "${CI:-}" ]] && command -v just >/dev/null 2>&1; then
    return
  fi
  echo "Installing just..."
  cargo install just --locked || {
    echo "cargo install just failed" >&2
    exit 1
  }
}

install_cargo_llvm_cov_for_coverage() {
  if [[ -z "${CI:-}" ]] && command -v cargo-llvm-cov >/dev/null 2>&1; then
    return
  fi
  echo "Installing cargo-llvm-cov..."
  cargo install cargo-llvm-cov --locked || {
    echo "cargo install cargo-llvm-cov failed" >&2
    exit 1
  }
}

print_installed_tool_versions() {
  if command -v rustc >/dev/null 2>&1; then rustc --version; else echo "rustc not found"; fi
  if command -v cargo >/dev/null 2>&1; then cargo --version; else echo "cargo not found"; fi
  if command -v just >/dev/null 2>&1; then just --version && just tools; else echo "just not found"; fi
  if command -v cargo-llvm-cov >/dev/null 2>&1; then cargo llvm-cov --version; else echo "cargo-llvm-cov not found"; fi
}

install_rustup_if_missing
source_cargo_env_if_present
abort_if_rustup_unavailable
configure_rust_stable_toolchain_with_dev_components
install_just_task_runner
install_cargo_llvm_cov_for_coverage
print_installed_tool_versions
