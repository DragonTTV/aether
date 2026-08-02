#!/usr/bin/env sh

set -e

OWNER="DragonTTV"
REPO="aether"

TMP_DIR="$(mktemp -d)"

cleanup() {
    rm -rf "$TMP_DIR"
}

trap cleanup EXIT

echo
echo "         Aether Bootstrap"
echo

echo "➜ Detecting operating system..."

OS="$(uname -s)"

case "$OS" in
    Linux)
        PLATFORM="linux"
        ;;
    *)
        echo "✗ Unsupported operating system: $OS"
        exit 1
        ;;
esac

echo "✓ $PLATFORM detected"

echo "➜ Detecting architecture..."

ARCH="$(uname -m)"

case "$ARCH" in
    x86_64|amd64)
        TARGET="x86_64"
        ;;
    aarch64|arm64)
        TARGET="aarch64"
        ;;
    *)
        echo "✗ Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

echo "✓ $TARGET detected"

ARCHIVE="aether-${PLATFORM}-${TARGET}.tar.gz"
URL="https://github.com/${OWNER}/${REPO}/releases/latest/download/${ARCHIVE}"

echo "➜ Downloading Aether..."

curl -fsSL "$URL" -o "$TMP_DIR/aether.tar.gz"

echo "✓ Download complete"

echo "➜ Extracting archive..."

tar -xzf "$TMP_DIR/aether.tar.gz" -C "$TMP_DIR"

echo "✓ Extraction complete"

echo "➜ Launching installer..."

chmod +x "$TMP_DIR/aether-setup"

exec "$TMP_DIR/aether-setup" install