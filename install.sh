#!/usr/bin/env sh
set -eu

RAW_URL="${AZ_RAW_URL:-https://raw.githubusercontent.com/arazgholami/az/refs/heads/main/az}"
BIN_DIR="${AZ_BIN_DIR:-$HOME/.local/bin}"
TARGET="$BIN_DIR/az"
TMP="$(mktemp)"

cleanup() {
  rm -f "$TMP"
}
trap cleanup EXIT INT TERM

if ! command -v php >/dev/null 2>&1; then
  echo "Error: PHP CLI is required."
  echo "On Debian or Ubuntu, install it with: sudo apt install php-cli"
  exit 1
fi

mkdir -p "$BIN_DIR"

if command -v curl >/dev/null 2>&1; then
  curl -fsSL "$RAW_URL" -o "$TMP"
elif command -v wget >/dev/null 2>&1; then
  wget -qO "$TMP" "$RAW_URL"
else
  echo "Error: curl or wget is required."
  exit 1
fi

chmod +x "$TMP"
mv "$TMP" "$TARGET"

case ":$PATH:" in
  *":$BIN_DIR:"*) ;;
  *)
    PROFILE="$HOME/.profile"
    LINE='export PATH="$HOME/.local/bin:$PATH"'

    if [ "$BIN_DIR" = "$HOME/.local/bin" ] && [ -f "$PROFILE" ] && ! grep -F "$LINE" "$PROFILE" >/dev/null 2>&1; then
      printf '\n%s\n' "$LINE" >> "$PROFILE"
      echo "Added ~/.local/bin to PATH in ~/.profile."
      echo "Restart your terminal or run: . ~/.profile"
    else
      echo "Note: $BIN_DIR is not in PATH."
      echo "Add this to your shell profile: export PATH=\"$BIN_DIR:\$PATH\""
    fi
    ;;
esac

echo "Az installed: $TARGET"
echo "Run it with: az"
