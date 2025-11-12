# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a reference implementation demonstrating **AI-controlled 3D game development in the terminal**. It integrates:

- **Bevy 0.16+** (ECS game engine)
- **BRP** (Bevy Remote Protocol for live entity manipulation)
- **bevy_ratatui_camera** (3D scene rendering to terminal)
- **MCP** (Model Context Protocol for AI interaction)

**Core Innovation**: AI prompts directly control and visualize 3D Bevy scenes rendered as ASCII/Unicode in the terminal, enabling headless development with visual feedback.

## Development Commands

### Building

```bash
# Check all features compile
cargo check --all-features

# Build with TUI rendering only
cargo build --features tui

# Build with BRP (Bevy Remote Protocol) only
cargo build --features brp

# Build with both TUI and BRP (full integration)
cargo build --features full

# Release build
cargo build --release --features full
```

### Running Examples

**Important: How TUI Rendering Works**

`bevy_ratatui_camera` requires the full 3D rendering pipeline to work. It captures the rendered 3D scene and converts it to ASCII. This means:

- **A window is created** (provides the 3D rendering infrastructure)
- **Terminal shows ASCII** (converted from the 3D render)
- **Both outputs run simultaneously**

This is "3D-to-ASCII conversion", not a pure terminal-only TUI app like nvim.

```bash
# Basic TUI rendering (window + terminal ASCII output)
cargo run --example tui_basic --features tui

# With BRP for AI control
cargo run --example tui_brp --features full

# Enhanced dual mode with complex scene
cargo run --example windowed_tui --features full
```

**Exit**: Press `Ctrl+C` or close the window.

### Testing

```bash
# Run all tests
cargo test --all-features

# Run specific test module
cargo test --test integration_tests --features full

# Run TUI-specific tests
cargo test --test tui_tests --features tui

# Run with output visible
cargo test -- --nocapture
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Lint with clippy
cargo clippy --all-features

# Strict clippy for production
cargo clippy --all-features -- -D warnings
```

## Architecture

### 5-Layer System Design

```
┌─────────────────┐
│   AI (Claude)   │  Natural language → MCP tool calls
└────────┬────────┘
         │
┌────────▼────────┐
│   MCP Bridge    │  Validates, translates to BRP JSON-RPC
└────────┬────────┘
         │
┌────────▼────────┐
│  Bevy Engine    │  ECS manages 3D scene, components, systems
└────────┬────────┘
         │
┌────────▼────────┐
│ TUI Rendering   │  bevy_ratatui_camera → Unicode/ASCII conversion
└────────┬────────┘
         │
┌────────▼────────┐
│    Terminal     │  24-bit color ANSI output
└─────────────────┘
```

### Project Structure (Planned)

```
src/
├── lib.rs                  # Library entry point, public API
├── main.rs                 # Binary entry point (Bevy app)
├── tui/                    # TUI rendering module
│   ├── mod.rs              # Public TUI interface
│   ├── plugin.rs           # BevyMcpTuiPlugin
│   ├── config.rs           # TUI configuration (render modes, dimensions)
│   ├── rendering.rs        # Rendering strategy management
│   └── widget.rs           # Custom ratatui widgets
├── brp/                    # BRP integration module
│   ├── mod.rs              # BRP module interface
│   └── tools.rs            # Custom MCP tools for TUI control
└── systems/                # Game systems
    ├── mod.rs              # Systems module
    └── demo.rs             # Demo scene systems
```

### Key Components

**TUI Rendering Pipeline**:
1. Bevy renders 3D scene to texture
2. `bevy_ratatui_camera` samples pixels
3. Pixel data converted to Unicode characters based on strategy:
   - **ASCII**: Character density mapping (`@%#*+=-:.`)
   - **Color**: RGB with Unicode blocks (`█▓▒░`)
   - **Edge**: Wireframe with box drawing (`┌─┐│└┘`)
4. Ratatui renders to terminal via crossterm

**MCP Tools** (for AI control):
- `bevy_spawn` - Create entities (cube, sphere, plane)
- `bevy_mutate_component` - Modify transforms, materials, components
- `bevy_query` - Query entities in scene
- `bevy_destroy` - Remove entities
- `bevy_camera_control` - Control TUI camera position and render mode

**BRP Integration**:
- Listens on `localhost:15702` (default)
- Uses `bevy_brp_extras` for full component mutation support
- All entities tagged with `Name` component for AI-friendly identification

## Implementation Status

**Current Phase**: Documentation Complete ✅

- ✅ Research & feasibility analysis (docs/research.md)
- ✅ System architecture design (docs/architecture.md)
- ✅ 5-phase implementation roadmap (docs/implementation-plan.md)
- ✅ Usage examples & AI prompts (docs/usage-examples.md)
- ⏳ **Next**: Phase 1 - Foundation setup (2-3 days)

**Implementation Plan**: See `docs/implementation-plan.md` for detailed 5-phase roadmap (16-21 days total).

## Critical Technical Decisions

### Feature Flags Design

```toml
[features]
default = []
brp = ["bevy/bevy_remote", "bevy_brp_extras"]
tui = ["bevy_ratatui_camera", "bevy_ratatui", "ratatui", "crossterm"]
full = ["brp", "tui"]
```

- **Rationale**: Users may want TUI rendering without BRP, or BRP without TUI
- **Default**: No features enabled (minimal Bevy app with window)
- **Development**: Use `--features full` for complete integration

### Plugin Usage Patterns

**Standard Usage (Window + Terminal ASCII)**:
```rust
use bevy::prelude::*;
use bevy_mcp_ratatui_ref::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)  // Provides 3D rendering pipeline
        .add_plugins(BevyMcpTuiPlugin::default())  // Converts to ASCII
        .run();
}
```

**Why DefaultPlugins is Required**:
- `bevy_ratatui_camera` needs the full rendering pipeline (meshes, materials, lighting)
- It captures the rendered 3D frame and converts pixels to ASCII characters
- Without `DefaultPlugins`, resources like `Assets<Mesh>` don't exist
- True headless rendering would require custom minimal plugin configuration

**Using RatatuiPlugins Directly**:
`RatatuiPlugins` is for pure TUI apps (text-based UIs like nvim), not 3D-to-ASCII conversion. It doesn't include the rendering infrastructure needed for this project.

### Rendering Strategy Selection

The TUI renderer must select appropriate strategies based on:
- Terminal capabilities (24-bit color support)
- Performance requirements (ASCII faster than color)
- Visual fidelity needs (edge detection for wireframes)

**Auto-detection logic** (to be implemented):
```rust
fn detect_terminal_capabilities() -> RenderingStrategy {
    if supports_24bit_color() && performance_budget_high() {
        RenderingStrategy::Color
    } else if supports_unicode() {
        RenderingStrategy::Unicode
    } else {
        RenderingStrategy::Ascii
    }
}
```

### Camera Synchronization (Dual Mode)

When running windowed + TUI simultaneously:
- Single camera entity drives both renderers
- `RatatuiCamera` component tags which cameras render to TUI
- Transform changes propagate to both rendering pipelines
- Performance: TUI rendering happens post-process, doesn't block main render

## AI Interaction Patterns

### Typical AI Workflow

1. **AI receives prompt**: "Create a spinning cube in the terminal"
2. **AI calls MCP tool**: `bevy_spawn` with cube mesh + rotation component
3. **MCP bridge translates**: JSON-RPC request to BRP
4. **Bevy ECS executes**: Spawns entity with components
5. **TUI renderer updates**: Next frame shows cube in terminal
6. **AI sees output**: Terminal rendering provides visual feedback for iteration

### Entity Naming Convention

All spawned entities MUST have descriptive `Name` components:
```rust
commands.spawn((
    // ... mesh, transform, material
    Name::new("Player Character"),  // AI-friendly identifier
));
```

This enables AI to:
- Query by semantic name ("find the player")
- Target mutations ("move the red cube left")
- Understand scene composition

## Performance Targets

| Metric | Target | Rationale |
|--------|--------|-----------|
| Frame Rate | 30-60 FPS | Smooth terminal updates |
| BRP Latency | <10ms | Responsive AI control |
| Terminal Redraw | <10ms | No visible lag |
| Memory | <100MB | Headless-friendly |
| Startup Time | <2s | Quick iteration |
| Entity Limit | 1000+ | Complex scenes |

**Optimization priorities**:
1. Frame time (most visible to users)
2. BRP latency (AI responsiveness)
3. Memory (headless deployment)

## Testing Strategy

### Unit Tests
- Component logic and data structures
- Rendering strategy conversions (pixel → character)
- MCP tool parameter validation

### Integration Tests
- BRP ↔ Bevy entity synchronization
- TUI rendering pipeline (3D → terminal)
- Camera system behavior

### End-to-End Tests
- Full AI prompt → terminal rendering flow
- Multi-entity scene management
- Error recovery and fallback rendering

**Test invocation**:
```bash
# Unit tests (fast)
cargo test --lib --features full

# Integration tests (moderate)
cargo test --test integration_tests --features full

# E2E tests (slow, requires terminal)
cargo test --test e2e_tests --features full -- --test-threads=1
```

## Common Development Tasks

### Adding a New Rendering Strategy

1. Implement `RenderStrategy` trait in `src/tui/rendering.rs`
2. Register in `RenderStrategyRegistry`
3. Add feature flag if external dependency required
4. Document in `docs/usage-examples.md`
5. Add visual regression test

### Adding a New MCP Tool

1. Define tool schema in `src/brp/tools.rs`
2. Implement BRP translation logic
3. Add validation and error handling
4. Document in `docs/usage-examples.md` with AI prompt example
5. Add integration test

### Debugging TUI Rendering Issues

**Common issues**:
- **Garbled output**: Check terminal size detection
- **Missing entities**: Verify frustum culling and depth sorting
- **Performance degradation**: Profile with `--features bevy/trace`

**Debug tools**:
```bash
# Enable Bevy tracing
cargo run --features full,bevy/trace

# Terminal size debugging
echo $COLUMNS x $LINES

# Test terminal color support
curl -s https://gist.githubusercontent.com/lifepillar/09a44b8cf0f9397465614e622979107f/raw/24-bit-color.sh | bash
```

## Documentation Structure

- **README.md** - Quick start, features, high-level overview
- **docs/research.md** (2,028 lines) - Feasibility study, integration analysis, performance benchmarks
- **docs/architecture.md** (1,730 lines) - Complete system design, data flow, mermaid diagrams
- **docs/ARCHITECTURE_SUMMARY.md** - Quick reference for architecture
- **docs/implementation-plan.md** - 5-phase roadmap with detailed tasks
- **docs/usage-examples.md** - 20+ AI prompt examples, MCP usage, troubleshooting

**When modifying architecture**: Update both `architecture.md` (detailed) and `ARCHITECTURE_SUMMARY.md` (quick ref).

## Reference Implementation Philosophy

This project is a **reference**, not a production framework. Priorities:

1. **Clarity over abstraction** - Explicit code, minimal magic
2. **Documentation over features** - Every pattern explained
3. **Examples over APIs** - Show, don't just tell
4. **Simplicity over completeness** - Core use cases only

**Anti-patterns to avoid**:
- Over-engineering plugin systems
- Premature optimization
- Feature creep beyond AI → TUI → Bevy integration
- Abstracting away Bevy/Ratatui patterns

## Related Projects

- **bevy-mcp-ref** - Foundation project (BRP + MCP without TUI)
  - Located at: `/Users/beengud/raibid-labs/bevy-mcp-ref`
  - Reference for BRP integration patterns
- **bevy_ratatui_camera** - TUI rendering library
  - GitHub: https://github.com/cxreiff/bevy_ratatui_camera
  - Handles 3D → Unicode conversion

## Terminal Compatibility

**Recommended terminals** (24-bit color + Unicode):
- Alacritty, Kitty, iTerm2, WezTerm, Rio, Ghostty

**Testing matrix**:
- macOS: iTerm2, Terminal.app
- Linux: Alacritty, gnome-terminal
- Windows: Windows Terminal, ConEmu

**Minimum requirements**:
- Unicode support (UTF-8)
- 80x24 character display
- ANSI escape sequences

**Optimal setup**:
- 24-bit true color
- 120x40+ character display
- GPU-accelerated rendering
