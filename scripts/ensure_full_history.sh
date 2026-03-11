#!/usr/bin/env bash
set -euo pipefail

# Ensures the local git clone has full history (un-shallows if needed).
# Safe to call unconditionally; exits successfully if history is already complete.

if git rev-parse --is-shallow-repository | grep -q true; then
    echo "Shallow repository detected — fetching full history..."
    git fetch --no-tags --prune --unshallow
    echo "Full history fetched."
else
    echo "Repository already has full history."
fi
