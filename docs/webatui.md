# WebATUI Reference Implementation

## Overview

A terminal UI library demonstrating how to build applications with Ratatui that work in both native terminals and web browsers via WebAssembly. This reference implementation showcases unified state management and cross-platform UI patterns.

## Key Features

- **Dual Platform Support**: Native terminal + web browser
- **Unified State**: Single state model across platforms
- **Component System**: Reusable UI widgets
- **Well Tested**: Comprehensive test suite for native and WASM
- **Production Ready**: Full feature flags and build configurations

## Quick Start

```bash
cd webatui-ref

# Install development tools
just install-deps

# List available examples
just list-examples

# Run an example
just example basic
just example dashboard
just example interactive
```

## Architecture

### Library Design

**This is a library crate**, not a binary application. It provides:
- Core state management (`src/state.rs`)
- UI components (`src/components/`)
- Screen layouts (`src/screens/`)
- Platform-agnostic logic

### Feature Flags

```toml
[features]
default = []
terminal = ["ratatui", "crossterm"]  # Native terminal support
web = ["yew", "wasm-bindgen"]        # Web/WASM support
examples = []                         # Additional example deps
```

**Usage**:
```bash
# Native terminal
cargo build --features terminal

# Web (WASM)
cargo build --target wasm32-unknown-unknown --no-default-features

# With examples
cargo run --example basic --features terminal,examples
```

## Project Structure

```
webatui-ref/
├── src/
│   ├── lib.rs           # Library entry point
│   ├── state.rs         # Core state (platform-independent)
│   ├── components/      # Terminal UI components
│   │   ├── mod.rs
│   │   ├── header.rs
│   │   ├── footer.rs
│   │   └── dashboard.rs
│   └── screens/         # Terminal UI screens
│       ├── mod.rs
│       ├── home.rs
│       └── settings.rs
├── examples/
│   ├── basic.rs         # Simple terminal UI
│   ├── dashboard.rs     # Dashboard with metrics
│   └── interactive.rs   # Interactive UI demo
├── tests/
│   ├── integration_test.rs  # Native tests
│   └── wasm_tests.rs        # WASM-specific tests
├── docs/
│   ├── QUICK_START.md       # 5-minute guide
│   ├── DEVELOPMENT.md       # Development workflow
│   ├── WASM_TESTING_SETUP.md    # WASM configuration
│   ├── APPLE_SILICON_SETUP.md   # M1/M2/M3 Mac setup
│   ├── AUTOMATION.md        # CI/CD and automation
│   ├── STRUCTURE.md         # Project organization
│   ├── ROADMAP.md           # Future plans
│   ├── architecture.md      # Technical architecture
│   ├── ARCHITECTURE_SUMMARY.md  # Quick reference
│   ├── research.md          # Technical research
│   └── design/              # Design documents
│       ├── component-specs.md
│       ├── state-management.md
│       └── visual-designs.md
├── .bacon/              # Bacon file watcher config
├── justfile             # Task automation
└── README.md
```

## Examples

### Basic Example

Simple terminal UI demonstrating core functionality:

```bash
just example basic
# or
cargo run --example basic --features terminal
```

**Features**: Text display, keyboard events, basic state updates

### Dashboard Example

Comprehensive dashboard with real-time metrics:

```bash
just example dashboard
```

**Features**: Charts, gauges, tables, system monitoring, live updates

### Interactive Example

Advanced interaction patterns:

```bash
just example interactive
```

**Features**: Text input, navigation, form handling, modal dialogs

## Using as a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
webatui-ref = { git = "https://github.com/raibid-labs/webatui-ref", features = ["terminal"] }
```

**Example usage**:

```rust
use webatui_ref::prelude::*;

fn main() -> anyhow::Result<()> {
    // Initialize state
    let mut state = AppState::default();

    // Update state
    state.update(Message::Navigate(Screen::Dashboard));

    // Use state in your app
    assert_eq!(state.current_screen, Screen::Dashboard);

    Ok(())
}
```

## Development Workflow

### Watch Mode with Bacon

The project uses [bacon](https://dystroy.org/bacon/) for fast file watching (Apple Silicon compatible):

```bash
bacon                # Build with watch
bacon test           # Test with watch
bacon example-basic  # Run example with watch
bacon wasm-check     # Check WASM compilation
```

See `.bacon/bacon.toml` for all configured jobs.

### Using Just Commands

```bash
# See all commands
just --list

# Development
just watch              # Watch and rebuild
just watch-test         # Watch and run tests
just watch-example basic  # Watch and run example

# Testing
just test              # Run all tests
just wasm-build        # Test WASM compilation
just wasm-test-browser # Browser-based WASM tests

# Code quality
just fmt               # Format code
just lint              # Run clippy
just check-all         # Format + lint + test
```

### Traditional Cargo Commands

```bash
# Build
cargo build --features terminal
cargo build --release --features terminal

# Run examples
cargo run --example basic --features terminal
cargo run --example dashboard --features terminal,examples

# Test
cargo test --features terminal
cargo test --lib --features terminal

# WASM
cargo build --target wasm32-unknown-unknown --no-default-features
wasm-pack test --node -- --lib --no-default-features
```

## Platform Support

### Native Terminals

**Supported Platforms**:
- ✅ macOS (Intel & Apple Silicon)
- ✅ Linux
- ✅ Windows

**Recommended Terminal Emulators**:
- Alacritty - GPU-accelerated
- Kitty - Feature-rich
- iTerm2 - macOS native
- WezTerm - Cross-platform
- Windows Terminal - Windows 10/11

### Browsers (WASM)

**Supported Browsers**:
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+

**WASM Features**:
- Full UI rendering in browser
- Same state management as native
- WebSocket support for backends
- IndexedDB for persistence

## Technology Stack

### Core Dependencies

```toml
[dependencies]
# Terminal UI (native only)
ratatui = { version = "0.29", optional = true }
crossterm = { version = "0.28", optional = true }

# Web support (WASM only)
yew = { version = "0.21", optional = true }
wasm-bindgen = { version = "0.2", optional = true }

# State management (always)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Async runtime
tokio = { version = "1.42", features = ["sync", "macros"], default-features = false }
```

## Testing

### Native Tests

```bash
cargo test --features terminal
```

**Coverage**:
- 6 unit tests in `src/lib.rs`
- 7 integration tests in `tests/integration_test.rs`
- Component-specific tests
- State management tests

### WASM Tests

```bash
# Build for WASM (verify compilation)
cargo build --target wasm32-unknown-unknown --no-default-features

# Run WASM tests
wasm-pack test --node -- --lib --no-default-features

# Browser-based tests (requires ChromeDriver)
just wasm-test-browser
```

## Component System

### Built-in Components

**Header** (`components/header.rs`):
- Title display
- Navigation tabs
- Status indicators

**Footer** (`components/footer.rs`):
- Help text
- Keyboard shortcuts
- Status bar

**Dashboard** (`components/dashboard.rs`):
- Real-time metrics
- Charts and graphs
- System information

### Creating Custom Components

```rust
use ratatui::{
    prelude::*,
    widgets::*,
};

pub struct MyComponent {
    title: String,
}

impl MyComponent {
    pub fn new(title: String) -> Self {
        Self { title }
    }

    pub fn render(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(self.title.as_str())
            .borders(Borders::ALL);

        f.render_widget(block, area);
    }
}
```

## State Management

### AppState Design

```rust
pub struct AppState {
    pub current_screen: Screen,
    pub input_buffer: String,
    pub items: Vec<String>,
    // ... more fields
}

pub enum Message {
    Navigate(Screen),
    UpdateInput(String),
    AddItem(String),
    // ... more messages
}

impl AppState {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Navigate(screen) => {
                self.current_screen = screen;
            }
            // ... handle other messages
        }
    }
}
```

### Pattern: Unidirectional Data Flow

```
User Input → Message → State Update → UI Re-render
```

This pattern ensures:
- Predictable state changes
- Easy debugging
- Testable logic
- Cross-platform compatibility

## Apple Silicon (M1/M2/M3) Support

Full compatibility with Apple Silicon Macs:

- Native ARM64 compilation
- Optimized bacon file watcher
- No Rosetta required
- Fast compile times

See [docs/APPLE_SILICON_SETUP.md](../webatui-ref/docs/APPLE_SILICON_SETUP.md) for setup details.

## Performance Considerations

### Terminal Rendering
- Target 30-60 FPS for smooth UI
- Minimize unnecessary redraws
- Use double buffering (handled by ratatui)
- Batch state updates

### WASM Optimization
```toml
[profile.release]
opt-level = "z"       # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Better optimization
```

Result: ~200KB WASM bundle (gzipped)

## Best Practices

### Component Design
1. Keep components focused and small
2. Accept data via constructor, don't store state
3. Use `render()` method for drawing
4. Separate layout logic from rendering

### State Management
1. All state in `AppState`
2. Update via messages only
3. Keep state serializable
4. Document state transitions

### Testing
1. Unit test state updates
2. Integration test component rendering
3. Test both native and WASM targets
4. Use snapshot tests for UI

## Common Patterns

### Modal Dialog
```rust
if state.show_modal {
    let modal_area = centered_rect(60, 20, f.size());
    f.render_widget(Clear, modal_area);
    f.render_widget(modal_widget, modal_area);
}
```

### List Navigation
```rust
match key.code {
    KeyCode::Up => state.list_index = state.list_index.saturating_sub(1),
    KeyCode::Down => state.list_index = (state.list_index + 1).min(max_index),
    _ => {}
}
```

### Async Operations
```rust
let (tx, rx) = tokio::sync::mpsc::channel(100);

tokio::spawn(async move {
    let result = fetch_data().await;
    tx.send(Message::DataLoaded(result)).await.unwrap();
});
```

## Documentation Resources

- **[Quick Start](../webatui-ref/docs/QUICK_START.md)** - Get started in 5 minutes
- **[Development Guide](../webatui-ref/docs/DEVELOPMENT.md)** - Complete workflow
- **[WASM Setup](../webatui-ref/docs/WASM_TESTING_SETUP.md)** - WASM configuration
- **[Apple Silicon](../webatui-ref/docs/APPLE_SILICON_SETUP.md)** - M1/M2/M3 setup
- **[Architecture](../webatui-ref/docs/architecture.md)** - Technical design
- **[Component Specs](../webatui-ref/docs/design/component-specs.md)** - Component details
- **[State Management](../webatui-ref/docs/design/state-management.md)** - State patterns

## Further Reading

- [Ratatui Documentation](https://ratatui.rs/)
- [WASM Book](https://rustwasm.github.io/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Yew Documentation](https://yew.rs/)
- [Bacon Documentation](https://dystroy.org/bacon/)
