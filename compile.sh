#!/bin/bash
# ProgramBench-style build: must produce ./executable at the repo root.
# tree-sitter is a workspace; the CLI is crates/cli (binary `tree-sitter`).
set -e
cd "$(dirname "$0")"
cargo build --release -p tree-sitter-cli
cp "target/release/tree-sitter" executable
