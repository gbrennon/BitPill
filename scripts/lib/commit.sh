set -euo pipefail

commit_release_artifacts() {
  local version="$1"

  run git add .
  run git commit -m "chore(release): v${version}"
}
