#!/usr/bin/env bash
set -e

# Repository information (Update this with your actual GitHub username/repo)
REPO="novitaswebworks/proc-manager"
BINARY_NAME="proc-manager"
INSTALL_DIR="/usr/local/bin"

# Detect OS and Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

echo "Detecting OS and Architecture..."

if [ "$OS" = "Linux" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET="linux-x86_64"
    else
        echo "Error: Unsupported Linux architecture: $ARCH"
        exit 1
    fi
elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "x86_64" ]; then
        TARGET="macos-x86_64"
    elif [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
        TARGET="macos-arm64"
    else
        echo "Error: Unsupported macOS architecture: $ARCH"
        exit 1
    fi
else
    echo "Error: Unsupported OS: $OS"
    exit 1
fi

echo "Found supported system: $OS ($TARGET)"

# Fetch latest release URL
echo "Fetching latest release information..."
LATEST_RELEASE_URL=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep "browser_download_url" | grep "$TARGET.tar.gz" | cut -d '"' -f 4)

if [ -z "$LATEST_RELEASE_URL" ]; then
    echo "Error: Could not find a release for $TARGET. Make sure you published a release on GitHub."
    exit 1
fi

echo "Downloading $BINARY_NAME from $LATEST_RELEASE_URL..."
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"
curl -sL "$LATEST_RELEASE_URL" -o "$BINARY_NAME.tar.gz"

echo "Extracting..."
tar -xzf "$BINARY_NAME.tar.gz"

echo "Installing to $INSTALL_DIR (requires sudo)..."
sudo mv "$BINARY_NAME" "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Clean up
cd - > /dev/null
rm -rf "$TMP_DIR"

echo ""
echo "==============================================="
echo " 🎉 Successfully installed $BINARY_NAME! "
echo " You can now run it by typing: $BINARY_NAME "
echo "==============================================="
