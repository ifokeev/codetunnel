#!/bin/bash

# Build ttyd universal binary for macOS
# This script builds ttyd from source for both x86_64 and arm64 architectures

set -e

TTYD_VERSION="1.7.7"

echo "Building ttyd ${TTYD_VERSION} universal binary for macOS..."

# Check dependencies
MISSING_DEPS=""
for cmd in cmake make git; do
    if ! command -v $cmd &> /dev/null; then
        MISSING_DEPS="$MISSING_DEPS $cmd"
    fi
done

if [ ! -z "$MISSING_DEPS" ]; then
    echo "Error: Missing required dependencies:$MISSING_DEPS"
    echo "Install with: brew install$MISSING_DEPS"
    exit 1
fi

# Create build directory
BUILD_DIR="/tmp/ttyd-universal-build-$$"
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Clone ttyd repository
echo "Cloning ttyd repository..."
git clone --depth 1 --branch ${TTYD_VERSION} https://github.com/tsl0922/ttyd.git
cd ttyd

# Install dependencies with Homebrew
echo "Installing build dependencies..."
brew install --quiet libwebsockets libevent jansson openssl@3 2>/dev/null || true

# Build for x86_64
echo "Building for x86_64..."
mkdir -p build-x86_64
cd build-x86_64
cmake .. \
    -DCMAKE_OSX_ARCHITECTURES=x86_64 \
    -DCMAKE_BUILD_TYPE=Release \
    -DOPENSSL_ROOT_DIR=/usr/local/opt/openssl@3 \
    -DOPENSSL_LIBRARIES=/usr/local/opt/openssl@3/lib
make -j$(sysctl -n hw.ncpu)
cd ..

# Build for arm64
echo "Building for arm64..."
mkdir -p build-arm64
cd build-arm64
cmake .. \
    -DCMAKE_OSX_ARCHITECTURES=arm64 \
    -DCMAKE_BUILD_TYPE=Release \
    -DOPENSSL_ROOT_DIR=/opt/homebrew/opt/openssl@3 \
    -DOPENSSL_LIBRARIES=/opt/homebrew/opt/openssl@3/lib
make -j$(sysctl -n hw.ncpu)
cd ..

# Create universal binary
echo "Creating universal binary..."
lipo -create \
    build-x86_64/ttyd \
    build-arm64/ttyd \
    -output ttyd-universal

# Verify the binary
echo "Verifying universal binary..."
lipo -info ttyd-universal
file ttyd-universal

# Copy to destination
DEST_DIR="$(dirname "$0")/../apps/desktop/src-tauri/resources/macos"
mkdir -p "$DEST_DIR"
cp ttyd-universal "$DEST_DIR/ttyd"
chmod +x "$DEST_DIR/ttyd"

# Clean up
cd /
rm -rf "$BUILD_DIR"

echo "Successfully built universal ttyd binary!"
echo "Location: $DEST_DIR/ttyd"