#!/usr/bin/env sh
set -eu

cd "$(dirname "$0")"

BIN_DIR="${AZ_BIN_DIR:-$HOME/.local/bin}"
TARGET="$BIN_DIR/az"

add_bin_dir_to_path() {
  case ":$PATH:" in
    *":$BIN_DIR:"*) return 0 ;;
  esac

  PROFILE="$HOME/.profile"
  LINE='export PATH="$HOME/.local/bin:$PATH"'

  if [ "$BIN_DIR" = "$HOME/.local/bin" ]; then
    touch "$PROFILE"
    if ! grep -F "$LINE" "$PROFILE" >/dev/null 2>&1; then
      printf '\n%s\n' "$LINE" >> "$PROFILE"
      echo "Added ~/.local/bin to PATH in ~/.profile."
      echo "Restart your terminal or run: . ~/.profile"
    fi
  else
    echo "Note: $BIN_DIR is not in PATH."
    echo "Add this to your shell profile: export PATH=\"$BIN_DIR:\$PATH\""
  fi
}

if command -v cargo >/dev/null 2>&1; then
  cargo build --release
  cp target/release/az ./az
elif command -v rustc >/dev/null 2>&1; then
  rustc --edition=2021 -O src/main.rs -o ./az
else
  echo "Rust is not installed. Install Rust, then run ./build.sh" >&2
  exit 1
fi

chmod +x ./az
mkdir -p "$BIN_DIR"
cp ./az "$TARGET"
chmod +x "$TARGET"
add_bin_dir_to_path

printf 'Built ./az\n'
printf 'Installed %s\n' "$TARGET"
printf 'Run it with: az\n'
