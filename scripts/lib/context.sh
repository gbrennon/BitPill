#!/usr/bin/env bash
set -euo pipefail

# shellcheck disable=SC2034
DRY_RUN=false
BUMP_TYPE="$1"
PRE_RELEASE="$2"

parse_cli_arguments() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --dry-run)
        # shellcheck disable=SC2034
        DRY_RUN=true
        shift
        ;;
      --pre-release)
        # shellcheck disable=SC2034
        PRE_RELEASE="$2"
        shift 2
        ;;
      patch|minor|major)
        # shellcheck disable=SC2034
        BUMP_TYPE="$1"
        shift
        ;;
      *)
        abort_with_error "unknown argument: $1"
        ;;
    esac
  done
}
