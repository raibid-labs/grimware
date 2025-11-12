# WebATUI Examples

This directory contains working example applications demonstrating webatui's capabilities.

## Available Examples

### 1. Basic Example (`basic.rs`)
**Minimal "Hello World" application**

A simple example showing the fundamental pattern for building webatui applications.

**Features:**
- Simple struct implementing the app pattern
- Basic keyboard event handling
- State management with a counter
- Clean terminal setup and teardown

**Running:**
```bash
cargo run --example basic
```

**Controls:**
- `q` or `Esc` - Quit
- `+` or `Up` - Increment counter
- `-` or `Down` - Decrement counter

---

### 2. Dashboard Example (`dashboard.rs`)
**Multi-widget dashboard with full component composition**

Demonstrates building complex UIs by combining multiple components with tab navigation.

**Features:**
- Multiple widget areas (Overview, Metrics, Charts, Logs)
- Tab navigation between different views
- Bar charts with animated data
- Gauge widgets for metrics
- Sparkline trends
- Activity log display
- Real-time state updates

**Running:**
```bash
cargo run --example dashboard
```

**Controls:**
- `q` or `Esc` - Quit
- `Tab` or `→` - Next tab
- `Shift+Tab` or `←` - Previous tab
- `u` - Update counter for current tab
- `r` - Refresh chart data

---

### 3. Interactive Example (`interactive.rs`)
**Full interactive demo with buttons, lists, and navigation**

Shows webatui's interactive features including focus management and complex state updates.

**Features:**
- Button interactions with visual feedback
- List navigation with selection highlighting
- Focus management between multiple areas
- Dynamic list manipulation (add/remove items)
- Status message updates
- Menu system

**Running:**
```bash
cargo run --example interactive
```

**Controls:**
- `q` or `Esc` - Quit
- `Tab` - Switch focus between Counter, List, and Menu
- `Enter` - Perform action in focused area
- `↑`/`↓` - Navigate list (when list is focused)
- `+`/`-` - Adjust counter (when counter is focused)
- `a` - Add item to list (when list is focused)
- `d` - Delete selected item (when list is focused)
- `r` - Reset state

---

### 4. Web Demo Example (`web_demo.rs`)
**WASM-compiled browser version**

Demonstrates deploying webatui applications to the web using WASM.

**Features:**
- WASM compilation for browsers
- Yew-based web components
- Same state management as terminal version
- Responsive web design
- Tab navigation
- Interactive buttons

**Building for Web:**

```bash
# Install required tools
cargo install wasm-pack
cargo install trunk

# Option 1: Using wasm-pack
wasm-pack build --target web --features web

# Option 2: Using trunk (recommended for development)
trunk serve --features web
```

**Accessing:**
After running `trunk serve`, open your browser to `http://localhost:8080`

---

## Code Structure

Each example is self-contained and demonstrates specific webatui patterns:

### Basic Pattern
```rust
struct App {
    should_quit: bool,
    // ... state
}

impl App {
    fn new() -> Self { /* ... */ }
    fn handle_event(&mut self) -> Result<()> { /* ... */ }
    fn render(&self, frame: &mut Frame) { /* ... */ }
}

fn main() -> Result<()> {
    // Setup terminal
    // Create app
    // Main loop: render -> handle events
    // Cleanup terminal
}
```

### Key Concepts Demonstrated

1. **State Management**: All examples show proper state encapsulation
2. **Event Handling**: Keyboard input processing with crossterm
3. **Rendering**: Using ratatui widgets and layouts
4. **Component Composition**: Building complex UIs from simple components
5. **Focus Management**: Handling multiple interactive areas
6. **Cross-Platform**: Same patterns work in terminal and web

---

## File Size Reference

All examples are designed to be readable and educational:

- `basic.rs` - ~140 lines (minimal example)
- `dashboard.rs` - ~312 lines (full-featured dashboard)
- `interactive.rs` - ~385 lines (comprehensive interactive demo)
- `web_demo.rs` - ~237 lines (WASM web version)

---

## Tips for Building Your Own Applications

1. **Start Simple**: Begin with the basic example pattern
2. **Compose Components**: Break complex UIs into smaller, reusable components
3. **State First**: Design your state structure before implementing rendering
4. **Event Flow**: Think about the flow: Event → Update State → Re-render
5. **Test Terminal First**: Terminal version is easier to debug than WASM
6. **Use Layouts**: Leverage ratatui's constraint-based layouts

---

## Next Steps

After exploring these examples:

1. Modify an example to add your own features
2. Combine patterns from different examples
3. Build a custom component library
4. Deploy to web using the web demo as template
5. Check out the main webatui documentation for advanced features

---

## Requirements

**Terminal Examples:**
- Rust 1.75+
- Terminal with ANSI color support
- crossterm-compatible terminal (most modern terminals)

**Web Example:**
- Rust 1.75+
- wasm-pack or trunk
- Modern web browser with WASM support

---

## Troubleshooting

**Terminal issues:**
- If colors don't work, ensure your terminal supports ANSI colors
- If layout looks broken, check terminal size (minimum 80x24 recommended)

**Web build issues:**
- Run `cargo clean` before building for web
- Ensure all `--features web` flags are included
- Check browser console for WASM-specific errors

**Performance:**
- Terminal rendering is typically very fast
- If experiencing slowness, reduce the polling interval in `handle_event()`
- WASM builds benefit from `--release` flag

---

## License

These examples are part of the webatui-ref project.
Licensed under MIT OR Apache-2.0.
