# CodeTunnel

<p align="center">
  <img src="apps/desktop/src/assets/logo-no-text.svg" alt="CodeTunnel Logo" width="128" height="128">
</p>

<p align="center">
  <strong>Instant, secure development tunnels for web terminals</strong><br>
  <em>Perfect for AI-assisted coding from anywhere 🌴</em>
</p>

<p align="center">
  <a href="https://github.com/ifokeev/codetunnel/releases/latest">
    <img src="https://img.shields.io/github/v/release/ifokeev/codetunnel?style=flat-square" alt="Latest Release">
  </a>
  <a href="https://github.com/ifokeev/codetunnel/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/ifokeev/codetunnel?style=flat-square" alt="License">
  </a>
  <a href="https://github.com/ifokeev/codetunnel/actions">
    <img src="https://img.shields.io/github/actions/workflow/status/ifokeev/codetunnel/build.yml?style=flat-square" alt="Build Status">
  </a>
  <a href="https://ifokeev.github.io/codetunnel/">
    <img src="https://img.shields.io/badge/website-live-brightgreen?style=flat-square" alt="Website">
  </a>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#usage">Usage</a> •
  <a href="#building">Building</a> •
  <a href="#security">Security</a> •
  <a href="#contributing">Contributing</a>
</p>

## Overview

CodeTunnel is a cross-platform desktop application that provides instant, secure web-based terminal access for developers. Built with Tauri 2.0, it bundles [ttyd](https://github.com/tsl0922/ttyd) and [cloudflared](https://github.com/cloudflare/cloudflared) to create secure development tunnels that make your local terminal accessible from anywhere.

## Features

- 🌴 **Hawaiian-Themed UI** - Beautiful tropical design with palm tree branding
- 🚀 **Instant Terminal Access** - Launch a web terminal in seconds
- 🌐 **Secure Tunnels** - Powered by Cloudflare's global network
- 🔒 **URL-based Security** - Each session protected by a unique 32-character token
- 📱 **Mobile Friendly** - Works perfectly with AI coding assistants on iPad/mobile
- 💻 **Cross-Platform** - Native apps for macOS, Windows, and Linux
- 🤖 **AI Development Ready** - Optimized for Claude Code, Gemini CLI, and other AI tools
- ⚡ **Lightning Fast** - Minimal latency with Rust backend

## Installation

### Download Pre-built Binaries

Visit our [website](https://ifokeev.github.io/codetunnel/) or [GitHub Releases](https://github.com/ifokeev/codetunnel/releases) to download:

#### macOS
- Download: `CodeTunnel_x.x.x_universal.dmg` (Intel & Apple Silicon)
- First run: Right-click and select "Open" to bypass Gatekeeper
- Or remove quarantine: `xattr -cr /Applications/CodeTunnel.app`

#### Windows
- Download: `CodeTunnel_x.x.x_x64-setup.exe` or `.msi`
- Run installer and follow the wizard
- First run: Click "More info" → "Run anyway" if SmartScreen appears

#### Linux
- **AppImage** (recommended): `CodeTunnel_x.x.x_amd64.AppImage`
  ```bash
  chmod +x CodeTunnel_*.AppImage
  ./CodeTunnel_*.AppImage
  ```
- **Debian/Ubuntu**: `codetunnel_x.x.x_amd64.deb`
  ```bash
  sudo dpkg -i codetunnel_*.deb
  ```

### Install from Source

See the [Building](#building) section below.

## Usage

1. **Launch CodeTunnel** - Open the application
2. **Start Terminal** - Click "Start Terminal" to begin a session
3. **Share URL** - Copy the secure URL to access your terminal from any browser
4. **Access Terminal** - Open the URL in a web browser to access your terminal
5. **Stop Terminal** - Click "Stop Terminal" when done

### Security Notice

Each terminal session is protected by a unique URL containing a 32-character random token. Only users with the exact URL can access the terminal. Keep URLs private and stop sessions when not in use.

## Building

### Prerequisites

- [Node.js](https://nodejs.org/) (v18+)
- [pnpm](https://pnpm.io/)
- [Rust](https://rustup.rs/)
- Platform-specific requirements:
  - **macOS**: Xcode Command Line Tools
  - **Windows**: Visual Studio Build Tools
  - **Linux**: `webkit2gtk-4.0`, `libssl-dev`

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/ifokeev/codetunnel.git
   cd codetunnel
   ```

2. Install dependencies:
   ```bash
   pnpm install
   ```

3. Download platform binaries:
   ```bash
   ./scripts/download-binaries.sh
   ```

   For macOS distribution builds:
   ```bash
   ./scripts/build-ttyd-macos.sh
   ```

### Development

Run in development mode with hot reload:
```bash
pnpm tauri:dev
```

### Production Build

Build for current platform:
```bash
pnpm tauri:build
```

Build for specific platforms:
```bash
# macOS Universal
pnpm tauri:build -- -- --target universal-apple-darwin

# Windows x64
pnpm tauri:build -- -- --target x86_64-pc-windows-msvc

# Linux x64
pnpm tauri:build -- -- --target x86_64-unknown-linux-gnu
```

## Project Structure

```
codetunnel/
├── apps/
│   └── desktop/          # Tauri desktop application
│       ├── src/          # React frontend
│       └── src-tauri/    # Rust backend
├── packages/             # Shared packages (future)
├── scripts/              # Build and utility scripts
└── pnpm-workspace.yaml   # Monorepo configuration
```

## Security

CodeTunnel implements several security measures:

- **URL-based Authentication**: Each session uses a cryptographically random 32-character token
- **Local Process Isolation**: Terminal processes run with user permissions
- **No Persistent Storage**: Sessions are temporary and not saved
- **HTTPS Only**: Cloudflare tunnels provide encrypted connections

### Best Practices

- Only share URLs with trusted users
- Stop sessions when not in use
- Use Cloudflare Access for production deployments
- Keep the application updated

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [ttyd](https://github.com/tsl0922/ttyd) - Terminal server
- [cloudflared](https://github.com/cloudflare/cloudflared) - Tunnel client
- [Tauri](https://tauri.app/) - Desktop application framework
- [React](https://reactjs.org/) - UI framework

## Support

- 📖 [Documentation](https://github.com/ifokeev/codetunnel/wiki)
- 🐛 [Issue Tracker](https://github.com/ifokeev/codetunnel/issues)
- 💬 [Discussions](https://github.com/ifokeev/codetunnel/discussions)

## Roadmap

- [ ] Auto-updater with signature verification
- [ ] Multiple concurrent terminal sessions
- [ ] Session persistence and reconnection
- [ ] Custom shell configuration
- [ ] Package manager distribution (Homebrew, Scoop, etc.)
- [ ] Browser extension for quick access
- [ ] Team collaboration features

---

<p align="center">
  <strong>Aloha! 🌺</strong><br>
  Made with ❤️ and 🌴 by the CodeTunnel team
</p>
