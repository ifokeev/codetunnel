#!/bin/bash

# Download binaries for all platforms

set -e

TTYD_VERSION="1.7.7"
CLOUDFLARED_VERSION="2024.12.2"

# Parse arguments
SKIP_MACOS_TTYD=false
if [[ "$1" == "--skip-macos-ttyd" ]]; then
    SKIP_MACOS_TTYD=true
fi

# Create directories
mkdir -p apps/desktop/src-tauri/resources/{macos,windows,linux}
mkdir -p apps/desktop/src-tauri/resources/macos/{arm64,x86_64}

echo "Downloading binaries..."

# macOS binaries
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "Downloading macOS binaries..."
    
    # For macOS, we'll use pre-built binaries from a GitHub Actions workflow
    # or fall back to system ttyd if available
    echo "Setting up ttyd for macOS..."
    
    # Check if ttyd binary already exists (from build script)
    if [[ -f "apps/desktop/src-tauri/resources/macos/ttyd" ]] && [[ -x "apps/desktop/src-tauri/resources/macos/ttyd" ]]; then
        echo "ttyd binary already exists, skipping download"
        TTYD_SIZE=$(stat -f%z "apps/desktop/src-tauri/resources/macos/ttyd" 2>/dev/null || stat -c%s "apps/desktop/src-tauri/resources/macos/ttyd" 2>/dev/null || echo "unknown")
        echo "Existing ttyd size: $TTYD_SIZE bytes"
    elif [[ "$SKIP_MACOS_TTYD" == "true" ]]; then
        echo "Skipping macOS ttyd (will be built separately)"
    elif command -v ttyd &> /dev/null; then
        echo "Found system ttyd, creating universal binary..."
        SYSTEM_TTYD=$(which ttyd)
        
        # Check architecture of system ttyd
        TTYD_ARCH=$(lipo -info "$SYSTEM_TTYD" 2>/dev/null | awk -F': ' '{print $2}' || echo "unknown")
        
        if [[ "$TTYD_ARCH" == *"arm64"* ]] && [[ "$TTYD_ARCH" == *"x86_64"* ]]; then
            # Already universal
            echo "System ttyd is already universal, copying..."
            cp "$SYSTEM_TTYD" apps/desktop/src-tauri/resources/macos/ttyd
        elif [[ "$TTYD_ARCH" == *"arm64"* ]]; then
            # ARM64 only
            echo "System ttyd is ARM64 only"
            cp "$SYSTEM_TTYD" apps/desktop/src-tauri/resources/macos/arm64/ttyd
            cp "$SYSTEM_TTYD" apps/desktop/src-tauri/resources/macos/ttyd
        elif [[ "$TTYD_ARCH" == *"x86_64"* ]]; then
            # x86_64 only
            echo "System ttyd is x86_64 only"
            cp "$SYSTEM_TTYD" apps/desktop/src-tauri/resources/macos/x86_64/ttyd
            cp "$SYSTEM_TTYD" apps/desktop/src-tauri/resources/macos/ttyd
        else
            echo "System ttyd architecture unknown, copying as-is"
            cp "$SYSTEM_TTYD" apps/desktop/src-tauri/resources/macos/ttyd
        fi
        
        chmod +x apps/desktop/src-tauri/resources/macos/ttyd
    else
        echo "WARNING: ttyd not found on system"
        echo "Please install ttyd with: brew install ttyd"
        echo ""
        echo "Or download pre-built binaries from:"
        echo "https://github.com/tsl0922/ttyd/releases"
        echo ""
        echo "Creating placeholder..."
        echo '#!/bin/bash' > apps/desktop/src-tauri/resources/macos/ttyd
        echo 'echo "Error: ttyd not installed. Please run: brew install ttyd" >&2' >> apps/desktop/src-tauri/resources/macos/ttyd
        echo 'exit 1' >> apps/desktop/src-tauri/resources/macos/ttyd
        chmod +x apps/desktop/src-tauri/resources/macos/ttyd
    fi
    
    # Download cloudflared for macOS (supports both Intel and Apple Silicon)
    curl -L "https://github.com/cloudflare/cloudflared/releases/download/${CLOUDFLARED_VERSION}/cloudflared-darwin-amd64.tgz" \
        -o /tmp/cloudflared-darwin.tgz
    tar -xzf /tmp/cloudflared-darwin.tgz -C apps/desktop/src-tauri/resources/macos/
    rm /tmp/cloudflared-darwin.tgz
fi

# Linux binaries
echo "Downloading Linux binaries..."

# Download ttyd for Linux
curl -L "https://github.com/tsl0922/ttyd/releases/download/${TTYD_VERSION}/ttyd.x86_64" \
    -o apps/desktop/src-tauri/resources/linux/ttyd
chmod +x apps/desktop/src-tauri/resources/linux/ttyd

# Download cloudflared for Linux
curl -L "https://github.com/cloudflare/cloudflared/releases/download/${CLOUDFLARED_VERSION}/cloudflared-linux-amd64" \
    -o apps/desktop/src-tauri/resources/linux/cloudflared
chmod +x apps/desktop/src-tauri/resources/linux/cloudflared

# Windows binaries
echo "Downloading Windows binaries..."

# Download ttyd for Windows
curl -L "https://github.com/tsl0922/ttyd/releases/download/${TTYD_VERSION}/ttyd.win32.exe" \
    -o apps/desktop/src-tauri/resources/windows/ttyd.exe

# Download cloudflared for Windows
curl -L "https://github.com/cloudflare/cloudflared/releases/download/${CLOUDFLARED_VERSION}/cloudflared-windows-amd64.exe" \
    -o apps/desktop/src-tauri/resources/windows/cloudflared.exe

echo "Binary download complete!"
echo ""
echo "Downloaded binaries to:"
echo "  - apps/desktop/src-tauri/resources/macos/"
echo "  - apps/desktop/src-tauri/resources/linux/"
echo "  - apps/desktop/src-tauri/resources/windows/"