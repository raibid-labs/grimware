# Quick Start Guide

## ⚠️ Important: This is a Library Crate

This project is a **library**, not a binary application. You cannot run `cargo run` or `just run` directly. Instead, use the provided examples.

## 🚀 Running Examples

### Simple Commands

```bash
# List all examples
just list-examples

# Run an example
just example basic
just example dashboard
just example interactive
```

### Full Command

```bash
cargo run --example basic --features terminal
```

## 🔍 Watch Mode (Auto-rebuild)

```bash
# Watch and rebuild library
just watch

# Watch and run tests
just watch-test

# Watch specific example (auto-restart on changes)
just watch-example basic
```

## 📖 Using Bacon Directly

Bacon is configured with custom jobs for this project:

```bash
# Build library
bacon

# Run tests
bacon test

# Run specific example
bacon example-basic
bacon example-dashboard
bacon example-interactive

# Check WASM
bacon wasm-check
```

## 🧪 Testing

```bash
# Native tests
just test

# WASM tests
just wasm-test-browser
```

## 🌐 WASM Build

```bash
# Build for web
just wasm-build

# Build and serve
just wasm-serve
```

## 💡 Common Issues

### Error: "a bin target must be available for `cargo run`"

**Solution**: This is a library crate. Use examples:
```bash
just example basic
```

### Error: "Device not configured" when running examples

**Cause**: Running terminal UI in non-terminal environment

**Solution**: Run in actual terminal, not IDE output panel

## 🎯 Key Commands

| Command | Description |
|---------|-------------|
| `just list-examples` | Show all examples |
| `just example <name>` | Run specific example |
| `just watch-example <name>` | Watch and auto-restart example |
| `just test` | Run all tests |
| `just watch-test` | Watch tests |
| `just wasm-build` | Build for WASM |
| `bacon` | Build with watch |
| `bacon test` | Test with watch |
| `bacon example-basic` | Run example with watch |

Run `just --list` to see all available commands!
