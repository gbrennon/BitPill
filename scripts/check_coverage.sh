#!/usr/bin/env bash
# shellcheck disable=SC1091
set -euo pipefail

# Source common functions
source "$(dirname "$0")/lib/common.sh"

readonly COVERAGE_THRESHOLD=90

run_coverage_and_emit_json() {
  echo "Running cargo-llvm-cov (generating JSON report)..."
  cargo llvm-cov --features test-helpers --ignore-filename-regex "ports/fakes" --json --output-path cov.json || true
}

run_coverage_and_emit_json
abort_if_coverage_json_is_missing
extract_coverage_totals_from_json
print_coverage_table
abort_if_line_coverage_is_below_threshold "$COVERAGE_THRESHOLD"
