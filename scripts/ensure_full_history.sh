#!/usr/bin/env bash
set -euo pipefail

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

ensure_full_git_history_is_available
