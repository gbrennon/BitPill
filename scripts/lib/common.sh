#!/usr/bin/env bash
# Common shared functions for BitPill scripts
# Source this file in other scripts: source "$(dirname "$0")/lib/common.sh"

set -euo pipefail

# ========================================
# Git Environment Helpers
# ========================================

resolve_current_branch_name() {
  if [ "$EVENT_NAME" = "pull_request" ]; then
    echo "$HEAD_REF"
  else
    echo "${GITHUB_REF_NAME:-${GITHUB_REF#refs/heads/}}"
  fi
}

resolve_commit_range() {
  if [ "$EVENT_NAME" = "pull_request" ] && \
     git rev-parse --verify "origin/${BASE_REF}" >/dev/null 2>&1 && \
     git rev-parse --verify "origin/${HEAD_REF}" >/dev/null 2>&1; then
    echo "origin/${BASE_REF}..origin/${HEAD_REF}"
  else
    echo ""
  fi
}

repository_is_shallow() {
  git rev-parse --is-shallow-repository | grep -q true
}

fetch_full_history_from_remote() {
  echo "Shallow repository detected — fetching full history..."
  git fetch --no-tags --prune --unshallow
  echo "Full history fetched."
}

ensure_full_git_history_is_available() {
  if repository_is_shallow; then
    fetch_full_history_from_remote
  else
    echo "Repository already has full history."
  fi
}

# ========================================
# Rust/Cargo Helpers
# ========================================

ensure_cargo_in_path() {
  export PATH="$HOME/.cargo/bin:$PATH"
}

install_rustup_if_missing() {
  if command -v rustup >/dev/null 2>&1; then
    return
  fi

  echo "Installing rustup..."
  local tmp_dir
  tmp_dir="$(mktemp -d)"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o "$tmp_dir/rustup-init.sh"
  sh "$tmp_dir/rustup-init.sh" -y
  rm -rf "$tmp_dir"
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
  rustup default stable
  rustup component add rustfmt clippy || true
}

install_cargo_tool_if_missing() {
  local tool_name="$1"
  local install_cmd="$2"
  
  if [[ -z "${CI:-}" ]] && command -v "$tool_name" >/dev/null 2>&1; then
    return
  fi
  
  echo "Installing $tool_name..."
  eval "$install_cmd" || {
    echo "Failed to install $tool_name" >&2
    exit 1
  }
}

print_installed_tool_versions() {
  if command -v rustc >/dev/null 2>&1; then rustc --version; else echo "rustc not found"; fi
  if command -v cargo >/dev/null 2>&1; then cargo --version; else echo "cargo not found"; fi
  if command -v just >/dev/null 2>&1; then just --version && just tools; else echo "just not found"; fi
  if command -v cargo-llvm-cov >/dev/null 2>&1; then cargo llvm-cov --version; else echo "cargo-llvm-cov not found"; fi
}

# ========================================
# Coverage Helpers
# ========================================

coverage_json_exists() {
  [ -f cov.json ]
}

abort_if_coverage_json_is_missing() {
  if ! coverage_json_exists; then
    echo "ERROR: cov.json not found. cargo-llvm-cov failed to produce JSON output." >&2
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

normalize_path() {
  local path="$1"
  # Strip leading project-name prefix (e.g. "BitPill/src/" -> "src/")
  path=$(echo "$path" | sed 's|^[^/]*/src/|src/|')
  # Remap bare layer paths that live under src/presentation/
  # Covers: tui/, rest/, dtos/, application/, domain/, infrastructure/, presentation/
  path=$(echo "$path" | sed \
    -e 's|^tui/|src/presentation/tui/|' \
    -e 's|^rest/|src/presentation/rest/|' \
    -e 's|^dtos/|src/application/dtos/|' \
    -e 's|^application/|src/application/|' \
    -e 's|^domain/|src/domain/|' \
    -e 's|^infrastructure/|src/infrastructure/|' \
    -e 's|^presentation/|src/presentation/|' \
  )
  echo "$path"
}

extract_missing_lines() {
  local file_path="$1"
  # Segments schema: [line, col, count, has_count, is_region_entry, is_gap_region]
  # A segment with count=0 and has_count=false starts an uncovered region.
  # The next segment (any count) closes it.
  jq -r --arg fp "$file_path" '
    .data[0].files[]
    | select(.filename == $fp)
    | .segments as $segs
    | [ range(0; $segs | length)
        | . as $i
        | $segs[$i]
        | select(.[2] == 0 and .[3] == false)
        | { start: .[0], end: ($segs[$i+1] // .[0:1])[0] }
      ]
    | group_by(.start)
    | map(.[0])
    | map(
        if .start == .end then (.start | tostring)
        else "\(.start)-\(.end)"
        end
      )
    | join(", ")
  ' cov.json
}

print_coverage_table() {
  printf "\n"

  # Collect normalized rows into a temp file so we can measure column widths first
  local tmp_rows
  tmp_rows=$(mktemp)

  jq -r '
    .data[0].files[]
    | select(.summary.lines.count > 0)
    | [
        .filename,
        (.summary.lines.count | tostring),
        ((.summary.lines.count - .summary.lines.covered) | tostring),
        .summary.lines.percent
      ]
    | @tsv
  ' cov.json | while IFS=$'\t' read -r raw_path stmts miss pct; do
    local norm
    norm=$(normalize_path "$raw_path")
    printf '%s\t%s\t%s\t%s\t%s\n' "$norm" "$stmts" "$miss" "$pct" "$raw_path"
  done | sort > "$tmp_rows"

  # Compute column widths
  local max_name_len
  max_name_len=$(awk -F'\t' '{print length($1)}' "$tmp_rows" | sort -n | tail -1)
  local name_col=$(( max_name_len > 4 ? max_name_len : 4 ))

  local max_missing_len=7  # minimum width for "Missing" header
  while IFS=$'\t' read -r norm stmts miss pct raw_path; do
    if [ "$miss" -gt 0 ]; then
      local mlines
      mlines=$(extract_missing_lines "$raw_path")
      local mlen=${#mlines}
      [ "$mlen" -gt "$max_missing_len" ] && max_missing_len=$mlen
    fi
  done < "$tmp_rows"
  
  # Limit max_missing_len to prevent excessive width, but ensure it's at least 7
  # If missing lines are too long, they will wrap but the table structure remains
  if [ "$max_missing_len" -gt 80 ]; then
    max_missing_len=80
  fi

  local sep
  sep=$(printf '%*s' $(( name_col + 28 + max_missing_len )) '' | tr ' ' '-')

  printf "%-${name_col}s  %6s  %4s  %6s  %-${max_missing_len}s\n" \
    "Name" "Stmts" "Miss" "Cover" "Missing"
  echo "$sep"

  while IFS=$'\t' read -r norm stmts miss pct raw_path; do
    local missing_lines=""
    if [ "$miss" -gt 0 ]; then
      missing_lines=$(extract_missing_lines "$raw_path")
      # Truncate very long missing line lists to prevent table breakage
      if [ ${#missing_lines} -gt $max_missing_len ]; then
        missing_lines="${missing_lines:0:$((max_missing_len-4))} ..."
      fi
    fi
    printf "%-${name_col}s  %6s  %4s  %5.1f%%  %-${max_missing_len}s\n" \
      "$norm" "$stmts" "$miss" "$pct" "$missing_lines"
  done < "$tmp_rows"

  echo "$sep"
  printf "%-${name_col}s  %6s  %4s  %5.1f%%\n" \
    "TOTAL" "$lines_count" "$(( lines_count - lines_covered ))" "$lines_percent"

  printf "\n"
  printf "  Functions: %.1f%%\n" "$functions_percent"
  printf "  Regions:   %.1f%%\n" "$regions_percent"

  rm -f "$tmp_rows"
}

abort_if_line_coverage_is_below_threshold() {
  local threshold="$1"
  local passes
  passes=$(awk -v p="$lines_percent" -v t="$threshold" \
    'BEGIN{ if (p+0 >= t+0) print 1; else print 0 }')
  if [ "$passes" -eq 1 ]; then
    printf "\nCoverage check: PASS (>= %s%%)\n" "$threshold"
  else
    printf "\nCoverage check: FAIL (< %s%%)\n" "$threshold"
    exit 1
  fi
}

# ========================================
# Utility Functions
# ========================================

abort_with_error() {
  local message="$1"
  echo "ERROR: $message" >&2
  exit 1
}

log_info() {
  local message="$1"
  echo "$message"
}

html_coverage_report_exists() {
  [ -d target/llvm-cov/html ]
}

print_html_coverage_report_path() {
  echo "HTML coverage report available at: target/llvm-cov/html/index.html"
}

show_coverage_report_if_available() {
  if html_coverage_report_exists; then
    print_html_coverage_report_path
  fi
}