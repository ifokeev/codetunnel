#!/bin/bash

# Build ttyd for macOS in CI environment
# This simplified version builds for the current architecture only

set -e

TTYD_VERSION="1.7.7"

echo "Building ttyd ${TTYD_VERSION} for macOS (CI)..."

# Create build directory
BUILD_DIR="/tmp/ttyd-build-$$"
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Clone ttyd repository
echo "Cloning ttyd repository..."
git clone --depth 1 --branch ${TTYD_VERSION} https://github.com/tsl0922/ttyd.git
cd ttyd

# Determine architecture
ARCH=$(uname -m)
echo "Building for architecture: $ARCH"

# Build ttyd
mkdir -p build
cd build

# Configure based on architecture
if [[ "$ARCH" == "arm64" ]]; then
    # ARM64 (Apple Silicon)
    cmake .. \
        -DCMAKE_BUILD_TYPE=Release \
        -DOPENSSL_ROOT_DIR=/opt/homebrew/opt/openssl@3 \
        -DOPENSSL_LIBRARIES=/opt/homebrew/opt/openssl@3/lib
else
    # x86_64 (Intel)
    cmake .. \
        -DCMAKE_BUILD_TYPE=Release \
        -DOPENSSL_ROOT_DIR=/usr/local/opt/openssl@3 \
        -DOPENSSL_LIBRARIES=/usr/local/opt/openssl@3/lib
fi

# Build
make -j$(sysctl -n hw.ncpu)

# Verify the binary
echo "Verifying binary..."
file ttyd
./ttyd --version || true

# Copy to destination
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DEST_DIR="$SCRIPT_DIR/../apps/desktop/src-tauri/resources/macos"
echo "Current directory: $(pwd)"
echo "Script directory: $SCRIPT_DIR"
echo "Destination directory: $DEST_DIR"
mkdir -p "$DEST_DIR"
echo "Copying ttyd to $DEST_DIR/ttyd"
cp ttyd "$DEST_DIR/ttyd"
chmod +x "$DEST_DIR/ttyd"
echo "Verifying copy..."
ls -la "$DEST_DIR/ttyd"

# Clean up
cd /
rm -rf "$BUILD_DIR"

echo "Successfully built ttyd binary for $ARCH!"
echo "Location: $DEST_DIR/ttyd"