#!/usr/bin/env bash
set -euo pipefail

export PATH="$HOME/.cargo/bin:$PATH"

echo "Running cargo-llvm-cov (generating JSON report)..."
# Generate JSON + text report; do not fail immediately so we can print friendly summary
cargo llvm-cov --features test-helpers --ignore-filename-regex "ports/fakes" --json --output-path cov.json || true

if [ ! -f cov.json ]; then
  echo "ERROR: cov.json not found. cargo-llvm-cov failed to produce JSON output."
  exit 1
fi

# Extract totals
lines_count=$(jq -r '.data[0].totals.lines.count' cov.json)
lines_covered=$(jq -r '.data[0].totals.lines.covered' cov.json)
lines_percent=$(jq -r '.data[0].totals.lines.percent' cov.json)
functions_percent=$(jq -r '.data[0].totals.functions.percent' cov.json)
regions_percent=$(jq -r '.data[0].totals.regions.percent' cov.json)

# Normalize null to 0
lines_percent=${lines_percent:-0}
functions_percent=${functions_percent:-0}
regions_percent=${regions_percent:-0}

printf "\nCoverage summary:\n"
printf "  Lines:     %s%%  (%s/%s)\n" "$lines_percent" "$lines_covered" "$lines_count"
printf "  Functions: %s%%\n" "$functions_percent"
printf "  Regions:   %s%%\n" "$regions_percent"

# Enforce threshold
threshold=90
# Use awk for numeric comparison (handles floats)
cmp=$(awk -v p="$lines_percent" -v t="$threshold" 'BEGIN{ if (p+0 >= t+0) print 0; else print 1 }')
if [ "$cmp" -eq 0 ]; then
  echo "Coverage check: PASS (>= ${threshold}%)"
  exit 0
else
  echo "Coverage check: FAIL (< ${threshold}%)"
  exit 1
fi
