set -euo pipefail

open_pull_request_if_possible() {
  local branch="$1"
  local version="$2"

  if command -v fj >/dev/null 2>&1; then
    if [ "$DRY_RUN" = true ]; then
      echo "[dry-run] fj pr create --title chore(release): v${version}"
      return
    fi

    fj pr create \
      --title "chore(release): v${version}" \
      --body "Release PR for v${version}" \
      --base main \
      --head "$branch"
  fi
}
