#!/usr/bin/env sh
set -eu

REPO_URL="${AZ_REPO_URL:-https://github.com/arazgholami/az.git}"
BIN_DIR="${AZ_BIN_DIR:-$HOME/.local/bin}"
TMP_DIR=""

cleanup() {
  if [ -n "$TMP_DIR" ]; then
    rm -rf "$TMP_DIR"
  fi
}
trap cleanup EXIT INT TERM

run_from_source_tree() {
  AZ_BIN_DIR="$BIN_DIR" ./build.sh
}

if [ -f ./Cargo.toml ] && [ -f ./src/main.rs ] && [ -x ./build.sh ]; then
  run_from_source_tree
  exit 0
fi

if ! command -v git >/dev/null 2>&1; then
  echo "Error: git is required for remote install."
  echo "Install git, or download the source zip and run ./build.sh inside it."
  exit 1
fi

TMP_DIR="$(mktemp -d)"
git clone --depth 1 "$REPO_URL" "$TMP_DIR/az" >/dev/null 2>&1
cd "$TMP_DIR/az"
run_from_source_tree
