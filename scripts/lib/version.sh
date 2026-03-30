set -euo pipefail

list_workspace_manifests() {
  find . -name Cargo.toml -not -path "*/target/*"
}

read_version_from_manifest() {
  local file="$1"
  grep '^version =' "$file" | sed -E 's/version = "(.*)"/\1/'
}

parse_semver() {
  local version="$1"
  IFS='.' read -r MAJOR MINOR PATCH <<< "$version"
  echo "$MAJOR $MINOR $PATCH"
}

bump_version() {
  local version="$1"
  local bump="$2"

  read -r MAJOR MINOR PATCH < <(parse_semver "$version")

  case "$bump" in
    patch) PATCH=$((PATCH + 1)) ;;
    minor) MINOR=$((MINOR + 1)); PATCH=0 ;;
    major) MAJOR=$((MAJOR + 1)); MINOR=0; PATCH=0 ;;
    *) abort_with_error "invalid bump type: $bump" ;;
  esac

  echo "${MAJOR}.${MINOR}.${PATCH}"
}

apply_pre_release() {
  local version="$1"
  local pre="$2"

  if [[ -z "$pre" ]]; then
    echo "$version"
  else
    echo "${version}-${pre}.0"
  fi
}

write_version_to_manifest() {
  local file="$1"
  local version="$2"

  sed -i.bak -E "s/^version = \".*\"/version = \"${version}\"/" "$file"
  rm -f "${file}.bak"
}

write_workspace_version() {
  local version="$1"

  while IFS= read -r manifest; do
    write_version_to_manifest "$manifest" "$version"
  done < <(list_workspace_manifests)
}

build_release_branch_name() {
  local version="$1"
  echo "release/${version}"
}
