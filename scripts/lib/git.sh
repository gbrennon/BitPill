set -euo pipefail

get_current_branch() {
  git rev-parse --abbrev-ref HEAD
}

ensure_on_branch() {
  local expected="$1"
  local current
  current="$(get_current_branch)"

  if [ "$current" != "$expected" ]; then
    abort_with_error "expected branch '$expected' but found '$current'"
  fi
}

ensure_working_tree_clean() {
  if ! git diff --quiet || ! git diff --cached --quiet; then
    abort_with_error "working tree is not clean"
  fi
}

ensure_main_is_up_to_date() {
  run git pull --ff-only
}

get_last_tag() {
  git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0"
}

get_commits_since_last_tag() {
  local last_tag="$1"
  git log "${last_tag}"..HEAD --pretty=format:%s
}

create_branch() {
  local branch="$1"
  run git checkout -b "$branch"
}

push_branch() {
  local branch="$1"
  run git push -u origin "$branch"
}
