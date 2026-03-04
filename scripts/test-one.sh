#!/usr/bin/env bash
set -euo pipefail

INPUT="${1:-}"

if [[ -z "$INPUT" ]]; then
  echo "Usage:" >&2
  echo "  $0 src/domain/value_objects/medication_frequency.rs        # all tests in file" >&2
  echo "  $0 src/domain/value_objects/                               # all tests in dir" >&2
  echo "  $0 domain::value_objects::medication_frequency::fail       # single test by module path" >&2
  echo "  $0 fail                                                     # substring match" >&2
  exit 1
fi

INPUT="${INPUT%/}"

# src/domain/value_objects/medication_frequency.rs → domain::value_objects::medication_frequency
file_to_module() {
  local file="$1"
  local stripped="${file#src/}"
  stripped="${stripped%.rs}"
  echo "${stripped//\//"::"}";
}

extract_tests() {
  local file="$1"
  awk '
    /#\[(tokio::)?test\]/ { found=1; next }
    found && match($0, /fn ([a-zA-Z0-9_]+)/, arr) { print arr[1]; found=0; next }
    { found=0 }
  ' "$file"
}

run_integration_file() {
  local file="$1"
  local base; base=$(basename "$file" .rs)
  echo "→ integration test target: $base"
  cargo test --test "$base" --features test-helpers -- --nocapture
}

run_src_file() {
  local file="$1"
  mapfile -t fns < <(extract_tests "$file")

  if [[ ${#fns[@]} -eq 0 ]]; then
    echo "No #[test] functions found in $file" >&2
    exit 1
  fi

  local module; module=$(file_to_module "$file")
  echo "→ module: $module"
  echo "→ tests:  ${fns[*]}"

  local exit_code=0
  for fn in "${fns[@]}"; do
    echo "  running ${module}::${fn}"
    cargo test --lib -- --nocapture --exact "${module}::${fn}" || exit_code=$?
  done

  exit $exit_code
}

run_dir() {
  local dir="$1"
  local exit_code=0

  if [[ "$dir" == tests/* || "$dir" == "tests" ]]; then
    mapfile -t files < <(find "$dir" -type f -name "*.rs")
    for f in "${files[@]}"; do
      run_integration_file "$f" || exit_code=$?
    done
  else
    mapfile -t files < <(find "$dir" -type f -name "*.rs")
    for f in "${files[@]}"; do
      run_src_file "$f" || exit_code=$?
    done
  fi

  exit $exit_code
}

# ── Dispatch ──────────────────────────────────────────────────────────────────

if [[ -f "$INPUT" ]]; then
  # e.g. src/domain/value_objects/medication_frequency.rs
  if [[ "$INPUT" == tests/* ]]; then
    run_integration_file "$INPUT"
  elif [[ "$INPUT" == src/* ]]; then
    run_src_file "$INPUT"
  else
    echo "File must be under src/ or tests/" >&2; exit 1
  fi

elif [[ -d "$INPUT" ]]; then
  # e.g. src/domain/value_objects/
  run_dir "$INPUT"

else
  # Bare module path or substring — pass directly as --exact filter
  # e.g. domain::value_objects::medication_frequency::fail
  echo "→ exact filter: $INPUT"
  cargo test --lib -- --nocapture --exact "$INPUT"
fi
