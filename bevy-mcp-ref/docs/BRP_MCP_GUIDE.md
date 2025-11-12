# Bevy Remote Protocol (BRP) MCP Integration Guide

## Overview

The Bevy Remote Protocol (BRP) is a JSON-RPC interface that allows external tools to inspect and modify a running Bevy application. The MCP (Model Context Protocol) server provides Claude Code with direct access to BRP, enabling AI-assisted game development with live inspection and editing capabilities.

## Architecture

```
┌─────────────────┐         ┌──────────────┐         ┌─────────────────┐
│   Claude Code   │ ◄─────► │  BRP MCP     │ ◄─────► │  Bevy Game      │
│   (AI Agent)    │  MCP    │  Server      │  HTTP   │  (Port 15702)   │
└─────────────────┘         └──────────────┘         └─────────────────┘
```

## Setup

### 1. Enable BRP in Your Bevy App

Add the `bevy_remote` feature and plugin to your game:

```rust
use bevy::prelude::*;
use bevy::remote::RemotePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RemotePlugin::default()) // Enable BRP
        .run();
}
```

### 2. Build with BRP Feature

```bash
cargo run --features brp
```

The game will start and listen on `localhost:15702` for BRP requests.

### 3. Verify Connection

Claude can check if your game is running:

```javascript
mcp__brp__brp_status({ app_name: "bevy-mcp-ref" })
```

## Core Concepts

### Entity-Component-System (ECS)

Bevy uses ECS architecture:
- **Entities**: Unique identifiers (e.g., `123`, `456`)
- **Components**: Data attached to entities (e.g., `Transform`, `Camera`)
- **Systems**: Functions that process entities with specific components

### Component Type Names

BRP requires fully-qualified type names:
- `bevy_transform::components::transform::Transform`
- `bevy_render::camera::camera::Camera`
- `bevy_sprite::sprite::Sprite`

### Discovery

To find available components:

```javascript
// List all registered components
mcp__brp__bevy_list({})

// Get schema for specific components
mcp__brp__bevy_registry_schema({
  with_crates: ["bevy_transform"]
})
```

## Common Operations

### 1. Query Entities

Find entities matching criteria:

```javascript
// Find all entities with Transform and Camera
mcp__brp__bevy_query({
  data: {
    components: [
      "bevy_transform::components::transform::Transform",
      "bevy_render::camera::camera::Camera"
    ]
  },
  filter: {
    with: ["bevy_render::camera::camera::Camera"]
  }
})
```

### 2. Inspect Components

Get component data from a specific entity:

```javascript
mcp__brp__bevy_get({
  entity: 123,
  components: [
    "bevy_transform::components::transform::Transform"
  ]
})
```

### 3. Spawn Entities

Create new entities in the running game:

```javascript
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      "translation": [0.0, 5.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    }
  }
})
```

### 4. Modify Components

Update component values in real-time:

```javascript
// Change just the Y position
mcp__brp__bevy_mutate_component({
  entity: 123,
  component: "bevy_transform::components::transform::Transform",
  path: ".translation.y",
  value: 10.0
})
```

### 5. Watch for Changes

Monitor entities for live updates:

```javascript
mcp__brp__bevy_get_watch({
  entity: 123,
  components: [
    "bevy_transform::components::transform::Transform"
  ]
})
```

## Data Formats

### Vector Types

BRP uses **array format** for math types, not object notation:

```javascript
// ✅ CORRECT
"translation": [1.0, 2.0, 3.0]  // Vec3

// ❌ WRONG
"translation": {"x": 1.0, "y": 2.0, "z": 3.0}
```

### Common Types

- **Vec2**: `[x, y]`
- **Vec3**: `[x, y, z]`
- **Vec4**: `[x, y, z, w]`
- **Quaternion**: `[x, y, z, w]`
- **Transform**: Object with `translation`, `rotation`, `scale`

### Example Transform

```json
{
  "bevy_transform::components::transform::Transform": {
    "translation": [1.0, 2.0, 3.0],
    "rotation": [0.0, 0.0, 0.0, 1.0],
    "scale": [1.0, 1.0, 1.0]
  }
}
```

## Advanced Features

### Schema Discovery

Understand component structure before using:

```javascript
mcp__brp__bevy_registry_schema({
  with_types: ["Component"],
  with_crates: ["bevy_transform"]
})
```

### Entity Hierarchies

Manipulate parent-child relationships:

```javascript
// Make entities 124 and 125 children of entity 123
mcp__brp__bevy_reparent({
  entities: [124, 125],
  parent: 123
})
```

### Resource Management

Access global game resources:

```javascript
// Get the Time resource
mcp__brp__bevy_get_resource({
  resource: "bevy_time::time::Time"
})

// Modify a resource field
mcp__brp__bevy_mutate_resource({
  resource: "my_game::config::GameSettings",
  path: ".difficulty",
  value: "hard"
})
```

## Best Practices

### 1. Use Name Component

Always name entities for easier identification:

```rust
commands.spawn((
    // ... other components
    Name::new("Player Character"),
));
```

Query by name pattern using standard queries.

### 2. Small Iterations

Make incremental changes via BRP:
1. Query current state
2. Make small mutation
3. Observe result
4. Refine and repeat

### 3. Watch Before Mutating

Understand dynamic behavior:
1. Start a watch on relevant entities
2. Observe how values change during gameplay
3. Make informed mutations based on observations

### 4. Schema First

Always check schema before spawning complex components:
1. Use `bevy_registry_schema` to see structure
2. Match the exact format required
3. Validate with simple spawns first

### 5. Error Handling

Common errors and solutions:

| Error | Cause | Solution |
|-------|-------|----------|
| Component not found | Wrong type name | Use `bevy_list` to find correct name |
| Invalid path | Wrong field name | Check schema with `bevy_registry_schema` |
| Entity not found | Entity ID changed | Query entities again or use Name |
| Type mismatch | Wrong value format | Use array format for Vec types |

## Workflow Examples

### Live Material Editing

```javascript
// 1. Find all entities with materials
const entities = mcp__brp__bevy_query({
  data: { components: ["bevy_pbr::pbr_material::StandardMaterial"] },
  filter: { with: ["bevy_pbr::pbr_material::StandardMaterial"] }
})

// 2. Change the color of entity 123
mcp__brp__bevy_mutate_component({
  entity: 123,
  component: "bevy_pbr::pbr_material::StandardMaterial",
  path: ".base_color",
  value: [1.0, 0.0, 0.0, 1.0]  // Red
})
```

### Debug Camera Position

```javascript
// 1. Find camera
const cameras = mcp__brp__bevy_query({
  data: { components: ["bevy_transform::components::transform::Transform"] },
  filter: { with: ["bevy_render::camera::camera::Camera"] }
})

// 2. Move camera to better view
mcp__brp__bevy_mutate_component({
  entity: cameras[0].entity,
  component: "bevy_transform::components::transform::Transform",
  path: ".translation",
  value: [10.0, 10.0, 10.0]
})
```

### Performance Monitoring

```javascript
// Watch Time resource to see FPS and delta
const watch = mcp__brp__bevy_get_watch({
  entity: TIME_RESOURCE_ID,
  components: ["bevy_time::time::Time"]
})

// Read logs later
mcp__brp__brp_read_log({ filename: "watch_12345.log" })
```

## Troubleshooting

### Game Not Responding

1. Check game is running: `ps aux | grep bevy-mcp-ref`
2. Verify BRP port: `lsof -i :15702`
3. Check logs: `cargo run --features brp 2>&1 | grep -i "remote"`

### Component Not Found

1. List all components: `mcp__brp__bevy_list({})`
2. Check crate name matches
3. Ensure component is registered with reflection

### Invalid Mutations

1. Get current value: `mcp__brp__bevy_get()`
2. Check path syntax: Use leading dot `.field.subfield`
3. Verify value type matches schema

## Reference

### Available MCP Tools

- **App Management**: `brp_launch_bevy_app`, `brp_status`, `brp_list_bevy_apps`
- **Entity Operations**: `bevy_spawn`, `bevy_destroy`, `bevy_query`, `bevy_reparent`
- **Component Operations**: `bevy_get`, `bevy_insert`, `bevy_remove`, `bevy_mutate_component`
- **Resource Operations**: `bevy_get_resource`, `bevy_insert_resource`, `bevy_mutate_resource`
- **Discovery**: `bevy_list`, `bevy_registry_schema`, `bevy_rpc_discover`
- **Monitoring**: `bevy_get_watch`, `bevy_list_watch`, `brp_stop_watch`
- **Logging**: `brp_list_logs`, `brp_read_log`, `brp_cleanup_logs`

### Port Configuration

Default BRP port is `15702`. To change:

```rust
RemotePlugin {
    port: 8080,  // Custom port
}
```

### Security Considerations

- BRP exposes full control over the running game
- Only use on localhost in development
- Never expose BRP port to public networks
- Consider authentication for production tools

---

For more information, see:
- [Official Bevy Remote Protocol Documentation](https://github.com/bevyengine/bevy/tree/main/crates/bevy_remote)
- [Example Code](../examples/brp_demo.rs)
- [Component Reference](COMPONENT_REFERENCE.md)
