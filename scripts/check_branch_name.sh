#!/usr/bin/env bash
set -euo pipefail

# Validates the current branch name against allowed patterns.
#
# Required environment variables:
#   EVENT_NAME  — github.event_name (e.g. "push" or "pull_request")
#   HEAD_REF    — github.head_ref (branch name for pull_request events)
#
# GITHUB_REF_NAME and GITHUB_REF are set automatically by the Actions runner
# and are used as fallbacks for push events.

if [ "$EVENT_NAME" = "pull_request" ]; then
    branch="$HEAD_REF"
else
    branch="${GITHUB_REF_NAME:-${GITHUB_REF#refs/heads/}}"
fi

echo "Branch: $branch"

# Allow: main, develop, feature/*, fix/*, hotfix/*
if [[ ! "$branch" =~ ^(main|develop|feature/.+|fix/.+|hotfix/.+)$ ]]; then
    echo "Branch name '$branch' does not match allowed patterns" >&2
    exit 1
fi

echo "Branch name check: PASS"
