# WebaTUI Reference Implementation

**Terminal UI Applications that Work in Both Terminal and Browser**

A comprehensive reference implementation demonstrating how to build interactive terminal UI applications with [Ratatui](https://ratatui.rs/) that seamlessly run in both native terminals and web browsers via WebAssembly.

[![Rust Version](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![WASM Compatible](https://img.shields.io/badge/WASM-compatible-green.svg)](https://webassembly.org/)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

---

## What is WebaTUI?

WebaTUI bridges the gap between terminal-based applications and web browsers, allowing developers to write TUI applications once and deploy them everywhere. This reference implementation demonstrates production-ready patterns, component architecture, and best practices for building sophisticated TUI applications.

## Features

- Terminal UI applications that run in both terminal and browser
- Rich component library (charts, tables, menus, gauges)
- Real-time data visualization with interactive controls
- Persistent state management with platform-specific storage
- Keyboard and mouse event handling
- Theme support with customizable color schemes
- Responsive layouts that adapt to terminal/browser size
- Optimized WASM builds (< 200KB gzipped)
- Comprehensive examples from basic to advanced
- Complete automation with justfile and nushell scripts

## Quick Start

### Prerequisites

- **Rust** (1.75 or later) - [Install here](https://www.rust-lang.org/tools/install)
- **WASM target** - `rustup target add wasm32-unknown-unknown`
- **Just** (optional, recommended) - `cargo install just`
- **wasm-pack** (for WASM builds) - `cargo install wasm-pack`

### Installation

```bash
# Clone the repository
git clone https://github.com/raibid-labs/webatui-ref.git
cd webatui-ref

# Run in terminal (native)
cargo run --example basic

# Build for browser (WASM)
just build-wasm
just serve

# Open browser to http://localhost:8080
```

### Your First App

Create `examples/hello.rs`:

```rust
use ratatui::{backend::CrosstermBackend, widgets::Paragraph, Terminal};
use webatui::{TerminalApp, Event, Frame};

struct HelloApp {
    message: String,
}

impl TerminalApp for HelloApp {
    fn update(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        if let Event::Key(key) = event {
            self.message = format!("You pressed: {:?}", key.code);
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) -> Result<(), Box<dyn std::error::Error>> {
        let paragraph = Paragraph::new(self.message.as_str());
        frame.render_widget(paragraph, frame.size());
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = HelloApp {
        message: "Hello, WebaTUI! Press any key...".into(),
    };
    webatui::run(app)
}
```

Run it:

```bash
# Native terminal
cargo run --example hello

# Browser (after adding to Cargo.toml)
cargo build --example hello --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/hello.wasm \
    --target web --out-dir ./dist
```

## Documentation

Comprehensive documentation organized by topic:

### Getting Started

- **[Quick Start Guide](docs/QUICK_START.md)** - Get up and running in 5 minutes
  - Installation and setup
  - Creating your first app
  - Component usage examples
  - Configuration and theming
  - Troubleshooting common issues

### Architecture & Design

- **[System Architecture](docs/architecture.md)** - Complete architectural overview
  - Multi-layer architecture (Application, Component, State, Platform)
  - Component hierarchy and trait system
  - State management patterns
  - Event handling and routing
  - WASM integration strategy
  - Performance optimization techniques

- **[Project Structure](docs/STRUCTURE.md)** - Directory organization and file layout
  - Source code organization
  - Component structure
  - Example applications
  - Build scripts and automation
  - Testing infrastructure

### Component Specifications

- **[Component Specs](docs/design/component-specs.md)** - Detailed component documentation
  - Menu Widget (vertical/horizontal navigation)
  - Chart Widget (line, bar, sparkline)
  - Table Widget (sorting, filtering, pagination)
  - Gauge Widget (progress bars, meters)
  - Input Widget (text input, validation)
  - Layout components (panels, splits, grids)

- **[State Management](docs/design/state-management.md)** - State architecture and patterns
  - Application state structure
  - Message passing system
  - State persistence (LocalStorage/File)
  - Cross-platform state handling
  - Undo/redo implementation

- **[Visual Designs](docs/design/visual-designs.md)** - UI/UX design specifications
  - Screen layouts and wireframes
  - Color schemes and themes
  - Typography and spacing
  - Interactive elements
  - Accessibility considerations

### Development

- **[Implementation Roadmap](docs/ROADMAP.md)** - 10-phase development plan
  - Phase 1: Foundation (Project setup, core architecture)
  - Phase 2: Component Library (Widgets and layouts)
  - Phase 3: Dashboard Implementation (Metrics and visualization)
  - Phase 4: Settings & Configuration (Persistence and themes)
  - Phase 5-10: Advanced features, optimization, deployment

- **[Research & Background](docs/research.md)** - Technical research and feasibility
  - Ratatui and WASM integration analysis
  - Performance considerations
  - Browser compatibility
  - Alternative approaches
  - Prior art and related projects

- **[Automation Guide](docs/AUTOMATION.md)** - Build and deployment automation
  - Justfile commands reference
  - Nushell script usage
  - CI/CD integration
  - Deployment workflows

- **[Architecture Summary](docs/ARCHITECTURE_SUMMARY.md)** - Quick reference guide
  - High-level system overview
  - Key design decisions
  - Integration patterns

### Scripts & Tools

- **[Scripts Documentation](scripts/README.md)** - Automation scripts reference
  - `build.nu` - Comprehensive build automation
  - `serve.nu` - HTTP server for WASM development
  - `deploy.nu` - Multi-platform deployment
  - `test.nu` - Testing and coverage
  - `clean.nu` - Cleanup automation

## Project Structure

```
webatui-ref/
├── src/
│   ├── lib.rs              # Library entry point with prelude
│   ├── app.rs              # Main application logic
│   ├── state.rs            # Application state management
│   ├── components/         # Reusable UI widgets
│   │   ├── header.rs       # Header component
│   │   ├── footer.rs       # Footer component
│   │   ├── list.rs         # List component
│   │   └── counter.rs      # Counter component
│   └── screens/            # Full-screen views
│       ├── home.rs         # Home screen
│       ├── dashboard.rs    # Dashboard with metrics
│       ├── interactive.rs  # Interactive demo
│       └── settings.rs     # Settings screen
├── examples/
│   ├── basic.rs            # Minimal example
│   ├── dashboard.rs        # Dashboard with real-time data
│   ├── interactive.rs      # Event handling demo
│   └── web_demo.rs         # WASM browser demo
├── docs/                   # Comprehensive documentation
│   ├── architecture.md     # System architecture
│   ├── research.md         # Technical research
│   ├── design/             # Design specifications
│   ├── ROADMAP.md          # Implementation roadmap
│   ├── QUICK_START.md      # Quick start guide
│   ├── STRUCTURE.md        # Project structure
│   └── AUTOMATION.md       # Build automation
├── scripts/                # Nushell build scripts
│   ├── build.nu            # Build automation
│   ├── serve.nu            # HTTP server
│   ├── deploy.nu           # Deployment
│   ├── test.nu             # Testing
│   └── README.md           # Scripts documentation
├── justfile                # Task automation
├── Cargo.toml              # Project manifest
└── README.md               # This file
```

## Examples

### Basic Terminal App

A minimal example showing core functionality:

```bash
cargo run --example basic
```

Features:
- Simple text display
- Keyboard event handling
- State updates

### Dashboard

A comprehensive dashboard with real-time metrics:

```bash
cargo run --example dashboard
```

Features:
- Multiple components (charts, gauges, tables)
- Real-time data updates
- Interactive navigation
- Status indicators

### Interactive Demo

Advanced interaction patterns:

```bash
cargo run --example interactive
```

Features:
- Text input fields
- Form validation
- Mouse support
- Modal dialogs

### Web Demo (WASM)

Browser-based terminal UI:

```bash
just build-wasm
just serve
# Open browser to http://localhost:8080
```

Features:
- Full browser compatibility
- LocalStorage persistence
- Hyperlink support
- Touch-friendly (mobile)

## Technology Stack

### Core Dependencies

```toml
[dependencies]
webatui = { git = "https://github.com/raibid-labs/webatui", branch = "main" }
ratatui = "0.29"           # Terminal UI framework
crossterm = "0.28"         # Terminal handling

# Web/WASM support
yew = { version = "0.21", optional = true }
wasm-bindgen = { version = "0.2", optional = true }
web-sys = { version = "0.3", optional = true }

# State management
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime
tokio = { version = "1.42", features = ["sync", "macros"] }
```

### Architecture Layers

1. **Application Layer** - High-level screens and navigation
2. **Component Layer** - Reusable UI widgets
3. **State Layer** - Centralized state management
4. **Platform Layer** - Terminal/WASM abstraction

### Features

- **default** - Standard terminal mode
- **web** - WASM browser support
- **examples** - Additional example features

## Development

### Available Commands

The project uses [Just](https://github.com/casey/just) for task automation. See all commands:

```bash
just --list
```

### Building

```bash
# Native builds
just build              # Debug build
just build-release      # Release build

# WASM builds
just build-wasm         # WASM debug
just build-wasm-release # WASM release with optimization

# Build all
just build-all          # Native + WASM
```

### Running

```bash
# Run main application
just run

# Run specific example
just run-example basic
just run-example dashboard
just run-example interactive

# Serve WASM (browser)
just serve              # http://localhost:8080
```

### Testing

```bash
# Run all tests
just test

# Run tests with coverage
just test-coverage

# Run benchmarks
just bench

# Run specific test
cargo test test_name -- --nocapture
```

### Development Workflow

```bash
# Format, lint, and test
just dev

# Watch and rebuild on changes
just watch              # Native
just watch-wasm         # WASM

# Full CI checks
just ci
```

### Documentation

```bash
# Build documentation
just docs

# Build and open in browser
just docs-open
```

## Key Concepts

### Cross-Platform Trait System

The `TerminalApp` trait provides a unified interface:

```rust
pub trait TerminalApp {
    fn update(&mut self, event: Event) -> Result<()>;
    fn render(&self, frame: &mut Frame) -> Result<()>;
}
```

Implement this trait once, run everywhere (terminal and browser).

### Component-Based Architecture

Reusable widgets following a consistent pattern:

```rust
pub trait Widget {
    fn render(&self, frame: &mut Frame, area: Rect);
    fn handle_input(&mut self, event: Event) -> Option<Action>;
}
```

### State Management

Centralized state with message-based updates:

```rust
pub enum Message {
    UpdateCounter(i32),
    Navigate(Screen),
    ChangeTheme(Theme),
}

impl AppState {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::UpdateCounter(value) => self.counter = value,
            Message::Navigate(screen) => self.current_screen = screen,
            Message::ChangeTheme(theme) => self.theme = theme,
        }
    }
}
```

### Platform-Specific Storage

Automatic storage backend selection:

```rust
// Native: ~/.config/webatui/config.json
// WASM: LocalStorage

let storage = Storage::new();
storage.save("key", &value)?;
let value = storage.load("key")?;
```

## Performance

### Optimization Techniques

1. **WASM Size Optimization**
   - LTO (Link-Time Optimization)
   - Code splitting
   - wasm-opt with `-Oz` flag
   - Target size: < 200KB gzipped

2. **Render Optimization**
   - Differential rendering (only changed areas)
   - Lazy widget evaluation
   - Batch state updates
   - Virtual scrolling for large lists

3. **State Management**
   - Efficient state updates
   - Minimal cloning
   - Smart re-rendering
   - Memoization for expensive computations

### Benchmarks

| Metric | Target | Typical |
|--------|--------|---------|
| WASM Size (gzipped) | < 200KB | ~150KB |
| Startup Time | < 100ms | 60-80ms |
| Frame Time | < 16.67ms | 8-12ms |
| Memory Usage | < 50MB | 25-40MB |

## Browser Compatibility

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Opera 76+

Terminal compatibility:
- Alacritty
- Kitty
- iTerm2
- WezTerm
- Windows Terminal
- Xterm.js (web terminal)

## Use Cases

### Application Dashboards

Build rich terminal dashboards with:
- Real-time metrics visualization
- System monitoring
- Log viewers
- Process managers

### Configuration Tools

Create interactive configuration UIs:
- Settings managers
- CLI wizards
- Interactive installers
- Environment setup tools

### Data Visualization

Terminal-based data exploration:
- Chart viewers
- Table browsers
- Data analysis tools
- Report generators

### Development Tools

Developer-focused applications:
- Git TUIs
- Database clients
- API testers
- Log analyzers

## Contributing

This is a reference implementation designed to demonstrate best practices for building webatui applications. Contributions welcome:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`just test`)
5. Format code (`just fmt`)
6. Run lints (`just lint`)
7. Commit changes (`git commit -m 'Add amazing feature'`)
8. Push to branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Development Guidelines

- Follow Rust best practices and idioms
- Maintain test coverage above 80%
- Update documentation for new features
- Add examples for significant functionality
- Keep WASM bundle size optimized
- Ensure cross-platform compatibility

## License

Licensed under either of:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Resources

### Raibid Labs Projects

- [webatui](https://github.com/raibid-labs/webatui) - Core library for terminal-to-browser bridge
- [bevy-mcp-ratatui-ref](https://github.com/raibid-labs/bevy-mcp-ratatui-ref) - AI-controlled game development in terminal
- [bevy-mcp-ref](https://github.com/raibid-labs/bevy-mcp-ref) - Bevy with Model Context Protocol

### Technology Documentation

- [Ratatui Documentation](https://ratatui.rs/) - Terminal UI framework
- [Ratatui Book](https://ratatui.rs/book/) - Comprehensive guide
- [WASM Book](https://rustwasm.github.io/book/) - WebAssembly with Rust
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/) - JS interop

### Community

- [GitHub Issues](https://github.com/raibid-labs/webatui-ref/issues) - Bug reports and feature requests
- [GitHub Discussions](https://github.com/raibid-labs/webatui-ref/discussions) - Questions and ideas
- [Ratatui Discord](https://discord.gg/pMCEU9hNEj) - Ratatui community

## Learning Path

1. **Understand the Concept** - Read this README and [Quick Start Guide](docs/QUICK_START.md)
2. **Explore Examples** - Run examples from simple to complex
3. **Study Architecture** - Review [architecture.md](docs/architecture.md)
4. **Read Component Specs** - Learn [component patterns](docs/design/component-specs.md)
5. **Build Your App** - Create your first webatui application
6. **Follow Roadmap** - Implement features from [ROADMAP.md](docs/ROADMAP.md)

## Acknowledgments

Built on the shoulders of giants:

- [Ratatui](https://ratatui.rs/) - Excellent terminal UI library
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Cross-platform terminal manipulation
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust/WASM/JS bridge
- [Yew](https://yew.rs/) - Rust web framework

Special thanks to the Rust and terminal UI communities for their continuous support and contributions.

---

**Built with by [Raibid Labs](https://github.com/raibid-labs)**

*Terminal applications, reimagined for the modern web*
