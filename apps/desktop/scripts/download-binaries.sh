#!/bin/bash

# Download binaries for all platforms

set -e

TTYD_VERSION="1.7.7"
CLOUDFLARED_VERSION="2024.12.2"

# Create directories
mkdir -p src-tauri/resources/{macos,windows,linux}

echo "Downloading binaries..."

# macOS binaries
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Downloading macOS binaries..."
    
    # Download ttyd for macOS
    curl -L "https://github.com/tsl0922/ttyd/releases/download/${TTYD_VERSION}/ttyd.x86_64.darwin" \
        -o src-tauri/resources/macos/ttyd
    chmod +x src-tauri/resources/macos/ttyd
    
    # Download cloudflared for macOS  
    curl -L "https://github.com/cloudflare/cloudflared/releases/download/${CLOUDFLARED_VERSION}/cloudflared-darwin-amd64.tgz" \
        -o /tmp/cloudflared-darwin.tgz
    tar -xzf /tmp/cloudflared-darwin.tgz -C src-tauri/resources/macos/
    rm /tmp/cloudflared-darwin.tgz
fi

# Linux binaries
echo "Downloading Linux binaries..."

# Download ttyd for Linux
curl -L "https://github.com/tsl0922/ttyd/releases/download/${TTYD_VERSION}/ttyd.x86_64" \
    -o src-tauri/resources/linux/ttyd
chmod +x src-tauri/resources/linux/ttyd

# Download cloudflared for Linux
curl -L "https://github.com/cloudflare/cloudflared/releases/download/${CLOUDFLARED_VERSION}/cloudflared-linux-amd64" \
    -o src-tauri/resources/linux/cloudflared
chmod +x src-tauri/resources/linux/cloudflared

# Windows binaries
echo "Downloading Windows binaries..."

# Download ttyd for Windows
curl -L "https://github.com/tsl0922/ttyd/releases/download/${TTYD_VERSION}/ttyd.win32.exe" \
    -o src-tauri/resources/windows/ttyd.exe

# Download cloudflared for Windows
curl -L "https://github.com/cloudflare/cloudflared/releases/download/${CLOUDFLARED_VERSION}/cloudflared-windows-amd64.exe" \
    -o src-tauri/resources/windows/cloudflared.exe

echo "Binary download complete!"
echo ""
echo "Downloaded binaries to:"
echo "  - src-tauri/resources/macos/"
echo "  - src-tauri/resources/linux/"
echo "  - src-tauri/resources/windows/"