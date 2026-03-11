#!/usr/bin/env bash
set -euo pipefail

# Validates that no commit message in the relevant range is empty.
#
# Required environment variables:
#   EVENT_NAME  — github.event_name (e.g. "push" or "pull_request")
#   BASE_REF    — github.event.pull_request.base.ref (only used for pull_request events)
#   HEAD_REF    — github.event.pull_request.head.ref (only used for pull_request events)

if [ "$EVENT_NAME" = "pull_request" ]; then
    if git rev-parse --verify "origin/${BASE_REF}" >/dev/null 2>&1 && \
       git rev-parse --verify "origin/${HEAD_REF}" >/dev/null 2>&1; then
        range="origin/${BASE_REF}..origin/${HEAD_REF}"
        commits=$(git log --format=%s "$range" || true)
    else
        commits=$(git log --format=%s -n 20 || true)
    fi
else
    commits=$(git log --format=%s -n 20 || true)
fi

if [ -z "$commits" ]; then
    echo "No commits to check"
    exit 0
fi

echo "$commits" | while IFS= read -r msg; do
    if [ -z "$msg" ]; then
        echo "Empty commit message detected" >&2
        exit 1
    fi
done

echo "Commit message check: PASS"
