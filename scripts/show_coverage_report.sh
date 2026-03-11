#!/usr/bin/env bash
set -euo pipefail

# Prints the location of the HTML coverage report produced by cargo-llvm-cov,
# if the report directory exists. Intended as a post-test informational step.

if [ -d target/llvm-cov/html ]; then
    echo "HTML coverage report available at: target/llvm-cov/html/index.html"
fi
