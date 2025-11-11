# Quick Reference - webatui Automation

Fast lookup for common commands and workflows.

## 🚀 Quick Start

```bash
just                    # Show all commands
just build-wasm         # Build WASM
just serve              # Serve locally on :8080
just dev                # Run dev workflow (fmt, lint, test)
just deploy             # Deploy to GitHub Pages
```

## 📦 Build

```bash
just build              # Debug build
just build-release      # Release build
just build-wasm         # WASM build
just build-wasm-release # WASM release (optimized)
```

## 🧪 Test

```bash
just test               # Run tests
just test-all           # Run tests (verbose)
just test-coverage      # Generate coverage
just watch-test         # Watch and test
```

## 🔍 Check & Lint

```bash
just check              # Cargo check
just fmt                # Format code
just fmt-check          # Check formatting
just lint               # Run clippy
just lint-strict        # Strict clippy
```

## 🧹 Clean

```bash
just clean              # Clean build artifacts
just clean-wasm         # Clean WASM
just clean-all          # Deep clean
```

## 🌐 Serve & Deploy

```bash
just serve              # Serve on :8080
just serve 3000         # Serve on :3000
just deploy             # Deploy to GitHub Pages
```

## 👀 Watch

```bash
just watch              # Watch and rebuild
just watch-test         # Watch and test
just watch-wasm         # Watch WASM
```

## 📚 Documentation

```bash
just docs               # Build docs
just docs-open          # Build and open docs
```

## 📊 Info & Stats

```bash
just info               # Tool versions
just stats              # Project stats
just status             # Git & dependencies status
just deps               # Dependency tree
```

## 🔧 Workflows

```bash
just dev                # fmt → lint → test
just prod               # fmt-check → lint-strict → test → build-release
just ci                 # All CI checks
just dev-wasm           # WASM development workflow
```

## 🎯 Common Patterns

### Development Iteration
```bash
# Terminal 1: Auto-rebuild
just watch

# Terminal 2: Serve with auto-reload
just serve --open
```

### WASM Development
```bash
just build-wasm-release # Build
just serve 8080         # Serve
just deploy             # Deploy
```

### Before Commit
```bash
just dev                # Full dev workflow
# or
just fmt && just lint && just test
```

### Production Release
```bash
just prod               # Full production workflow
# or
just fmt-check && just lint-strict && just test-all && just build-release
```

## 🛠️ Nushell Scripts (Advanced)

### Build with Options
```bash
nu scripts/build.nu --wasm --pack --release --optimize
nu scripts/build.nu --target x86_64-pc-windows-gnu --release
```

### Serve with Options
```bash
nu scripts/serve.nu --port 3000 --open --cors --spa
nu scripts/serve.nu --dir www --host localhost
```

### Deploy with Options
```bash
nu scripts/deploy.nu --target netlify
nu scripts/deploy.nu --dry-run
nu scripts/deploy.nu --branch main --message "v1.0.0"
```

### Test with Options
```bash
nu scripts/test.nu --coverage
nu scripts/test.nu --wasm
nu scripts/test.nu --filter my_test --nocapture
```

### Clean with Options
```bash
nu scripts/clean.nu --all --dry-run
nu scripts/clean.nu --cargo --wasm
nu scripts/clean.nu --temp
```

## 📋 Installation

```bash
# Install task runner
brew install just       # macOS
cargo install just      # Cross-platform

# Install nushell
brew install nushell    # macOS
cargo install nu        # Cross-platform

# Install project tools
just install-all
```

## 🔑 Key Files

| File | Purpose |
|------|---------|
| `justfile` | High-level task automation |
| `scripts/build.nu` | Build automation |
| `scripts/serve.nu` | HTTP server for WASM |
| `scripts/deploy.nu` | Deployment automation |
| `scripts/test.nu` | Test runner |
| `scripts/clean.nu` | Cleanup automation |
| `docs/AUTOMATION.md` | Full documentation |
| `scripts/README.md` | Scripts documentation |

## 💡 Tips

- Use `just` for quick tasks
- Use `nu scripts/*.nu --help` for advanced options
- Run `just dev` before committing
- Use `--dry-run` for preview (deploy, clean)
- Enable watch mode during development
- Check `just --list` for all commands

## 🆘 Help

```bash
just --list             # List all just commands
just help               # Show help with descriptions
nu scripts/build.nu --help    # Build script help
nu scripts/serve.nu --help    # Serve script help
nu scripts/deploy.nu --help   # Deploy script help
nu scripts/test.nu --help     # Test script help
nu scripts/clean.nu --help    # Clean script help
```

## 📖 Full Documentation

- [docs/AUTOMATION.md](docs/AUTOMATION.md) - Complete automation guide
- [scripts/README.md](scripts/README.md) - Nushell scripts documentation
- [docs/STRUCTURE.md](docs/STRUCTURE.md) - Project structure

## 🎨 Color Output

All scripts provide colorful output:
- 🟢 Green: Success
- 🔵 Blue: Information
- 🟡 Yellow: Warnings
- 🔴 Red: Errors
- 🔷 Cyan: Steps
- 🟣 Purple: Commands

---

**Pro Tip**: Add this to your shell aliases:

```bash
# ~/.bashrc or ~/.zshrc
alias jb='just build'
alias jt='just test'
alias js='just serve'
alias jd='just deploy'
alias jw='just watch'
```
