#!/usr/bin/env bash
# shellcheck disable=SC1091
set -euo pipefail

# Source common functions
source "$(dirname "$0")/lib/common.sh"

install_os_dependencies() {
  apt-get upgrade && apt-get install -y jq
}

install_just_task_runner() {
  install_cargo_tool_if_missing "just" "cargo install just --locked --force"
}

install_cargo_llvm_cov_for_coverage() {
  install_cargo_tool_if_missing "cargo-llvm-cov" "cargo install cargo-llvm-cov --locked"
}

install_os_dependencies
ensure_cargo_in_path
install_rustup_if_missing
source_cargo_env_if_present
abort_if_rustup_unavailable
configure_rust_stable_toolchain_with_dev_components
install_just_task_runner
install_cargo_llvm_cov_for_coverage
print_installed_tool_versions
