# webatui-ref

**Reference implementation for webatui - Terminal UI that works in both terminal and browser**

A library demonstrating how to build terminal UI applications with [Ratatui](https://ratatui.rs/) that can run in both native terminals and web browsers via WebAssembly.

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![WASM Compatible](https://img.shields.io/badge/WASM-compatible-green.svg)](https://webassembly.org/)

---

## 🚀 Features

- **Terminal Support**: Full ratatui-based terminal UI with crossterm
- **Web Support**: WASM compilation ready (Yew components)
- **State Management**: Unified state across platforms
- **Component System**: Reusable UI widgets
- **Examples**: Multiple working examples
- **Well Tested**: Comprehensive test suite for both native and WASM

## ⚡ Quick Start

### Prerequisites

- Rust 1.75+ - [Install here](https://www.rust-lang.org/tools/install)
- (Optional) [Just](https://github.com/casey/just) - `cargo install just`

### Installation

```bash
# Clone the repository
git clone https://github.com/raibid-labs/webatui-ref.git
cd webatui-ref

# Install development tools (bacon, wasm-pack, etc.)
just install-deps
```

### Running Examples

**This is a library crate** - run the examples to see it in action:

```bash
# List available examples
just list-examples

# Run an example
just example basic
just example dashboard
just example interactive

# Or use cargo directly
cargo run --example basic --features terminal
```

## 📚 Documentation

- **[Quick Start Guide](docs/QUICK_START.md)** - Get started in 5 minutes
- **[Development Guide](docs/DEVELOPMENT.md)** - Complete development workflow
- **[WASM Testing Setup](docs/WASM_TESTING_SETUP.md)** - WASM configuration details
- **[Apple Silicon Setup](docs/APPLE_SILICON_SETUP.md)** - M1/M2/M3 Mac compatibility

## 🏗️ Project Structure

```
webatui-ref/
├── src/
│   ├── lib.rs           # Library entry point
│   ├── state.rs         # Core state (platform-independent)
│   ├── components/      # Terminal UI components
│   └── screens/         # Terminal UI screens
├── examples/
│   ├── basic.rs         # Simple terminal UI
│   ├── dashboard.rs     # Dashboard with metrics
│   └── interactive.rs   # Interactive UI demo
├── tests/
│   ├── integration_test.rs  # Native tests
│   └── wasm_tests.rs        # WASM-specific tests
├── docs/                # Documentation
├── .bacon/              # Bacon file watcher config
└── justfile             # Task automation
```

## 🛠️ Development

### Watch Mode

```bash
# Watch and rebuild
just watch

# Watch and run tests
just watch-test

# Watch specific example
just watch-example basic
```

### Testing

```bash
# Run all tests
just test

# Test WASM compilation
just wasm-build

# Test in browser (requires ChromeDriver)
just wasm-test-browser
```

### Building

```bash
# Build for terminal (native)
cargo build --features terminal

# Build for web (WASM)
just wasm-build

# Build release
cargo build --release --features terminal
```

## 📦 Using as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
webatui-ref = { git = "https://github.com/raibid-labs/webatui-ref", features = ["terminal"] }
```

Example usage:

```rust
use webatui_ref::prelude::*;

fn main() -> anyhow::Result<()> {
    let mut state = AppState::default();
    state.update(Message::Navigate(Screen::Dashboard));
    assert_eq!(state.current_screen, Screen::Dashboard);
    Ok(())
}
```

## 🎯 Features

The crate uses feature flags for platform-specific dependencies:

- **`terminal`** - Terminal UI support (ratatui, crossterm) - Default on native
- **`web`** - Web/WASM support (yew, wasm-bindgen)
- **`examples`** - Additional example features

## 🔧 Available Commands

Run `just --list` to see all available commands. Key commands:

| Command | Description |
|---------|-------------|
| `just list-examples` | Show all examples |
| `just example <name>` | Run specific example |
| `just test` | Run all tests |
| `just watch` | Watch and rebuild |
| `just watch-test` | Watch and run tests |
| `just wasm-build` | Build for WASM |
| `just fmt` | Format code |
| `just lint` | Run clippy |

### Bacon (File Watcher)

The project uses [bacon](https://dystroy.org/bacon/) for file watching (Apple Silicon compatible):

```bash
bacon                # Build with watch
bacon test           # Test with watch
bacon example-basic  # Run example with watch
bacon wasm-check     # Check WASM compilation
```

See `.bacon/bacon.toml` for all configured jobs.

## 📖 Examples

### Basic Example

Simple terminal UI demonstrating core functionality:

```bash
just example basic
```

Features: Text display, keyboard events, state updates

### Dashboard Example

Comprehensive dashboard with real-time metrics:

```bash
just example dashboard
```

Features: Charts, gauges, tables, system monitoring

### Interactive Example

Advanced interaction patterns:

```bash
just example interactive
```

Features: Text input, navigation, interactive widgets

## 🧪 Testing

### Native Tests

```bash
cargo test --features terminal
```

- 6 unit tests in `src/lib.rs`
- 7 integration tests in `tests/integration_test.rs`
- Component-specific tests

### WASM Tests

```bash
# Build for WASM (verify compilation)
cargo build --target wasm32-unknown-unknown --lib --no-default-features

# Run WASM tests (requires wasm-pack)
wasm-pack test --node -- --lib --no-default-features
```

## 🖥️ Platform Support

### Native Terminals
- ✅ macOS (Intel & Apple Silicon)
- ✅ Linux
- ✅ Windows

### Terminal Emulators
- Alacritty
- Kitty
- iTerm2
- WezTerm
- Windows Terminal

### Browsers (WASM)
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+

## ⚙️ Technology Stack

### Core Dependencies

```toml
[dependencies]
ratatui = "0.29"           # Terminal UI framework
crossterm = "0.28"         # Terminal handling (native only)

# WASM support (optional)
yew = { version = "0.21", optional = true }
wasm-bindgen = { version = "0.2", optional = true }

# State management
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime
tokio = { version = "1.42", features = ["sync", "macros"], default-features = false }
```

## 🤝 Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test --features terminal`
5. Format code: `cargo fmt`
6. Run lints: `cargo clippy --features terminal`
7. Commit changes
8. Submit a pull request

## 📝 License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

## 🙏 Acknowledgments

Built with:
- [Ratatui](https://ratatui.rs/) - Terminal UI framework
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust/WASM/JS bridge
- [Yew](https://yew.rs/) - Rust web framework
- [Bacon](https://dystroy.org/bacon/) - File watcher (Apple Silicon compatible)

## 🔗 Resources

### Documentation
- [Ratatui Documentation](https://ratatui.rs/)
- [WASM Book](https://rustwasm.github.io/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)

### Community
- [GitHub Issues](https://github.com/raibid-labs/webatui-ref/issues)
- [Ratatui Discord](https://discord.gg/pMCEU9hNEj)

---

**Built by [Raibid Labs](https://github.com/raibid-labs)**

*Terminal applications that work everywhere*
