#!/usr/bin/env bash
# shellcheck disable=SC1091
# Run tests matching a given filter.
#
# The FILTER argument is passed directly to `cargo test`, so it matches against
# both test names (e.g. "services::create") and file-derived module paths
# (e.g. "application::services").  For a file path like
# "src/application/services/create_medication_service.rs" the script converts
# it to the equivalent module filter automatically.
#
# Usage:
#   ./scripts/test_path.sh <filter>
#   ./scripts/test_path.sh src/application/services/create_medication_service.rs
#   ./scripts/test_path.sh services::create
#   ./scripts/test_path.sh create_medication

set -euo pipefail

source "$(dirname "$0")/lib/common.sh"

usage() {
  echo "Usage: $(basename "$0") <filter>" >&2
  echo "" >&2
  echo "  filter  Test name, module path, or source file path." >&2
  echo "" >&2
  echo "Examples:" >&2
  echo "  $(basename "$0") services::create_medication" >&2
  echo "  $(basename "$0") src/application/services/create_medication_service.rs" >&2
  echo "  $(basename "$0") create_medication" >&2
  exit 1
}

to_module_filter() {
  local input="$1"
  if [[ "$input" == *.rs ]] || [[ "$input" == src/* ]] || [[ "$input" == */* ]]; then
    input="${input#src/}"
    input="${input%.rs}"
    input="${input//\//::}"
  fi
  echo "$input"
}

[ $# -lt 1 ] && usage

RAW_FILTER="$1"
FILTER=$(to_module_filter "$RAW_FILTER")

echo "Running tests matching: $FILTER"
echo ""

cargo test --features test-helpers "$FILTER" -- --nocapture
