#!/usr/bin/env bash
set -euo pipefail

resolve_commit_range() {
  if [ "$EVENT_NAME" = "pull_request" ] && \
     git rev-parse --verify "origin/${BASE_REF}" >/dev/null 2>&1 && \
     git rev-parse --verify "origin/${HEAD_REF}" >/dev/null 2>&1; then
    echo "origin/${BASE_REF}..origin/${HEAD_REF}"
  else
    echo ""
  fi
}

collect_commit_messages() {
  local range
  range="$(resolve_commit_range)"
  if [ -n "$range" ]; then
    git log --format=%s "$range" || true
  else
    git log --format=%s -n 20 || true
  fi
}

abort_if_any_commit_message_violates_conventional_commits() {
  local commits="$1"
  local types="feat|fix|docs|style|refactor|perf|test|chore|revert|ci|build"
  local pattern="^(${types})(\(.+\))?: .+"

  echo "$commits" | while IFS= read -r msg; do
    if [ -z "$msg" ]; then
      continue  # skip blank separator lines
    fi

    if ! echo "$msg" | grep -qE "$pattern"; then
      echo "Invalid conventional commit message: '$msg'" >&2
      echo "Allowed types: ${types//|/, }" >&2
      exit 1
    fi
  done
}

validate_commit_messages() {
  local commits
  commits="$(collect_commit_messages)"

  if [ -z "$commits" ]; then
    echo "No commits to check"
    return
  fi

  abort_if_any_commit_message_violates_conventional_commits "$commits"
  echo "Commit message check: PASS"
}

validate_commit_messages
