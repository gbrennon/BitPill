#!/usr/bin/env bash
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
