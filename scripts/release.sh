#!/usr/bin/env bash
set -euo pipefail

source "$(dirname "$0")/lib/common.sh"
source "$(dirname "$0")/lib/context.sh"
source "$(dirname "$0")/lib/exec.sh"
source "$(dirname "$0")/lib/git.sh"
source "$(dirname "$0")/lib/version.sh"
source "$(dirname "$0")/lib/bump.sh"
source "$(dirname "$0")/lib/changelog.sh"
source "$(dirname "$0")/lib/commit.sh"
source "$(dirname "$0")/lib/pr.sh"

main() {
  parse_cli_arguments "$@"

  ensure_working_tree_clean
  ensure_on_branch "main"
  ensure_main_is_up_to_date

  last_tag="$(get_last_tag)"
  commits="$(get_commits_since_last_tag "$last_tag")"

  if [[ -z "$commits" ]]; then
    abort_with_error "no changes detected since last release"
  fi

  bump_type="$(resolve_release_bump_type "$commits")"

  first_manifest="$(list_workspace_manifests | head -n 1)"
  current_version="$(read_version_from_manifest "$first_manifest")"

  next_version="$(bump_version "$current_version" "$bump_type")"
  next_version="$(apply_pre_release "$next_version" "$PRE_RELEASE")"

  branch="$(build_release_branch_name "$next_version")"

  create_branch "$branch"

  write_workspace_version "$next_version"
  generate_changelog

  commit_release_artifacts "$next_version"
  push_branch "$branch"

  open_pull_request_if_possible "$branch" "$next_version"
}

main "$@"
