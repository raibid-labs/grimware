# WebATUI Reference Implementation - Project Structure

## Overview

This document describes the project structure and organization of the webatui-ref reference implementation.

## Directory Structure

```
webatui-ref/
├── Cargo.toml              # Project manifest with dependencies and features
├── src/
│   ├── lib.rs              # Library root with prelude module
│   ├── app.rs              # Main TerminalApp implementation
│   ├── state.rs            # Application state and message definitions
│   ├── components/         # Reusable UI components
│   │   ├── mod.rs
│   │   ├── header.rs       # Header component
│   │   ├── footer.rs       # Footer component
│   │   ├── list.rs         # List component
│   │   └── counter.rs      # Counter component
│   └── screens/            # Full-screen views
│       ├── mod.rs
│       ├── home.rs         # Home screen
│       ├── dashboard.rs    # Dashboard screen
│       ├── interactive.rs  # Interactive screen
│       └── settings.rs     # Settings screen
├── examples/               # Example applications
│   ├── basic.rs            # Basic terminal app
│   ├── dashboard.rs        # Dashboard example
│   ├── interactive.rs      # Interactive example
│   └── web_demo.rs         # WASM web demo
├── scripts/                # Build and utility scripts
│   └── build.nu            # Nushell build script
├── docs/                   # Documentation
│   └── STRUCTURE.md        # This file
└── tests/                  # Integration tests
```

## Key Components

### Library Structure (`src/`)

- **lib.rs**: Entry point with prelude module for convenient imports
- **app.rs**: Main application logic and lifecycle management
- **state.rs**: State management with messages for updates
- **components/**: Reusable UI components (header, footer, list, counter)
- **screens/**: Full-screen views (home, dashboard, interactive, settings)

### Features

- **default**: Standard terminal mode with ratatui
- **web**: WASM compilation with Yew for browser support
- **examples**: Additional example features

### Examples

1. **basic**: Minimal application demonstrating core functionality
2. **dashboard**: Multi-widget layout with components
3. **interactive**: Event handling and state updates
4. **web_demo**: WASM-compiled browser version

## Building

### Native Terminal App
```bash
cargo build --release
cargo run --example basic
```

### WASM Web App
```bash
cargo build --target wasm32-unknown-unknown --release --features web
```

### Using Nushell Script
```bash
./scripts/build.nu all      # Build everything
./scripts/build.nu wasm     # Build WASM only
./scripts/build.nu examples # Build examples only
```

## Dependencies

### Core
- **webatui**: Main framework (from raibid-labs)
- **ratatui**: Terminal UI library
- **crossterm**: Terminal handling

### Web
- **yew**: Web framework for Rust/WASM
- **wasm-bindgen**: JavaScript interop
- **web-sys**: Web API bindings

### Utilities
- **serde**: Serialization
- **tokio**: Async runtime
- **anyhow**: Error handling

## Next Steps

1. Implement component rendering logic
2. Add event handling
3. Create complete examples
4. Add integration tests
5. Write comprehensive documentation
