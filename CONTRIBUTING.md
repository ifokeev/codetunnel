# Contributing to CodeTunnel

Thank you for your interest in contributing to CodeTunnel! We welcome contributions from the community and are grateful for any help you can provide.

## Code of Conduct

By participating in this project, you agree to abide by our Code of Conduct:
- Be respectful and inclusive
- Welcome newcomers and help them get started
- Focus on constructive criticism
- Respect differing viewpoints and experiences

## How to Contribute

### Reporting Issues

1. **Check existing issues** - Before creating a new issue, please check if it already exists
2. **Use issue templates** - Fill out the appropriate issue template
3. **Provide details** - Include steps to reproduce, expected behavior, and actual behavior
4. **Include environment info** - OS, version, and any relevant configuration

### Suggesting Features

1. **Open a discussion** - Start with a GitHub Discussion to gather feedback
2. **Explain the use case** - Help us understand why this feature would be useful
3. **Consider alternatives** - What other solutions have you tried?

### Contributing Code

#### Getting Started

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/ifokeev/codetunnel.git
   cd codetunnel
   ```
3. Add upstream remote:
   ```bash
   git remote add upstream https://github.com/originalowner/codetunnel.git
   ```
4. Create a feature branch:
   ```bash
   git checkout -b feature/your-feature-name
   ```

#### Development Setup

1. Install dependencies:
   ```bash
   pnpm install
   ```
2. Download binaries:
   ```bash
   ./scripts/download-binaries.sh
   ```
3. Run in development mode:
   ```bash
   pnpm tauri:dev
   ```

#### Making Changes

1. **Follow the code style** - Use the existing code as a guide
2. **Write tests** - Add tests for new functionality
3. **Update documentation** - Keep README and other docs up to date
4. **Commit messages** - Use clear, descriptive commit messages:
   ```
   feat: add new terminal theme support
   fix: resolve connection issue on Windows
   docs: update build instructions
   ```

#### Submitting Pull Requests

1. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
2. Open a Pull Request from your fork to our `main` branch
3. Fill out the PR template completely
4. Wait for review and address any feedback

### Pull Request Guidelines

- **Keep it focused** - One feature or fix per PR
- **Test thoroughly** - Ensure all tests pass on all platforms
- **Update CHANGELOG** - Add your changes to the unreleased section
- **Sign your commits** - Use `git commit -s` for DCO

## Development Guidelines

### Project Structure

```
codetunnel/
├── apps/desktop/         # Main desktop application
│   ├── src/             # React frontend
│   └── src-tauri/       # Rust backend
├── scripts/             # Build and utility scripts
└── packages/            # Shared packages (future)
```

### Technology Stack

- **Frontend**: React, TypeScript, Vite
- **Backend**: Rust, Tauri 2.0
- **Build**: pnpm workspaces, GitHub Actions

### Testing

Run tests before submitting:
```bash
# Frontend tests
cd apps/desktop && pnpm test

# Rust tests
cd apps/desktop/src-tauri && cargo test

# Lint checks
pnpm lint
```

### Platform-Specific Notes

#### macOS
- ttyd must be built from source or copied from Homebrew
- Test on both Intel and Apple Silicon

#### Windows
- Test on Windows 10 and 11
- Ensure proper code signing

#### Linux
- Test on major distributions (Ubuntu, Fedora, Arch)
- Check AppImage and .deb packages

## Release Process

1. Update version in `apps/desktop/src-tauri/Cargo.toml`
2. Update CHANGELOG.md
3. Create a PR with version bump
4. After merge, tag the release
5. GitHub Actions will build and publish

## Getting Help

- 💬 [GitHub Discussions](https://github.com/ifokeev/codetunnel/discussions)
- 🐛 [Issue Tracker](https://github.com/ifokeev/codetunnel/issues)
- 📖 [Documentation](https://github.com/ifokeev/codetunnel/wiki)

## Recognition

Contributors will be recognized in:
- The project README
- Release notes
- Our contributors page

Thank you for contributing to CodeTunnel! 🚀