# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Tauri 2.0 desktop application that provides a graphical interface for launching web-based terminals. It bundles ttyd (terminal server) and cloudflared (Cloudflare tunnel) binaries to create instant, secure public web terminals with a simple UI showing the connection URL. The app includes platform-specific binaries, so users don't need to install dependencies separately.

The project is structured as a monorepo using pnpm workspaces, with the desktop application located in `apps/desktop/`.

## Build Commands

### Setup (from project root)
- `pnpm install` - Install all workspace dependencies
- `cd apps/desktop && ./scripts/download-binaries.sh` - Download ttyd and cloudflared binaries for all platforms

### Development (from project root)
- `pnpm tauri:dev` - Run desktop app in development mode with hot reload
- `pnpm dev` - Run frontend dev server only
- `cd apps/desktop/src-tauri && cargo build` - Build Rust backend only
- `cd apps/desktop/src-tauri && cargo test` - Run Rust tests

### Production (from project root)
- `pnpm tauri:build` - Build production app for current platform
- `pnpm tauri:build -- -- --target universal-apple-darwin` - Build for macOS Universal
- `pnpm tauri:build -- -- --target x86_64-pc-windows-msvc` - Build for Windows x64
- `pnpm tauri:build -- -- --target x86_64-unknown-linux-gnu` - Build for Linux x64

## Architecture

### Core Components

**Frontend** (`src/`)
- React-based UI with modern dark theme design
- Start/Stop controls for terminal sessions
- Real-time display of secure connection URL with copy functionality

**Tauri Backend** (`src-tauri/`)
- `src-tauri/src/main.rs` - Application entry point
- `src-tauri/src/lib.rs` - Core application logic
- Process management for ttyd and cloudflared subprocesses
- Platform-specific binary handling

### Key Implementation Areas

1. **Process Management**
   - Spawn and monitor ttyd subprocess on random port with secret base path
   - Spawn and monitor cloudflared subprocess for tunnel creation
   - Graceful shutdown handling on app close
   - Parse cloudflared output to extract tunnel URL
   - Real-time process health monitoring

2. **State Management**
   - Tauri state for process handles and connection info
   - Mutex-protected shared state between commands
   - Event emission for UI updates
   - Terminal info includes URL with secret token

3. **Binary Bundling**
   - Platform-specific binaries in `src-tauri/resources/{macos,windows,linux}/`
   - ttyd and cloudflared binaries for each platform
   - Automatic executable permission setting on Unix
   - Resource path resolution using Tauri's path API
   - Download script available at `scripts/download-binaries.sh`

4. **Authentication Strategy**
   - URL-based security using 32-character random tokens
   - ttyd runs with `--base-path /<secret-token>` for access control
   - No traditional authentication due to ttyd WebSocket proxy limitations
   - Works seamlessly with Cloudflare tunnels and mobile devices

### Tauri Commands

The app implements these core commands:
- `start_terminal` - Launches ttyd and cloudflared, returns connection info
- `stop_terminal` - Terminates both processes cleanly
- `get_status` - Returns current terminal status and connection details

### Security Considerations

- Generate 32-character random tokens for URL-based authentication
- Each terminal session has a unique secret path (e.g., `/AbCdEf...`)
- Only users with the complete URL can access the terminal
- No password/username authentication due to ttyd WebSocket limitations
- Suitable for temporary sessions; use Cloudflare Access for production

### Known Issues & Workarounds

1. **ttyd WebSocket Authentication Bug**
   - Issue: ttyd's credential authentication fails through proxies
   - Workaround: Using URL path-based security instead
   - Reference: https://github.com/tsl0922/ttyd/issues/1293

2. **Mobile Compatibility**
   - Works perfectly with the URL-based authentication approach
   - No CORS or WebSocket issues on mobile Safari

### Binary Sources

- **ttyd**: Currently using system-installed version via Homebrew
  - Future: Use official releases from https://github.com/tsl0922/ttyd
- **cloudflared**: Downloaded from official Cloudflare releases
  - Version specified in `scripts/download-binaries.sh`