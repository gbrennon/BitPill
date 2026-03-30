#!/usr/bin/env bash
set -euo pipefail

export DRY_RUN=false
export BUMP_TYPE="$1"
export PRE_RELEASE="$2"

parse_cli_arguments() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --dry-run)
        DRY_RUN=true
        shift
        ;;
      --pre-release)
        PRE_RELEASE="$2"
        shift 2
        ;;
      patch|minor|major)
        BUMP_TYPE="$1"
        shift
        ;;
      *)
        abort_with_error "unknown argument: $1"
        ;;
    esac
  done
}
