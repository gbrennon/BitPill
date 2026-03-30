set -euo pipefail

generate_changelog() {
  local output="${1:-CHANGELOG.md}"

  if [ "$DRY_RUN" = true ]; then
    echo "[dry-run] git-cliff --config cliff.toml --output $output"
    return
  fi

  git-cliff --config cliff.toml --output "$output"

  if [[ ! -s "$output" ]]; then
    abort_with_error "generated changelog is empty"
  fi
}
