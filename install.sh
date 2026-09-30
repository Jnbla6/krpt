#!/bin/bash
set -e

REPO="Jnbla6/krpt"
BIN_NAME="krpt"

echo "Detecting OS and architecture..."
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

if [ "$ARCH" = "x86_64" ]; then
    ARCH="amd64"
elif [ "$ARCH" = "aarch64" ] || [ "$ARCH" = "arm64" ]; then
    ARCH="arm64"
else
    echo "Error: Unsupported architecture $ARCH"
    exit 1
fi

ASSET_NAME="${BIN_NAME}-${OS}-${ARCH}"

echo "Fetching latest release version for $OS ($ARCH)..."
DOWNLOAD_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep "browser_download_url.*$ASSET_NAME" | cut -d : -f 2,3 | tr -d \")

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Error: Could not find a compiled binary for $OS-$ARCH."
    echo "Check the releases page manually: https://github.com/$REPO/releases"
    exit 1
fi

echo "Downloading $BIN_NAME..."
curl -sL "$DOWNLOAD_URL" -o "$BIN_NAME"

echo "Installing $BIN_NAME to /usr/local/bin (may require sudo password)..."
chmod +x "$BIN_NAME"
sudo mv "$BIN_NAME" /usr/local/bin/

echo ""
echo "Success! '$BIN_NAME' has been installed."
echo "Run 'krpt --help' to get started."
