#!/bin/bash

# Generate app icons for Tauri from source image
# Requires ImageMagick to be installed

set -e

SOURCE_ICON="apps/desktop/src/assets/a1-shell-black.png"
ICON_DIR="apps/desktop/src-tauri/icons"

# Check if ImageMagick is installed
if ! command -v convert &> /dev/null; then
    echo "ImageMagick is required but not installed."
    echo "Install it with: brew install imagemagick (macOS) or apt-get install imagemagick (Linux)"
    exit 1
fi

# Check if source icon exists
if [ ! -f "$SOURCE_ICON" ]; then
    echo "Source icon not found: $SOURCE_ICON"
    exit 1
fi

echo "Generating icons from $SOURCE_ICON..."

# Create icons directory if it doesn't exist
mkdir -p "$ICON_DIR"

# Generate PNG icons (ensure RGBA format with alpha channel)
echo "Generating PNG icons..."
convert "$SOURCE_ICON" -alpha set -resize 32x32 -define png:color-type=6 -define png:bit-depth=8 "$ICON_DIR/32x32.png"
convert "$SOURCE_ICON" -alpha set -resize 128x128 -define png:color-type=6 -define png:bit-depth=8 "$ICON_DIR/128x128.png"
convert "$SOURCE_ICON" -alpha set -resize 256x256 -define png:color-type=6 -define png:bit-depth=8 "$ICON_DIR/128x128@2x.png"
convert "$SOURCE_ICON" -alpha set -resize 512x512 -define png:color-type=6 -define png:bit-depth=8 "$ICON_DIR/icon.png"

# Generate Windows icons
echo "Generating Windows icon (.ico)..."
convert "$SOURCE_ICON" -resize 16x16 "$ICON_DIR/icon-16.png"
convert "$SOURCE_ICON" -resize 24x24 "$ICON_DIR/icon-24.png"
convert "$SOURCE_ICON" -resize 32x32 "$ICON_DIR/icon-32.png"
convert "$SOURCE_ICON" -resize 48x48 "$ICON_DIR/icon-48.png"
convert "$SOURCE_ICON" -resize 64x64 "$ICON_DIR/icon-64.png"
convert "$SOURCE_ICON" -resize 128x128 "$ICON_DIR/icon-128.png"
convert "$SOURCE_ICON" -resize 256x256 "$ICON_DIR/icon-256.png"

# Create .ico file with multiple sizes
convert "$ICON_DIR/icon-16.png" "$ICON_DIR/icon-24.png" "$ICON_DIR/icon-32.png" \
        "$ICON_DIR/icon-48.png" "$ICON_DIR/icon-64.png" "$ICON_DIR/icon-128.png" \
        "$ICON_DIR/icon-256.png" "$ICON_DIR/icon.ico"

# Clean up temporary Windows icon files
rm -f "$ICON_DIR"/icon-{16,24,32,48,64,128,256}.png

# Generate macOS icon (.icns)
echo "Generating macOS icon (.icns)..."
mkdir -p "$ICON_DIR/icon.iconset"
convert "$SOURCE_ICON" -resize 16x16 "$ICON_DIR/icon.iconset/icon_16x16.png"
convert "$SOURCE_ICON" -resize 32x32 "$ICON_DIR/icon.iconset/icon_16x16@2x.png"
convert "$SOURCE_ICON" -resize 32x32 "$ICON_DIR/icon.iconset/icon_32x32.png"
convert "$SOURCE_ICON" -resize 64x64 "$ICON_DIR/icon.iconset/icon_32x32@2x.png"
convert "$SOURCE_ICON" -resize 128x128 "$ICON_DIR/icon.iconset/icon_128x128.png"
convert "$SOURCE_ICON" -resize 256x256 "$ICON_DIR/icon.iconset/icon_128x128@2x.png"
convert "$SOURCE_ICON" -resize 256x256 "$ICON_DIR/icon.iconset/icon_256x256.png"
convert "$SOURCE_ICON" -resize 512x512 "$ICON_DIR/icon.iconset/icon_256x256@2x.png"
convert "$SOURCE_ICON" -resize 512x512 "$ICON_DIR/icon.iconset/icon_512x512.png"
convert "$SOURCE_ICON" -resize 1024x1024 "$ICON_DIR/icon.iconset/icon_512x512@2x.png"

# Generate icns file (macOS only)
if [[ "$OSTYPE" == "darwin"* ]]; then
    iconutil -c icns "$ICON_DIR/icon.iconset" -o "$ICON_DIR/icon.icns"
else
    echo "Note: .icns generation requires macOS. Using png2icns as fallback..."
    if command -v png2icns &> /dev/null; then
        png2icns "$ICON_DIR/icon.icns" "$ICON_DIR/icon.iconset"/*.png
    else
        echo "Warning: Could not generate .icns file. Install png2icns or run on macOS."
    fi
fi

# Clean up iconset directory
rm -rf "$ICON_DIR/icon.iconset"

# Generate additional Windows Store icons if needed
echo "Generating Windows Store icons..."
convert "$SOURCE_ICON" -resize 30x30 "$ICON_DIR/Square30x30Logo.png"
convert "$SOURCE_ICON" -resize 44x44 "$ICON_DIR/Square44x44Logo.png"
convert "$SOURCE_ICON" -resize 71x71 "$ICON_DIR/Square71x71Logo.png"
convert "$SOURCE_ICON" -resize 89x89 "$ICON_DIR/Square89x89Logo.png"
convert "$SOURCE_ICON" -resize 107x107 "$ICON_DIR/Square107x107Logo.png"
convert "$SOURCE_ICON" -resize 142x142 "$ICON_DIR/Square142x142Logo.png"
convert "$SOURCE_ICON" -resize 150x150 "$ICON_DIR/Square150x150Logo.png"
convert "$SOURCE_ICON" -resize 284x284 "$ICON_DIR/Square284x284Logo.png"
convert "$SOURCE_ICON" -resize 310x310 "$ICON_DIR/Square310x310Logo.png"
convert "$SOURCE_ICON" -resize 50x50 "$ICON_DIR/StoreLogo.png"

echo "Icon generation complete!"
echo ""
echo "Generated icons in $ICON_DIR:"
ls -la "$ICON_DIR"/*.{png,ico,icns} 2>/dev/null | awk '{print "  - " $9}'