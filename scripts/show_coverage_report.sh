#!/usr/bin/env bash
# shellcheck disable=SC1091
set -euo pipefail

# Source common functions
source "$(dirname "$0")/lib/common.sh"

show_coverage_report_if_available
