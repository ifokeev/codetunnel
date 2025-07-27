# Signing Keys Guide

This guide explains the different types of keys used in CodeTunnel distribution.

## Types of Keys

### 1. Tauri Updater Keys (Required for Auto-Updates)

These are the `TAURI_PRIVATE_KEY` and `TAURI_KEY_PASSWORD` in the GitHub workflow.

**Purpose**: Sign update manifests for Tauri's built-in auto-updater
**Required**: Only if you want to implement auto-updates
**Cost**: Free

#### Generate Tauri Keys:
```bash
./scripts/generate-tauri-keys.sh
```

This will output:
- **Private key**: Add to GitHub Secrets as `TAURI_PRIVATE_KEY`
- **Public key**: Add to `tauri.conf.json` under `tauri.updater.pubkey`
- **Password**: Choose a password and add to GitHub Secrets as `TAURI_KEY_PASSWORD`

### 2. Code Signing Certificates (Optional but Recommended)

These certificates prevent OS security warnings when users install your app.

#### macOS Code Signing
**Purpose**: Sign the app so macOS doesn't show "unidentified developer" warnings
**Required**: For App Store or smooth distribution
**Cost**: $99/year (Apple Developer Program)

**GitHub Secrets needed**:
- `APPLE_CERTIFICATE`: Base64 encoded .p12 certificate
- `APPLE_CERTIFICATE_PASSWORD`: Certificate password
- `APPLE_SIGNING_IDENTITY`: Your Developer ID
- `APPLE_ID`: Your Apple ID
- `APPLE_PASSWORD`: App-specific password

#### Windows Code Signing
**Purpose**: Sign the app so Windows doesn't show "Unknown Publisher" warnings
**Required**: For professional distribution
**Cost**: $200-500/year (from various Certificate Authorities)

**GitHub Secrets needed**:
- `WINDOWS_CERTIFICATE`: Base64 encoded .pfx certificate
- `WINDOWS_CERTIFICATE_PASSWORD`: Certificate password

## Quick Start (Without Code Signing)

If you want to release immediately without code signing:

1. **Option A: Remove Tauri updater keys from workflow**
   ```yaml
   # Comment out or remove these lines in .github/workflows/build.yml
   # TAURI_PRIVATE_KEY: ${{ secrets.TAURI_PRIVATE_KEY }}
   # TAURI_KEY_PASSWORD: ${{ secrets.TAURI_KEY_PASSWORD }}
   ```

2. **Option B: Generate Tauri keys (for future auto-updates)**
   ```bash
   ./scripts/generate-tauri-keys.sh
   ```
   Then add the keys to GitHub Secrets.

3. **Release your app**
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

## Setting Up GitHub Secrets

1. Go to your repository on GitHub
2. Click Settings → Secrets and variables → Actions
3. Click "New repository secret"
4. Add each secret with its corresponding value

### For Tauri Updater (minimum required):
- `TAURI_PRIVATE_KEY`: The private key from generate-tauri-keys.sh
- `TAURI_KEY_PASSWORD`: A password you choose

### For macOS Code Signing (optional):
```bash
# Export certificate to .p12
# Then convert to base64:
base64 -i certificate.p12 | pbcopy
```

### For Windows Code Signing (optional):
```bash
# Convert .pfx to base64:
base64 -w 0 certificate.pfx > certificate_base64.txt
```

## Distribution Without Keys

You can still distribute your app without any signing:

**macOS**: Users will need to right-click → Open
**Windows**: Users will click "More info" → "Run anyway"
**Linux**: No issues, works normally

## Recommended Approach

1. **Start with**: No code signing, just release
2. **Add later**: Tauri updater keys for auto-updates
3. **Eventually**: Code signing certificates for professional distribution

Your app will work perfectly fine without any of these keys - they just improve the user experience!