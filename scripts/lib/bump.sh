set -euo pipefail

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

  if [[ -n "$BUMP_TYPE" ]]; then
    echo "$BUMP_TYPE"
    return
  fi

  determine_bump_from_commits "$commits"
}
