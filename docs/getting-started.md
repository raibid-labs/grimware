# Getting Started with Grimware

Universal setup guide for all reference implementations in this repository.

## Prerequisites

### Required for All Projects

**Rust** (latest stable)
```bash
# Install via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

**Git**
```bash
# Verify installation
git --version

# If not installed, see: https://git-scm.com/downloads
```

### Project-Specific Requirements

| Project | Additional Requirements |
|---------|------------------------|
| **Bevy MCP** | Claude Code CLI |
| **Bevy MCP Ratatui** | Claude Code CLI, 24-bit color terminal |
| **Tauri** | Node.js 18+, platform-specific tools |
| **WebATUI** | Optional: Just, Bacon |

## Quick Setup by Project

### Bevy MCP

```bash
cd bevy-mcp-ref

# Install Claude Code (if not already)
# See: https://docs.claude.com/en/docs/claude-code

# Run with BRP
cargo run --features brp

# Or run demo
cargo run --example brp_demo --features brp
```

**Next steps**: See [Bevy MCP Guide](./bevy-mcp.md)

### Bevy MCP Ratatui

```bash
cd bevy-mcp-ratatui-ref

# Check terminal color support
echo $TERM  # Should show xterm-256color or similar

# Run basic TUI example
cargo run --example tui_basic --features tui

# Run with AI control
cargo run --example tui_brp --features full
```

**Recommended terminals**: Alacritty, Kitty, iTerm2, WezTerm

**Next steps**: See [Bevy MCP Ratatui Guide](./bevy-mcp-ratatui.md)

### Tauri

```bash
cd tauri-ref

# Install Node.js dependencies
npm install

# Desktop development
npm run tauri:dev

# Build for production
npm run tauri:build
```

**Platform-specific setup**: See [docs/SETUP.md](../tauri-ref/docs/SETUP.md)

**Next steps**: See [Tauri Guide](./tauri.md)

### WebATUI

```bash
cd webatui-ref

# Optional: Install just for task automation
cargo install just

# Optional: Install bacon for file watching
cargo install bacon

# Install development tools (if using just)
just install-deps

# Run an example
cargo run --example basic --features terminal
# or
just example basic
```

**Next steps**: See [WebATUI Guide](./webatui.md)

## Development Tools

### Recommended IDE Setup

**VS Code** with extensions:
- `rust-analyzer` - Rust language support
- `Even Better TOML` - TOML file support
- `Error Lens` - Inline error display

**RustRover / IntelliJ IDEA** with:
- Rust plugin
- Tauri plugin (for Tauri projects)

### Useful Cargo Tools

```bash
# Fast incremental builds
cargo install cargo-watch

# Better error messages
cargo install cargo-expand

# Code coverage
cargo install cargo-tarpaulin

# Dependency tree visualization
cargo install cargo-tree

# Security audits
cargo install cargo-audit
```

### Platform-Specific Tools

**macOS**:
```bash
# Xcode Command Line Tools
xcode-select --install

# Homebrew (for additional tools)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

**Linux** (Ubuntu/Debian):
```bash
# Build essentials
sudo apt update
sudo apt install build-essential pkg-config libssl-dev

# For Tauri (if needed)
sudo apt install libwebkit2gtk-4.0-dev \
  libgtk-3-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

**Windows**:
- Visual Studio Build Tools or Visual Studio Community
- See individual project docs for specific requirements

## Verification Steps

### Test Rust Installation

```bash
# Create test project
cargo new test-project
cd test-project

# Build and run
cargo run

# Should output: "Hello, world!"
```

### Test Bevy Projects

```bash
cd bevy-mcp-ref

# Check compilation
cargo check --all-features

# Run tests
cargo test

# Should compile without errors
```

### Test Tauri Project

```bash
cd tauri-ref

# Check frontend
npm run build

# Check Rust backend
cd src-tauri && cargo check

# Should build successfully
```

### Test WebATUI Project

```bash
cd webatui-ref

# Test native compilation
cargo check --features terminal

# Test WASM compilation
cargo check --target wasm32-unknown-unknown --no-default-features

# Run tests
cargo test --features terminal
```

## Common Issues

### Rust Linker Errors

**Problem**: `error: linker 'cc' not found`

**Solution**:
```bash
# macOS
xcode-select --install

# Linux
sudo apt install build-essential

# Windows
# Install Visual Studio Build Tools
```

### Bevy Compilation Slow

**Problem**: First Bevy compilation takes 10+ minutes

**Solution**: Use dynamic linking for faster dev builds
```bash
cargo run --features bevy/dynamic_linking
```

### WASM Target Missing

**Problem**: `error: can't find crate for 'std'`

**Solution**: Add WASM target
```bash
rustup target add wasm32-unknown-unknown
```

### Tauri Android Build Fails

**Problem**: Android build errors

**Solution**: Ensure Android setup is complete
```bash
# Check environment
echo $ANDROID_HOME
echo $JAVA_HOME

# Sync dependencies
cd tauri-ref
npm run tauri android init
```

### Terminal Colors Not Working

**Problem**: Terminal rendering shows no colors

**Solution**: Check terminal capabilities
```bash
# Test 24-bit color support
printf "\x1b[38;2;255;100;0mTRUECOLOR\x1b[0m\n"

# Should display "TRUECOLOR" in orange
```

## Next Steps

After initial setup:

1. **Choose a project** based on your use case
2. **Read the specific guide** for that project
3. **Run the examples** to understand capabilities
4. **Review the architecture** to learn patterns
5. **Start building** with the reference as a guide

## Learning Resources

### Rust Fundamentals
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises

### Framework-Specific
- [Bevy Quick Start](https://bevyengine.org/learn/quick-start/introduction/)
- [Tauri Guides](https://tauri.app/v1/guides/)
- [Ratatui Tutorial](https://ratatui.rs/tutorial/)

### AI Integration
- [Claude Code Documentation](https://docs.claude.com/en/docs/claude-code)
- [MCP Documentation](https://modelcontextprotocol.io/)

## Getting Help

### Documentation
Each project has extensive documentation:
- Project README - Overview and quick start
- Project docs/ folder - Detailed guides
- CLAUDE.md - AI assistant configuration
- This docs/ folder - Consolidated guides

### Community Resources
- GitHub Issues - Report bugs and request features
- Discussions - Ask questions and share ideas

### Debugging Tips

1. **Start simple**: Run basic examples first
2. **Check logs**: Use `RUST_LOG=debug` for verbose output
3. **Isolate issues**: Test components individually
4. **Read errors carefully**: Rust errors are descriptive
5. **Search existing issues**: Problem might be documented

## Development Workflow

### Typical Session

```bash
# 1. Update repository
git pull

# 2. Navigate to project
cd <project-name>

# 3. Check for updates
cargo update

# 4. Run in development mode
cargo run --features <features>

# 5. Make changes and test
cargo test

# 6. Format and lint
cargo fmt
cargo clippy --all-features
```

### Best Practices

- **Commit often**: Small, focused commits
- **Write tests**: Test new functionality
- **Document changes**: Update relevant docs
- **Follow conventions**: Match existing code style
- **Check before pushing**: Run tests and lints

---

**Ready to start?** Choose a project from the [main README](../README.md) and dive in!
