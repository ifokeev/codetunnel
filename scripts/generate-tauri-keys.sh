#!/bin/bash

# This script generates the keys needed for Tauri's updater system
# These keys are used to sign update manifests

echo "Generating Tauri updater keys..."

# Check if Tauri CLI is installed
if ! command -v cargo-tauri &> /dev/null; then
    echo "Installing Tauri CLI..."
    cargo install tauri-cli
fi

# Generate the keys
echo "Running: cargo tauri signer generate"
cargo tauri signer generate

echo ""
echo "Keys generated! You'll see output like:"
echo "  - Private key (base64): ..."
echo "  - Public key: ..."
echo ""
echo "Next steps:"
echo "1. Add the private key to GitHub Secrets as TAURI_PRIVATE_KEY"
echo "2. Add a password for the key to GitHub Secrets as TAURI_KEY_PASSWORD"
echo "3. Add the public key to your tauri.conf.json under tauri.updater.pubkey"
echo ""
echo "Note: Keep the private key secure and never commit it to your repository!"