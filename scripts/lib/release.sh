set -euo pipefail

collect_commits_since_last_release() {
  local last_tag="$1"
  get_commits_since_last_tag "$last_tag"
}

abort_if_no_changes_detected() {
  local commits="$1"
  if [[ -z "$commits" ]]; then
    abort_with_error "no changes detected since last release"
  fi
}

determine_bump_from_commits() {
  local commits="$1"

  if echo "$commits" | grep -qE "BREAKING CHANGE|!:"; then
    echo "major"
    return
  fi

  if echo "$commits" | grep -qE "^feat(\(.+\))?:"; then
    echo "minor"
    return
  fi

  echo "patch"
}

resolve_release_bump_type() {
  local commits="$1"
  local explicit="$2"

  if [[ -n "$explicit" ]]; then
    echo "$explicit"
    return
  fi

  determine_bump_from_commits "$commits"
}

generate_changelog() {
  local output="${1:-CHANGELOG.md}"

  git-cliff --config cliff.toml --output "$output"

  if [[ ! -s "$output" ]]; then
    abort_with_error "generated changelog is empty"
  fi
}

commit_release_artifacts() {
  local version="$1"

  git add Cargo.toml CHANGELOG.md
  git commit -m "chore(release): v${version}"
}

open_pull_request_if_possible() {
  local branch="$1"
  local version="$2"

  if command -v fj >/dev/null 2>&1; then
    fj pr create \
      --title "chore(release): v${version}" \
      --body "Automated release PR for v${version}" \
      --base main \
      --head "$branch"
  fi
}
