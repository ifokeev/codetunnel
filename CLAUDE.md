# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

CodeTunnel is a monorepo project that provides instant, secure development tunnels for web terminals. It's structured using pnpm workspaces to allow for future expansion with multiple applications and shared packages.

## Repository Structure

```
codetunnel/
├── apps/
│   └── desktop/        # Tauri 2.0 desktop application
├── packages/          # Shared packages (future)
├── scripts/           # Root-level scripts
├── pnpm-workspace.yaml
└── package.json
```

## Workspace Management

- **Package Manager**: pnpm with workspaces
- **Root Commands**: All commands in the root package.json proxy to the desktop app
- **Dependencies**: Run `pnpm install` from root to install all workspace dependencies

## Applications

### Desktop App (`apps/desktop/`)
The main Tauri 2.0 desktop application that bundles ttyd and cloudflared binaries. See `apps/desktop/CLAUDE.md` for detailed information about the desktop app architecture and development.

## Quick Start

```bash
# Install dependencies
pnpm install

# Run desktop app in development
pnpm tauri:dev

# Build desktop app for production
pnpm tauri:build
```