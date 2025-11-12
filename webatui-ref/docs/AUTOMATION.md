# Automation & Build System

Comprehensive guide to the webatui project's automation system using `just` and `nushell` scripts.

## Overview

This project uses a two-tier automation system:

1. **justfile**: High-level task runner with simple commands
2. **Nushell scripts**: Detailed automation with advanced options and error handling

## Quick Start

```bash
# Show all available commands
just

# Build and serve WASM locally
just build-wasm
just serve

# Run development workflow
just dev

# Run production workflow
just prod

# Deploy to GitHub Pages
just deploy
```

## justfile Commands

### Build Commands

```bash
just build              # Build in debug mode
just build-release      # Build in release mode
just build-wasm         # Build WASM with wasm-pack
just build-wasm-pack    # Build WASM (web target)
just build-wasm-release # Build WASM (optimized)
```

### Run Commands

```bash
just run [ARGS]              # Run default binary
just run-example NAME [ARGS] # Run specific example
just serve [PORT]            # Serve WASM (default: 8080)
```

### Test Commands

```bash
just test           # Run all tests
just test-all       # Run tests with verbose output
just test-wasm      # Run WASM tests
just test-coverage  # Generate coverage report
just test-one TEST  # Run specific test
```

### Check Commands

```bash
just check      # Run cargo check
just check-wasm # Check WASM target
just check-all  # Check all targets and features
```

### Format & Lint

```bash
just fmt          # Format code
just fmt-check    # Check formatting
just lint         # Run clippy
just lint-strict  # Run clippy (strict)
just lint-fix     # Fix clippy warnings automatically
```

### Clean Commands

```bash
just clean      # Clean build artifacts
just clean-wasm # Clean WASM artifacts
just clean-all  # Deep clean (includes node_modules)
```

### Watch Commands

```bash
just watch              # Watch and rebuild
just watch-test         # Watch and run tests
just watch-example NAME # Watch specific example
just watch-wasm         # Watch WASM build
```

### Install Commands

```bash
just install-deps       # Install dev dependencies
just install-wasm-tools # Install WASM toolchain
just install-all        # Install everything
```

### Workflow Commands

```bash
just dev   # Development: fmt, lint, test
just prod  # Production: fmt-check, lint-strict, test, build-release
just ci    # CI: all checks
just dev-wasm # WASM dev workflow
```

### Documentation

```bash
just docs         # Build documentation
just docs-open    # Build and open docs
just docs-private # Build docs with private items
```

### Stats & Info

```bash
just stats    # Show project statistics
just status   # Show project status
just info     # Show tool versions
just deps     # Show dependency tree
just outdated # Check outdated dependencies
```

### WASM Specific

```bash
just wasm-web      # Build WASM for web
just wasm-bundler  # Build WASM for bundler
just wasm-node     # Build WASM for Node.js
just wasm-test-browser # Test in browser
just wasm-optimize # Optimize WASM bundle
```

### Server Commands

```bash
just server [PORT]     # Start HTTP server
just server-pkg [PORT] # Serve pkg/ directory
```

### Deploy

```bash
just deploy       # Deploy to GitHub Pages
just deploy-build # Build for deployment
```

### Benchmarks

```bash
just bench           # Run benchmarks
just bench-one NAME  # Run specific benchmark
```

### Audit & Security

```bash
just audit  # Audit dependencies
just update # Update dependencies
```

### Utilities

```bash
just init-wasm      # Initialize WASM project structure
just new-example NAME # Create new example
just tags           # Generate ctags
just help           # Show help
```

### Advanced

```bash
just profile-build # Profile build time
just timings       # Show compilation times
just expand        # Expand macros
just size          # Check binary size
just strip         # Strip binary
just asm           # Show assembly output
```

## Nushell Scripts

For advanced usage with more options, use nushell scripts directly.

### scripts/build.nu

Build automation with multiple targets and optimization.

```bash
# Build in release mode
nu scripts/build.nu --release

# Build WASM with wasm-pack and optimize
nu scripts/build.nu --wasm --pack --release --optimize

# Build for specific target
nu scripts/build.nu --target x86_64-pc-windows-gnu --release

# Show all options
nu scripts/build.nu --help
```

**Options:**
- `--release`: Build in release mode
- `--wasm`: Build for WASM target
- `--debug`: Build in debug mode (default)
- `--target TARGET`: Specify target triple
- `--features FEATS`: Enable specific features
- `--optimize`: Optimize WASM with wasm-opt
- `--pack`: Use wasm-pack for building

### scripts/serve.nu

Serve WASM builds with automatic server detection.

```bash
# Serve with all features
nu scripts/serve.nu --port 3000 --open --cors --spa

# Serve specific directory
nu scripts/serve.nu --dir www --port 8080

# Show all options
nu scripts/serve.nu --help
```

**Options:**
- `--port PORT`: Port to serve on (default: 8080)
- `--dir DIR`: Directory to serve (default: pkg)
- `--host HOST`: Host to bind to (default: 0.0.0.0)
- `--open`: Open browser after starting
- `--cors`: Enable CORS headers
- `--spa`: Enable SPA mode (fallback to index.html)

**Supported Servers:**
- Python 3 (http.server)
- Python 2 (SimpleHTTPServer)
- basic-http-server
- http-server (Node.js)

### scripts/deploy.nu

Deploy to various hosting platforms.

```bash
# Deploy to GitHub Pages
nu scripts/deploy.nu

# Deploy to Netlify
nu scripts/deploy.nu --target netlify

# Dry run
nu scripts/deploy.nu --dry-run

# Show all options
nu scripts/deploy.nu --help
```

**Options:**
- `--target TARGET`: Deployment target (github-pages, netlify, vercel)
- `--branch BRANCH`: Git branch for GitHub Pages (default: gh-pages)
- `--dir DIR`: Directory to deploy (default: pkg)
- `--message MSG`: Commit message
- `--dry-run`: Show what would be deployed

**Targets:**
- `github-pages`: GitHub Pages (free hosting)
- `netlify`: Netlify (requires CLI)
- `vercel`: Vercel (requires CLI)

### scripts/test.nu

Comprehensive test runner.

```bash
# Run all tests with coverage
nu scripts/test.nu --coverage

# Run WASM tests
nu scripts/test.nu --wasm

# Run benchmarks
nu scripts/test.nu --bench

# Show all options
nu scripts/test.nu --help
```

**Options:**
- `--all`: Run all tests (unit, integration, doc)
- `--unit`: Run unit tests only
- `--integration`: Run integration tests only
- `--doc`: Run documentation tests only
- `--wasm`: Run WASM-specific tests
- `--bench`: Run benchmarks
- `--coverage`: Generate coverage report
- `--filter PATTERN`: Run tests matching pattern
- `--nocapture`: Show test output
- `--release`: Run tests in release mode

### scripts/clean.nu

Cleanup build artifacts.

```bash
# Clean everything
nu scripts/clean.nu --all

# Clean specific artifacts
nu scripts/clean.nu --cargo --wasm

# Dry run
nu scripts/clean.nu --all --dry-run

# Show all options
nu scripts/clean.nu --help
```

**Options:**
- `--all`: Clean everything
- `--cargo`: Clean cargo artifacts (target/)
- `--wasm`: Clean WASM artifacts (pkg/, wasm target)
- `--node`: Clean node_modules
- `--coverage`: Clean coverage reports
- `--temp`: Clean temporary files
- `--dry-run`: Show what would be deleted

## Common Workflows

### Development Workflow

```bash
# 1. Initial setup
just install-all

# 2. Development iteration
just watch         # Terminal 1: auto-rebuild
just serve --open  # Terminal 2: serve with live reload

# 3. Before commit
just dev          # Format, lint, test
```

### WASM Development

```bash
# 1. Build WASM
just build-wasm-release

# 2. Serve locally
just serve 8080

# 3. Test in browser
just wasm-test-browser

# 4. Deploy
just deploy
```

### Testing Workflow

```bash
# Run all tests
just test-all

# Run with coverage
just test-coverage

# Run specific test
just test-one my_test_name

# Watch mode
just watch-test
```

### Production Build

```bash
# Full production workflow
just prod

# Or step by step
just fmt-check
just lint-strict
just test-all
just build-release
```

### CI/CD Pipeline

```bash
# Run all CI checks
just ci

# Individual steps
just fmt-check
just lint-strict
just test-all
just check-all
just build-release
```

## Advanced Usage

### Custom Build Targets

```bash
# Windows cross-compilation
nu scripts/build.nu --target x86_64-pc-windows-gnu --release

# Linux cross-compilation
nu scripts/build.nu --target x86_64-unknown-linux-gnu --release

# macOS cross-compilation
nu scripts/build.nu --target x86_64-apple-darwin --release
```

### WASM Optimization

```bash
# Build with maximum optimization
nu scripts/build.nu --wasm --pack --release --optimize

# Or using justfile
just build-wasm-release
just wasm-optimize
```

### Deployment Options

```bash
# GitHub Pages with custom branch
nu scripts/deploy.nu --branch main --message "Deploy v1.0.0"

# Netlify deployment
nu scripts/deploy.nu --target netlify --dir pkg

# Dry run to preview
nu scripts/deploy.nu --dry-run
```

### Coverage Analysis

```bash
# Generate HTML coverage report
just test-coverage

# View report (automatically opens)
open coverage/index.html
```

## Prerequisites

### Required Tools

- Rust and Cargo
- Nushell (for scripts)
- just (task runner)

### Optional Tools

```bash
# WASM development
rustup target add wasm32-unknown-unknown
cargo install wasm-pack wasm-bindgen-cli wasm-opt

# Testing
cargo install cargo-tarpaulin cargo-watch

# HTTP servers
cargo install basic-http-server
npm install -g http-server

# Deployment
npm install -g netlify-cli vercel

# Documentation
cargo install cargo-outdated cargo-audit
```

## Installation

### Install just

```bash
# macOS
brew install just

# Linux
cargo install just

# Windows
scoop install just
```

### Install Nushell

```bash
# macOS
brew install nushell

# Linux
cargo install nu

# Windows
winget install nushell
```

### Install Project Dependencies

```bash
just install-all
```

## Tips & Best Practices

1. **Use justfile for common tasks**: Shorter, easier to remember commands.

2. **Use nushell scripts for advanced options**: More flexibility and detailed output.

3. **Enable watch mode during development**: Faster iteration cycle.

4. **Always run `just dev` before committing**: Ensures code quality.

5. **Use `--dry-run` for destructive operations**: Preview changes before applying.

6. **Check help for all options**: Every script has comprehensive `--help`.

7. **Use colorful output**: Scripts provide status-based colors for clarity.

8. **Leverage parallel commands**: justfile runs independent tasks in parallel.

## Troubleshooting

### just command not found

```bash
cargo install just
# or
brew install just
```

### Nushell scripts fail

```bash
# Check nushell installation
nu --version

# Make scripts executable
chmod +x scripts/*.nu

# Run with nu explicitly
nu scripts/build.nu --help
```

### WASM build fails

```bash
# Install WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-pack
cargo install wasm-pack
```

### Server won't start

```bash
# Install a HTTP server
cargo install basic-http-server
# or
pip install http-server
```

### Tests fail

```bash
# Check test dependencies
cargo test --all-features

# Run with verbose output
just test-all
```

## Performance Tips

1. **Use release mode for benchmarks**: `just bench`
2. **Profile builds**: `just profile-build`
3. **Check binary size**: `just size`
4. **Optimize WASM**: `just wasm-optimize`
5. **Use watch mode**: Incremental compilation is faster

## Integration with IDEs

### VS Code

Add to `.vscode/tasks.json`:

```json
{
  "version": "2.0.0",
  "tasks": [
    {
      "label": "just dev",
      "type": "shell",
      "command": "just dev",
      "group": "build"
    },
    {
      "label": "just test",
      "type": "shell",
      "command": "just test",
      "group": "test"
    }
  ]
}
```

### IntelliJ IDEA / RustRover

Add external tools for just commands.

## Resources

- [justfile Documentation](https://github.com/casey/just)
- [Nushell Documentation](https://www.nushell.sh/)
- [wasm-pack Documentation](https://rustwasm.github.io/wasm-pack/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)

## Contributing

When adding new automation:

1. Add command to justfile for common cases
2. Create/update nushell script for advanced options
3. Update this documentation
4. Add examples to scripts/README.md
5. Include help messages in scripts

## License

Same as parent project.
