# Bevy MCP Reference Implementation

## Overview

The Bevy MCP reference demonstrates AI-assisted game development using Bevy game engine with the Bevy Remote Protocol (BRP) MCP server. This enables real-time interaction with running games through natural language commands via Claude Code.

## Key Concepts

### Bevy Remote Protocol (BRP)
- JSON-RPC interface for external tool communication
- Query and modify entity-component data in real-time
- Spawn/destroy entities without recompilation
- Access global resources and monitor changes

### AI-Assisted Workflow
1. Launch game with BRP enabled
2. Use Claude Code to inspect running game
3. Make live changes through MCP tools
4. Test ideas without recompiling
5. Finalize working code

## Quick Start

```bash
# Clone and run
cd bevy-mcp-ref
cargo run --features brp

# Game starts with BRP on localhost:15702
```

## MCP Tools Available

### Entity Management
- `bevy_spawn` - Create new entities
- `bevy_destroy` - Remove entities
- `bevy_query` - Query entities with specific components

### Component Operations
- `bevy_get` - Retrieve component data
- `bevy_insert` - Add components to entities
- `bevy_mutate_component` - Modify component fields (via bevy_brp_extras)

### Resource Access
- `bevy_get_resource` - Access global resources
- `bevy_mutate_resource` - Modify global resources

### Discovery & Monitoring
- `bevy_list` - List available components/resources
- `bevy_registry_schema` - Get component type schemas
- `bevy_get_watch` - Monitor entity changes
- `brp_list_logs` - Access application logs

## Example Workflows

### Spawning Entities
```javascript
// Via MCP tool
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      translation: [0.0, 5.0, 0.0],
      rotation: [0.0, 0.0, 0.0, 1.0],
      scale: [1.0, 1.0, 1.0]
    },
    "bevy_core::name::Name": "Golden Sphere"
  }
})
```

### Live Editing Transforms
```javascript
// Modify position in real-time
mcp__brp__bevy_mutate_component({
  entity: 123,
  component: "bevy_transform::components::transform::Transform",
  path: ".translation.y",
  value: 10.0
})
```

### Querying Scene State
```javascript
// Find all entities with cameras
mcp__brp__bevy_query({
  data: {
    components: ["bevy_transform::components::transform::Transform"]
  },
  filter: {
    with: ["bevy_render::camera::camera::Camera"]
  }
})
```

## Project Structure

```
bevy-mcp-ref/
├── src/
│   └── main.rs              # Main game with BRP
├── examples/
│   ├── basic_scene.rs       # Simple Bevy scene
│   └── brp_demo.rs          # Interactive BRP demo
├── docs/
│   ├── BRP_MCP_GUIDE.md     # Complete MCP reference
│   └── EXAMPLES.md          # Practical examples
├── CLAUDE.md                # AI assistant config
└── README.md
```

## Interactive Demo

The `brp_demo` example showcases:
- 3D park scene with rotating spheres
- Interactive first-person camera controls
- Named entities for easy MCP interaction
- Live component mutation demonstrations

```bash
cargo run --example brp_demo --features brp

# Controls:
# C - Grab cursor for mouse look
# WASD - Movement
# Mouse - Camera rotation
# Space/Shift - Fly up/down
# ESC - Release cursor
```

## Best Practices

### Entity Naming
Always name entities for AI-friendly interaction:
```rust
commands.spawn((
    // ... components
    Name::new("Player Character"),
));
```

### Plugin Setup
Include RemotePlugin for BRP access:
```rust
use bevy::prelude::*;
use bevy::remote::RemotePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RemotePlugin::default())
        .run();
}
```

### Component Registration
Register custom components with reflection:
```rust
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}
```

## Development Workflow

### Using justfile Commands
```bash
just demo              # Run interactive demo
just watch-demo        # Auto-reload on changes
just check-all         # Format, lint, test, build
just prod              # Production build
```

### Traditional Cargo Commands
```bash
cargo run --features brp               # Run with BRP
cargo build --release --features brp   # Release build
cargo test                             # Run tests
cargo clippy --all-features           # Lint
```

## Performance Benefits

Traditional workflow:
1. Write code
2. Compile (minutes)
3. Run game
4. Test
5. Repeat

BRP + MCP workflow:
1. Run game once
2. Test ideas via live editing
3. Experiment freely
4. Finalize working code
5. Minimal recompilation

**Result**: 10x faster iteration cycles!

## Technical Requirements

- **Rust**: Latest stable
- **Bevy**: 0.16+ (for full BRP support)
- **Claude Code**: For AI assistance
- **Dependencies**: bevy_brp_extras (for full mutation support)

## Further Reading

- [Full BRP MCP Guide](../bevy-mcp-ref/docs/BRP_MCP_GUIDE.md)
- [Practical Examples](../bevy-mcp-ref/docs/EXAMPLES.md)
- [Project README](../bevy-mcp-ref/README.md)
- [Bevy Documentation](https://bevyengine.org/learn/)
- [Claude Code Docs](https://docs.claude.com/en/docs/claude-code)
