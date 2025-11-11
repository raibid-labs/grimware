# Architecture Summary: AI-Driven Bevy MCP TUI

## Quick Reference

This is a condensed summary of the full architecture document. For complete details, see [architecture.md](./architecture.md).

## System Components

```
AI (Claude)
  ↓ Natural Language
MCP Server (Tool Calls)
  ↓ JSON-RPC
BRP Client → Bevy App (ECS)
  ↓ Scene Data
TUI Renderer (Ratatui)
  ↓ ANSI
Terminal Display
```

## Core MCP Tools

| Tool | Purpose | Example |
|------|---------|---------|
| `bevy_spawn` | Create entities | Spawn cube, sphere, plane |
| `bevy_mutate_component` | Modify entities | Update position, color |
| `bevy_query` | Query scene | List entities in bounds |
| `bevy_destroy` | Remove entities | Delete by ID |
| `bevy_camera_control` | Camera control | Set position, render mode |

## Key Technologies

- **Bevy 0.16+**: ECS game engine
- **BRP**: Bevy Remote Protocol for external control
- **MCP**: Model Context Protocol for AI tool calls
- **bevy_ratatui_camera**: Terminal 3D rendering
- **Ratatui**: Terminal UI framework

## Rendering Strategies

1. **ASCII Renderer**: Character density (@%#*+=-:.)
2. **Color Renderer**: RGB with Unicode blocks (█)
3. **Edge Renderer**: Wireframe with box drawing chars (│┌┐)

## Architecture Principles

### Separation of Concerns
- **AI Layer**: Understands intent, generates commands
- **MCP Bridge**: Translates to BRP, validates, tracks state
- **Bevy App**: Manages 3D scene, physics, entities
- **TUI Layer**: Renders to terminal, handles input

### Performance Targets
- Frame Rate: 30-60 FPS
- BRP Latency: < 10ms
- Entity Limit: 1000+
- Memory: < 100MB
- Startup: < 2s

### Error Handling
- **Retry**: BRP timeouts (3 attempts, exponential backoff)
- **Fallback**: Rendering errors (ASCII mode)
- **Skip**: Entity not found
- **Abort**: Critical errors

## Implementation Phases

1. **Core Infrastructure** (2 weeks): Bevy setup, BRP, MCP server
2. **TUI Rendering** (2 weeks): Ratatui integration, renderers
3. **MCP Tools** (1 week): Spawn, mutate, query, destroy, camera
4. **Scene Management** (1 week): Behaviors, prefabs, serialization
5. **Polish** (2 weeks): Edge detection, controls, optimization
6. **Testing** (1 week): Integration, benchmarks, documentation

## Extensibility

### Plugin System
```rust
pub trait BevyMcpExtension: Plugin {
    fn register_mcp_tools(&self, registry: &mut ToolRegistry);
    fn register_render_strategies(&self, strategies: &mut RenderStrategyRegistry);
    fn register_behaviors(&self, behaviors: &mut BehaviorRegistry);
    fn add_systems(&self, app: &mut App);
}
```

### Custom Renderers
Implement `RenderStrategy` trait for custom rendering logic.

### Entity Behaviors
Implement `EntityBehavior` trait for custom animations/physics.

## Security Measures

- Input validation (entity limits, parameter sizes)
- Rate limiting (100 commands/second)
- ANSI escape sequence sanitization
- Resource quotas (max entities, vertices, memory)

## Configuration

Environment variables with `BEVY_MCP_` prefix:
- `BEVY_MCP_BRP_HOST` - BRP server host
- `BEVY_MCP_BRP_PORT` - BRP server port (default: 6001)
- `BEVY_MCP_MCP_SERVER_PORT` - MCP server port (default: 6000)
- `BEVY_MCP_RENDERING_WIDTH` - Terminal width (default: 80)
- `BEVY_MCP_RENDERING_HEIGHT` - Terminal height (default: 24)

## Quick Start Example

```bash
# Terminal 1: Start Bevy app with BRP
cargo run --bin bevy-mcp-tui

# Terminal 2: Start MCP server
cargo run --bin mcp-server

# Terminal 3: AI interaction (via Claude Desktop)
# AI will use MCP tools to control the scene
```

## Project Structure

```
bevy-mcp-ratatui-ref/
├── src/
│   ├── main.rs                 # Bevy app entry point
│   ├── lib.rs                  # Core library
│   ├── plugins/
│   │   ├── brp_server.rs       # BRP integration
│   │   ├── tui_render.rs       # Ratatui rendering
│   │   └── scene_manager.rs    # Entity/scene management
│   ├── rendering/
│   │   ├── strategies.rs       # ASCII/Color/Edge renderers
│   │   ├── camera.rs           # RatatuiCamera component
│   │   └── frame_buffer.rs     # Terminal frame buffer
│   ├── mcp/
│   │   ├── server.rs           # MCP server
│   │   ├── bridge.rs           # MCP-to-BRP translation
│   │   └── tools.rs            # Tool implementations
│   └── ecs/
│       ├── components.rs       # Custom components
│       ├── systems.rs          # Bevy systems
│       └── behaviors.rs        # Entity behaviors
├── docs/
│   ├── architecture.md         # Full architecture document
│   └── ARCHITECTURE_SUMMARY.md # This file
├── config/
│   └── default.toml            # Default configuration
├── examples/
│   ├── spinning_cube.rs        # Basic example
│   └── ai_controlled_scene.rs  # AI interaction example
└── tests/
    ├── integration/
    └── e2e/
```

## Development Workflow

1. AI sends natural language prompt
2. MCP server translates to tool calls
3. BRP client sends JSON-RPC to Bevy
4. Bevy ECS updates entities/components
5. TUI renderer projects 3D to 2D
6. Ratatui draws to terminal
7. User sees real-time 3D in terminal

## Testing Strategy

- **Unit Tests**: Components, renderers, projections (10-15 tests)
- **Integration Tests**: MCP-BRP workflow, rendering pipeline (5-8 tests)
- **E2E Tests**: Full AI-to-terminal scenarios (3-5 tests)
- **Manual Tests**: User acceptance, visual quality (2-3 scenarios)

## Monitoring

- Frame times and FPS
- BRP request latencies
- Entity counts
- Memory usage
- Error rates

## Future Enhancements

1. Custom entity types and procedural geometry
2. Ray-marching renderer for volumetric effects
3. Particle system renderer
4. Multi-user collaboration
5. Web terminal interface
6. AI-driven camera cinematography

---

For complete architectural details, design patterns, code examples, and decision rationale, see the full [architecture.md](./architecture.md) document.
