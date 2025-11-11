# WebaTUI Quick Start Guide

## 5-Minute Setup

### 1. Prerequisites Check

```bash
# Check Rust version (need 1.75+)
rustc --version

# Check if WASM target is installed
rustup target list | grep wasm32-unknown-unknown

# Check if Just is installed
just --version
```

### 2. Install Tools

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install WASM tools
cargo install wasm-pack wasm-bindgen-cli wasm-opt

# Install development tools
cargo install cargo-watch

# Optional: Install Just
cargo install just
```

### 3. Clone and Build

```bash
# Clone repository
git clone https://github.com/raibid-labs/webatui-ref.git
cd webatui-ref

# Build for WASM
just build-wasm

# Or manually:
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/webatui_ref.wasm \
    --target web --out-dir ./dist --no-typescript
```

### 4. Run Examples

#### Option A: Native Terminal

```bash
# Run main application
cargo run

# Run specific example
cargo run --example basic
cargo run --example dashboard
```

#### Option B: Web Browser

```bash
# Serve WASM application
just serve

# Or manually with Python
python3 -m http.server --directory ./dist 8080

# Open browser to http://localhost:8080
```

## Common Tasks

### Development Workflow

```bash
# Start development with hot reload
just watch              # Native
just watch-wasm         # WASM

# Format, lint, and test
just dev

# Run all checks (CI)
just ci
```

### Building

```bash
# Debug builds (faster)
cargo build                                      # Native
cargo build --target wasm32-unknown-unknown     # WASM

# Release builds (optimized)
just build-release          # Native
just build-wasm-release     # WASM
```

### Testing

```bash
# Run all tests
just test

# Run specific test
cargo test test_name

# Run with coverage
just test-coverage

# Run benchmarks
just bench
```

### Documentation

```bash
# Build and open docs
just docs-open

# Or manually
cargo doc --open --no-deps
```

## Project Structure Quick Reference

```
webatui-ref/
├── src/
│   ├── lib.rs           # Library entry point
│   ├── app/             # Application core
│   ├── components/      # UI widgets
│   ├── screens/         # Application screens
│   ├── state/           # State management
│   └── utils/           # Utilities
├── examples/            # Example applications
├── docs/                # Documentation
├── scripts/             # Build scripts
└── justfile            # Build automation
```

## Creating Your First App

### Step 1: Basic Structure

```rust
// examples/my_app.rs
use ratatui::{
    backend::CrosstermBackend,
    layout::Rect,
    widgets::Paragraph,
    Terminal,
};
use webatui::{TerminalApp, Event, Frame};

struct MyApp {
    message: String,
}

impl TerminalApp for MyApp {
    fn update(&mut self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => {
                    // Signal exit
                }
                _ => {
                    self.message = format!("Pressed: {:?}", key.code);
                }
            }
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
    let app = MyApp {
        message: "Press any key...".into(),
    };
    webatui::run(app)
}
```

### Step 2: Add to Cargo.toml

```toml
[[example]]
name = "my_app"
path = "examples/my_app.rs"

[dependencies]
ratatui = "0.27"
webatui = "0.1"
```

### Step 3: Build and Run

```bash
# Native
cargo run --example my_app

# WASM
cargo build --example my_app --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/debug/my_app.wasm \
    --target web --out-dir ./examples/my_app/dist
```

## Component Usage Examples

### Using a Chart

```rust
use webatui::components::ChartWidget;

let data = vec![(0.0, 10.0), (1.0, 20.0), (2.0, 15.0)];
let chart = ChartWidget::line()
    .data(data)
    .title("My Chart")
    .x_label("Time")
    .y_label("Value");

chart.render(frame, area);
```

### Using a Table

```rust
use webatui::components::TableWidget;

let columns = vec![
    Column::new("ID").width(10),
    Column::new("Name").width(20),
    Column::new("Status").width(10),
];

let table = TableWidget::new(columns)
    .rows(data)
    .selected(0);

table.render(frame, area);
```

### Using a Menu

```rust
use webatui::components::MenuWidget;

let menu = MenuWidget::horizontal()
    .add_item("Dashboard", Action::Navigate(Screen::Dashboard))
    .add_item("Settings", Action::Navigate(Screen::Settings))
    .add_item("Help", Action::Navigate(Screen::Help));

menu.render(frame, area);
```

## State Management Pattern

```rust
use webatui::state::AppState;

// Define your state
struct MyState {
    counter: usize,
    message: String,
}

// Implement state updates
impl MyState {
    fn increment(&mut self) {
        self.counter += 1;
        self.message = format!("Count: {}", self.counter);
    }

    fn reset(&mut self) {
        self.counter = 0;
        self.message = "Reset!".into();
    }
}

// Use in your app
impl TerminalApp for MyApp {
    fn update(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Key(KeyCode::Char(' ')) => {
                self.state.increment();
            }
            Event::Key(KeyCode::Char('r')) => {
                self.state.reset();
            }
            _ => {}
        }
        Ok(())
    }
}
```

## Configuration

### Theme Configuration

```rust
use webatui::theme::{Theme, ColorPalette};

let theme = Theme {
    name: "My Theme".into(),
    colors: ColorPalette {
        primary: Color::Cyan,
        secondary: Color::Blue,
        background: Color::Black,
        foreground: Color::White,
        // ...
    },
};

app.set_theme(theme);
```

### Persistence

```rust
use webatui::storage::Storage;

// Save state
let storage = Storage::new();
storage.save("my_key", &serde_json::to_string(&state)?)?;

// Load state
let json = storage.load("my_key")?;
let state: MyState = serde_json::from_str(&json)?;
```

## Troubleshooting

### WASM Build Fails

```bash
# Ensure target is installed
rustup target add wasm32-unknown-unknown

# Check wasm-bindgen version matches
cargo install wasm-bindgen-cli --version 0.2.92

# Clean and rebuild
cargo clean
just build-wasm
```

### "Cannot find module" in Browser

```bash
# Check all files are in dist/
ls dist/

# Ensure index.html loads correct paths
cat dist/index.html | grep ".wasm"
cat dist/index.html | grep ".js"

# Try serving from dist directory
cd dist && python3 -m http.server 8080
```

### Performance Issues

```bash
# Build in release mode
cargo build --target wasm32-unknown-unknown --release

# Optimize WASM
wasm-opt -Oz -o output.wasm input.wasm

# Check bundle size
ls -lh dist/*.wasm

# Enable release optimizations in Cargo.toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
```

### State Not Persisting (WASM)

```javascript
// Check browser console for errors
// Ensure LocalStorage is available
console.log(localStorage);

// Check storage quota
navigator.storage.estimate().then(estimate => {
    console.log(`Quota: ${estimate.quota}`);
    console.log(`Usage: ${estimate.usage}`);
});
```

## Next Steps

1. **Read Architecture Docs**: [`docs/architecture.md`](architecture.md)
2. **Explore Examples**: Check `examples/` directory
3. **Component Reference**: [`docs/design/component-specs.md`](design/component-specs.md)
4. **State Management**: [`docs/design/state-management.md`](design/state-management.md)
5. **Join Community**: GitHub Discussions

## Useful Commands Reference

```bash
# Building
just build              # Native debug
just build-release      # Native release
just build-wasm         # WASM debug
just build-wasm-release # WASM release

# Running
just run                # Run main app
just run-example NAME   # Run specific example
just serve              # Serve WASM

# Development
just watch              # Watch native
just watch-wasm         # Watch WASM
just dev                # Dev workflow
just ci                 # CI checks

# Testing
just test               # Run tests
just test-coverage      # With coverage
just bench              # Benchmarks

# Documentation
just docs               # Build docs
just docs-open          # Build and open

# Utilities
just clean              # Clean artifacts
just fmt                # Format code
just lint               # Run clippy
just check              # Check compilation
```

## Resources

- [Full Documentation](../README.md)
- [Architecture Overview](architecture.md)
- [Component Specs](design/component-specs.md)
- [State Management](design/state-management.md)
- [Ratatui Docs](https://ratatui.rs/)
- [WASM Book](https://rustwasm.github.io/book/)

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/raibid-labs/webatui-ref/issues)
- **Discussions**: [GitHub Discussions](https://github.com/raibid-labs/webatui-ref/discussions)
- **Examples**: Check `examples/` directory
- **Documentation**: `just docs-open`

---

**Happy Building! 🦀🌐**
