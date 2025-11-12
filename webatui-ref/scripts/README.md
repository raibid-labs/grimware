# Nushell Scripts for webatui

Comprehensive automation scripts for building, testing, serving, and deploying the webatui project.

## Prerequisites

- [Nushell](https://www.nushell.sh/) installed
- Rust and Cargo
- (Optional) wasm-pack for WASM builds
- (Optional) cargo-tarpaulin for coverage
- (Optional) Netlify/Vercel CLI for deployment

## Scripts Overview

### build.nu - Build Automation

Comprehensive build automation with support for multiple targets and WASM.

```bash
# Build in debug mode
nu scripts/build.nu

# Build in release mode
nu scripts/build.nu --release

# Build WASM with wasm-pack
nu scripts/build.nu --wasm --pack --release

# Build WASM and optimize
nu scripts/build.nu --wasm --release --optimize

# Build for specific target
nu scripts/build.nu --target x86_64-pc-windows-gnu --release

# Show help
nu scripts/build.nu --help
```

### serve.nu - HTTP Server

Serve WASM builds locally with automatic server detection.

```bash
# Serve default directory (pkg) on port 8080
nu scripts/serve.nu

# Serve on custom port
nu scripts/serve.nu --port 3000

# Serve specific directory
nu scripts/serve.nu --dir www

# Open browser automatically
nu scripts/serve.nu --open

# Enable CORS for cross-origin requests
nu scripts/serve.nu --cors

# Enable SPA mode
nu scripts/serve.nu --spa --open

# Show help
nu scripts/serve.nu --help
```

Supported servers (auto-detected):
- Python 3 (built-in http.server)
- Python 2 (SimpleHTTPServer)
- basic-http-server (cargo install basic-http-server)
- http-server (npm install -g http-server)

### deploy.nu - Deployment Automation

Deploy WASM builds to various hosting platforms.

```bash
# Deploy to GitHub Pages
nu scripts/deploy.nu

# Deploy to Netlify
nu scripts/deploy.nu --target netlify

# Deploy to Vercel
nu scripts/deploy.nu --target vercel

# Dry run to see what would be deployed
nu scripts/deploy.nu --dry-run

# Deploy with custom branch and message
nu scripts/deploy.nu --branch main --message "Update site"

# Show help
nu scripts/deploy.nu --help
```

Deployment targets:
- **github-pages**: Deploy to GitHub Pages (free, automatic HTTPS)
- **netlify**: Deploy to Netlify (requires Netlify CLI)
- **vercel**: Deploy to Vercel (requires Vercel CLI)

### test.nu - Test Runner

Comprehensive test runner with coverage support.

```bash
# Run all tests
nu scripts/test.nu

# Run unit tests only
nu scripts/test.nu --unit

# Run tests matching pattern
nu scripts/test.nu --filter my_test_name

# Run tests with output
nu scripts/test.nu --nocapture

# Generate coverage report
nu scripts/test.nu --coverage

# Run WASM tests
nu scripts/test.nu --wasm

# Run benchmarks
nu scripts/test.nu --bench

# Show help
nu scripts/test.nu --help
```

### clean.nu - Cleanup Automation

Clean build artifacts and temporary files.

```bash
# Clean everything
nu scripts/clean.nu --all

# Clean only cargo artifacts
nu scripts/clean.nu --cargo

# Clean WASM and cargo
nu scripts/clean.nu --wasm --cargo

# Dry run to see what would be deleted
nu scripts/clean.nu --all --dry-run

# Clean temporary files only
nu scripts/clean.nu --temp

# Show help
nu scripts/clean.nu --help
```

Cleans:
- `target/` - Cargo build artifacts
- `pkg/` - WASM package output
- `node_modules/` - Node.js dependencies
- `coverage/` - Coverage reports
- Temporary files (*.swp, *~, .DS_Store, etc.)

## Integration with justfile

These scripts are integrated with the project's justfile:

```bash
# Using justfile commands (recommended)
just build-wasm
just serve
just deploy
just test
just clean

# Directly using nushell scripts
nu scripts/build.nu --wasm --release
nu scripts/serve.nu --open
nu scripts/deploy.nu
nu scripts/test.nu --coverage
nu scripts/clean.nu --all
```

## Features

### Colorful Output

All scripts provide colorful, informative output:
- Green: Success messages
- Blue: Information messages
- Yellow: Warnings
- Red: Errors
- Cyan: Step headers
- Purple: Commands being executed

### Error Handling

- Graceful error handling with informative messages
- Exit codes for CI/CD integration
- Tool availability checks before execution

### Dry Run Support

- `deploy.nu` and `clean.nu` support `--dry-run` flag
- See what would happen without making changes

### Smart Tool Detection

- Automatically detects available tools (Python, Node.js servers, etc.)
- Fallback to alternative tools when primary tools aren't available
- Helpful installation instructions when tools are missing

## Installation of Optional Tools

### WASM Tools

```bash
# WASM target
rustup target add wasm32-unknown-unknown

# wasm-pack (recommended)
cargo install wasm-pack

# wasm-bindgen-cli
cargo install wasm-bindgen-cli

# wasm-opt (for optimization)
cargo install wasm-opt
```

### Testing Tools

```bash
# Coverage
cargo install cargo-tarpaulin

# Watch mode
cargo install cargo-watch
```

### HTTP Servers

```bash
# Rust-based server
cargo install basic-http-server

# Node.js-based server
npm install -g http-server
```

### Deployment Tools

```bash
# Netlify CLI
npm install -g netlify-cli
netlify login

# Vercel CLI
npm install -g vercel
vercel login
```

## Examples

### Full Development Workflow

```bash
# 1. Build the project
nu scripts/build.nu --wasm --pack --release --optimize

# 2. Test locally
nu scripts/serve.nu --open --cors

# 3. Run tests
nu scripts/test.nu --all

# 4. Deploy
nu scripts/deploy.nu --target github-pages
```

### CI/CD Pipeline

```bash
# Check
cargo check --all-targets

# Build
nu scripts/build.nu --release --wasm --pack

# Test with coverage
nu scripts/test.nu --coverage

# Deploy (only on main branch)
nu scripts/deploy.nu --target netlify
```

### Quick Development Iteration

```bash
# Terminal 1: Watch and rebuild
cargo watch -x "build --target wasm32-unknown-unknown"

# Terminal 2: Serve with auto-reload
nu scripts/serve.nu --open --cors
```

## Tips

1. **Use justfile for common tasks**: The justfile provides shorter commands and additional workflows.

2. **Enable shell completion**: Nushell provides excellent tab completion for command arguments.

3. **Check help**: Every script has a `--help` flag with detailed usage information.

4. **Dry run first**: When using deploy or clean, use `--dry-run` to see what will happen.

5. **Use colorful output**: The scripts are designed to be readable with status-based colors.

## Troubleshooting

### Script doesn't run

Make sure scripts are executable:
```bash
chmod +x scripts/*.nu
```

### Nushell not found

Install Nushell:
```bash
# macOS
brew install nushell

# Linux
cargo install nu

# Windows
winget install nushell
```

### WASM build fails

Check WASM toolchain:
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### Server won't start

Install a HTTP server:
```bash
# Python 3 (usually pre-installed)
python3 --version

# Or install basic-http-server
cargo install basic-http-server
```

## Contributing

When adding new scripts:
1. Follow the existing pattern for command-line arguments
2. Add colorful output with utility functions
3. Include comprehensive help message
4. Add error checking for required tools
5. Update this README
6. Add corresponding justfile command

## License

Same as parent project.
