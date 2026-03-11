#!/usr/bin/env bash
set -euo pipefail

resolve_current_branch_name() {
  if [ "$EVENT_NAME" = "pull_request" ]; then
    echo "$HEAD_REF"
  else
    echo "${GITHUB_REF_NAME:-${GITHUB_REF#refs/heads/}}"
  fi
}

abort_if_branch_name_violates_naming_convention() {
  local branch="$1"
  if [[ ! "$branch" =~ ^(main|develop|feature/.+|fix/.+|hotfix/.+)$ ]]; then
    echo "Branch name '$branch' does not match allowed patterns" >&2
    exit 1
  fi
}

validate_branch_name() {
  local branch
  branch="$(resolve_current_branch_name)"
  echo "Branch: $branch"
  abort_if_branch_name_violates_naming_convention "$branch"
  echo "Branch name check: PASS"
}

validate_branch_name
