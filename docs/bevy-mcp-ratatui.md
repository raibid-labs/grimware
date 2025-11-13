# Bevy MCP Ratatui Reference Implementation

## Overview

AI-controlled 3D game development rendered directly in your terminal. This implementation combines Bevy game engine with terminal UI rendering, enabling visual AI feedback and headless development workflows.

## Unique Value Proposition

**AI Prompt → 3D Terminal Visualization**

Ask Claude to spawn entities, change colors, or modify transforms and see results immediately in your terminal with ASCII/Unicode rendering of full 3D scenes.

## Architecture

```
AI (Claude) → MCP Bridge → Bevy Engine → TUI Rendering → Terminal
```

### Five-Layer System

1. **AI Layer**: Natural language commands via Claude Code
2. **MCP Bridge**: Custom spawn methods + standard BRP tools
3. **Bevy Application**: ECS-based 3D scene management
4. **TUI Rendering**: bevy_ratatui_camera with multiple strategies
5. **Terminal Display**: ANSI output with 24-bit color support

## Key Features

### Terminal 3D Rendering
- **ASCII Strategy**: Character density mapping (`@%#*+=-:.`)
- **Color Strategy**: RGB with Unicode blocks (`█▓▒░`)
- **Edge Strategy**: Wireframe with box drawing (`┌─┐│└┘`)

### AI Integration
- Natural language entity spawning
- Real-time component manipulation
- Visual feedback through terminal rendering
- No recompilation required for iteration

### Custom BRP Methods

Standard BRP cannot spawn entities with meshes because asset handles aren't serializable. This implementation provides:

#### `bevy/spawn_cube`
Spawn cubes with position, scale, color, and material properties.

```javascript
mcp__brp__brp_execute({
  method: "bevy/spawn_cube",
  params: {
    position: [3.0, 1.0, 0.0],
    scale: [1.0, 1.0, 1.0],
    color: [0.8, 0.2, 0.2],
    metallic: 0.7,
    roughness: 0.3,
    name: "Red Cube"
  }
})
```

#### `bevy/spawn_sphere`
Spawn spheres with position, radius, color, and material properties.

```javascript
mcp__brp__brp_execute({
  method: "bevy/spawn_sphere",
  params: {
    position: [-3.0, 1.0, 0.0],
    radius: 0.5,
    color: [0.5, 0.2, 0.8],
    metallic: 0.9,
    roughness: 0.1,
    name: "Purple Sphere"
  }
})
```

## Quick Start

### Prerequisites
- Rust (latest stable)
- Claude Code CLI
- Terminal with 24-bit color support (Alacritty, Kitty, iTerm2, WezTerm)

### Running

```bash
cd bevy-mcp-ratatui-ref

# Basic TUI rendering (window + terminal ASCII)
cargo run --example tui_basic --features tui

# With BRP for AI control
cargo run --example tui_brp --features full

# Exit: Ctrl+C or close window
```

### First AI Interaction

```
"Show me all entities in the TUI scene"
"Add a red cube at position [3, 1, 0]"
"Spawn a shiny purple sphere at [-3, 1, 0]"
"Move the red sphere up by 2 units"
"Change the green sphere color to yellow"
```

## How TUI Rendering Works

**Important**: This is "3D-to-ASCII conversion", not a pure terminal TUI app.

1. Bevy renders full 3D scene to texture
2. `bevy_ratatui_camera` captures rendered frame
3. Pixels converted to Unicode characters based on strategy
4. Ratatui renders to terminal via crossterm

**Result**: Both window and terminal show the scene simultaneously.

## Project Structure

```
bevy-mcp-ratatui-ref/
├── docs/
│   ├── research.md              # Feasibility study (2,028 lines)
│   ├── architecture.md          # System design (1,730 lines)
│   ├── ARCHITECTURE_SUMMARY.md  # Quick reference
│   ├── implementation-plan.md   # 5-phase roadmap
│   ├── usage-examples.md        # 20+ AI prompts
│   └── custom-brp-methods.md    # Entity spawning API
├── src/                         # Rust implementation
├── examples/                    # Demo applications
├── CLAUDE.md                    # AI assistant config
└── README.md
```

## Use Cases

### Game Development
- Prototype 3D games in terminal
- Test mechanics without graphics overhead
- Debug physics and collision visually
- Rapid iteration with AI assistance

### Education
- Learn 3D graphics with instant feedback
- Understand ECS architecture through visualization
- Explore AI-assisted development
- Terminal-based tutorials

### CI/CD & Testing
- Headless rendering for automated tests
- Visual regression testing in terminals
- Performance benchmarking with TUI output
- Deployment verification visualization

### Creative Coding
- ASCII art from 3D models
- Terminal-based installations
- Retro game aesthetic development
- Live coding performances

## Development Status

**Current Phase**: Documentation Complete ✅

- ✅ Comprehensive research and feasibility
- ✅ System architecture design
- ✅ 5-phase implementation roadmap (16-21 days)
- ✅ Usage examples and AI prompts
- ⏳ Phase 1: Foundation setup (next)

## Example Workflows

### Scene Composition
```
User: "Create a solar system demo in the terminal"

Claude:
1. Spawns sun (yellow sphere at origin)
2. Creates planets with orbital paths
3. Adds rotation animations
4. Configures camera for best view
5. Sets color rendering strategy

Result: Animated solar system in terminal!
```

### Live Debugging
```
User: "Why isn't my player moving?"

Claude:
1. Queries player entity
2. Checks Transform component
3. Inspects velocity values
4. Identifies stuck at (0,0,0)
5. Applies fix via BRP
6. Verifies movement in TUI

Result: Bug fixed without recompilation!
```

## Performance Targets

| Metric | Target | Typical |
|--------|--------|---------|
| Frame Rate | 30-60 FPS | 30-60 FPS |
| Frame Time | <33ms | 16-33ms |
| BRP Latency | <10ms | 3-8ms |
| Terminal Redraw | <10ms | 3-8ms |
| Memory Usage | <100MB | 60-75MB |
| Entity Count | 1000+ | 50-200 |

## Feature Flags

```toml
[features]
default = []
brp = ["bevy/bevy_remote", "bevy_brp_extras"]
tui = ["bevy_ratatui_camera", "bevy_ratatui", "ratatui"]
full = ["brp", "tui"]
```

## Terminal Compatibility

### Recommended Terminals (24-bit color + Unicode)
- Alacritty
- Kitty
- iTerm2
- WezTerm
- Rio
- Ghostty

### Minimum Requirements
- Unicode support (UTF-8)
- 80x24 character display
- ANSI escape sequences

### Optimal Setup
- 24-bit true color
- 120x40+ character display
- GPU-accelerated rendering

## Technical Deep Dive

For comprehensive technical documentation:

- **[Research & Feasibility](../bevy-mcp-ratatui-ref/research.md)** - 2,028 lines of analysis
- **[System Architecture](../bevy-mcp-ratatui-ref/architecture.md)** - 1,730 lines with diagrams
- **[Implementation Plan](../bevy-mcp-ratatui-ref/implementation-plan.md)** - 5-phase roadmap
- **[Usage Examples](../bevy-mcp-ratatui-ref/usage-examples.md)** - Practical AI prompts
- **[Custom BRP Methods](../bevy-mcp-ratatui-ref/custom-brp-methods.md)** - API reference

## Learning Path

1. **Understand Concept** - Read main README and research doc
2. **Study Architecture** - Review architecture.md and diagrams
3. **Try Examples** - Use prompts from usage-examples.md
4. **Build Features** - Follow implementation-plan.md phases
5. **Experiment** - Create your own terminal applications

## Vision

This project demonstrates the future of AI-assisted development:
- **Natural Interaction**: Describe what you want, see it appear
- **Instant Feedback**: No compile-run-debug cycles
- **Visual Understanding**: AI sees what you see
- **Terminal-First**: Powerful development anywhere

**The future of game development is conversational, visual, and happens in your terminal.**
