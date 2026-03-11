#!/usr/bin/env bash
set -euo pipefail

export PATH="$HOME/.cargo/bin:$PATH"

readonly COVERAGE_THRESHOLD=90

run_coverage_and_emit_json() {
  echo "Running cargo-llvm-cov (generating JSON report)..."
  cargo llvm-cov --features test-helpers --ignore-filename-regex "ports/fakes" --json --output-path cov.json || true
}

abort_if_coverage_json_is_missing() {
  if [ ! -f cov.json ]; then
    echo "ERROR: cov.json not found. cargo-llvm-cov failed to produce JSON output."
    exit 1
  fi
}

extract_coverage_totals_from_json() {
  lines_count=$(jq -r '.data[0].totals.lines.count' cov.json)
  lines_covered=$(jq -r '.data[0].totals.lines.covered' cov.json)
  lines_percent=$(jq -r '.data[0].totals.lines.percent' cov.json)
  functions_percent=$(jq -r '.data[0].totals.functions.percent' cov.json)
  regions_percent=$(jq -r '.data[0].totals.regions.percent' cov.json)

  lines_percent=${lines_percent:-0}
  functions_percent=${functions_percent:-0}
  regions_percent=${regions_percent:-0}
}

print_coverage_summary() {
  printf "\nCoverage summary:\n"
  printf "  Lines:     %s%%  (%s/%s)\n" "$lines_percent" "$lines_covered" "$lines_count"
  printf "  Functions: %s%%\n" "$functions_percent"
  printf "  Regions:   %s%%\n" "$regions_percent"
}

abort_if_line_coverage_is_below_threshold() {
  local passes
  passes=$(awk -v p="$lines_percent" -v t="$COVERAGE_THRESHOLD" 'BEGIN{ if (p+0 >= t+0) print 1; else print 0 }')
  if [ "$passes" -eq 1 ]; then
    echo "Coverage check: PASS (>= ${COVERAGE_THRESHOLD}%)"
  else
    echo "Coverage check: FAIL (< ${COVERAGE_THRESHOLD}%)"
    exit 1
  fi
}

run_coverage_and_emit_json
abort_if_coverage_json_is_missing
extract_coverage_totals_from_json
print_coverage_summary
abort_if_line_coverage_is_below_threshold
