#!/usr/bin/env bash
set -euo pipefail

html_coverage_report_exists() {
  [ -d target/llvm-cov/html ]
}

print_html_coverage_report_path() {
  echo "HTML coverage report available at: target/llvm-cov/html/index.html"
}

show_coverage_report_if_available() {
  if html_coverage_report_exists; then
    print_html_coverage_report_path
  fi
}

show_coverage_report_if_available
