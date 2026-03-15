#!/usr/bin/env bash
set -euo pipefail

# Source common functions
source "$(dirname "$0")/lib/common.sh"

show_coverage_report_if_available
