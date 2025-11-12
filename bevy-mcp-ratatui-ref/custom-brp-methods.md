# Custom BRP Methods for Entity Spawning

This document describes the custom Bevy Remote Protocol (BRP) methods implemented for AI-controlled entity spawning.

## Overview

Standard BRP cannot spawn entities with meshes and materials because asset handles contain `Arc<StrongHandle>` which aren't serializable. The custom methods in this project solve this limitation by creating mesh and material assets internally and returning the spawned entity ID.

## Implementation

The custom BRP methods are implemented in `src/brp/tools.rs` via the `CustomBrpPlugin`.

### Plugin Registration

```rust
use bevy_mcp_ratatui_ref::prelude::*;

App::new()
    .add_plugins(CustomBrpPlugin)  // Registers custom BRP methods
    .add_plugins(BrpExtrasPlugin)  // Optional: adds screenshot, shutdown features
    .run();
```

## Available Methods

### bevy/spawn_cube

Spawns a cube entity with mesh and material.

**Endpoint**: `POST http://localhost:15702`

**Method**: `bevy/spawn_cube`

**Parameters**:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `position` | `[f32; 3]` | `[0, 0, 0]` | Position of the cube [x, y, z] |
| `scale` | `[f32; 3]` | `[1, 1, 1]` | Scale of the cube [x, y, z] |
| `color` | `[f32; 3]` | `[0.8, 0.7, 0.6]` | RGB color in range 0.0-1.0 |
| `metallic` | `f32` | `0.5` | Metallic value (0.0-1.0) |
| `roughness` | `f32` | `0.5` | Perceptual roughness (0.0-1.0) |
| `name` | `String` | `"AI Spawned Cube"` | Name for the entity |

**Returns**:

```json
{
  "entity": 4294967330,
  "name": "My Cube"
}
```

**Example Request** (using curl):

```bash
curl -X POST http://localhost:15702 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "bevy/spawn_cube",
    "params": {
      "position": [3.0, 1.0, 0.0],
      "scale": [1.0, 1.0, 1.0],
      "color": [0.8, 0.2, 0.2],
      "metallic": 0.7,
      "roughness": 0.3,
      "name": "Red Cube"
    }
  }'
```

**Example Request** (using MCP BRP tool):

```javascript
mcp__brp__brp_execute({
  method: "bevy/spawn_cube",
  params: {
    position: [3.0, 1.0, 0.0],
    color: [0.8, 0.2, 0.2],
    name: "Red Cube"
  }
})
```

### bevy/spawn_sphere

Spawns a sphere entity with mesh and material.

**Endpoint**: `POST http://localhost:15702`

**Method**: `bevy/spawn_sphere`

**Parameters**:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `position` | `[f32; 3]` | `[0, 0, 0]` | Position of the sphere [x, y, z] |
| `radius` | `f32` | `0.5` | Radius of the sphere |
| `color` | `[f32; 3]` | `[0.8, 0.7, 0.6]` | RGB color in range 0.0-1.0 |
| `metallic` | `f32` | `0.5` | Metallic value (0.0-1.0) |
| `roughness` | `f32` | `0.5` | Perceptual roughness (0.0-1.0) |
| `name` | `String` | `"AI Spawned Sphere"` | Name for the entity |

**Returns**:

```json
{
  "entity": 4294967331,
  "name": "My Sphere"
}
```

**Example Request** (using curl):

```bash
curl -X POST http://localhost:15702 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": 1,
    "method": "bevy/spawn_sphere",
    "params": {
      "position": [-3.0, 1.0, 0.0],
      "radius": 0.7,
      "color": [0.2, 0.2, 0.8],
      "metallic": 0.8,
      "roughness": 0.2,
      "name": "Blue Sphere"
    }
  }'
```

**Example Request** (using MCP BRP tool):

```javascript
mcp__brp__brp_execute({
  method: "bevy/spawn_sphere",
  params: {
    position: [-3.0, 1.0, 0.0],
    radius: 0.7,
    color: [0.2, 0.2, 0.8],
    name: "Blue Sphere"
  }
})
```

## AI Prompt Examples

When using with Claude Code or other AI assistants via MCP:

### Example 1: Adding a Cube

**Prompt**: "Add a red cube at position [3, 1, 0]"

**AI Action**:
```javascript
mcp__brp__brp_execute({
  method: "bevy/spawn_cube",
  params: {
    position: [3.0, 1.0, 0.0],
    color: [0.8, 0.2, 0.2],
    name: "Red Cube"
  }
})
```

### Example 2: Adding a Shiny Sphere

**Prompt**: "Spawn a shiny purple sphere at [-3, 1, 0]"

**AI Action**:
```javascript
mcp__brp__brp_execute({
  method: "bevy/spawn_sphere",
  params: {
    position: [-3.0, 1.0, 0.0],
    color: [0.6, 0.2, 0.8],
    metallic: 0.9,
    roughness: 0.1,
    name: "Purple Sphere"
  }
})
```

### Example 3: Adding Multiple Entities

**Prompt**: "Create a row of 3 cubes with different colors"

**AI Action**: (executes 3 spawn_cube calls in sequence)

## Technical Details

### Why Custom Methods Are Needed

Standard BRP provides these built-in methods:
- `bevy/spawn` - Spawns entities with serializable components only
- `bevy/insert` - Inserts components into existing entities
- `bevy/mutate_component` - Modifies component fields

However, asset handles (`Handle<Mesh>`, `Handle<StandardMaterial>`) cannot be serialized because they contain internal `Arc<StrongHandle>` references. This means you cannot use `bevy/spawn` to create entities with meshes and materials.

### How Custom Methods Work

Custom BRP methods are registered using the `RemotePlugin::with_method()` API:

```rust
impl Plugin for CustomBrpPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            RemotePlugin::default()
                .with_method("bevy/spawn_cube", Self::spawn_cube)
                .with_method("bevy/spawn_sphere", Self::spawn_sphere),
        )
        .add_plugins(RemoteHttpPlugin::default());
    }
}
```

The method handlers have this signature:

```rust
fn spawn_cube(
    In(params): In<Option<Value>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> BrpResult {
    // Parse parameters with defaults
    let params: SpawnCubeParams = params
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    // Create mesh and material assets
    let entity = commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial { /* ... */ })),
        Transform { /* ... */ },
        Name::new(params.name.clone()),
    )).id();

    // Return entity ID
    Ok(json!({
        "entity": entity.index(),
        "name": params.name,
    }))
}
```

### Extending with More Shapes

To add more shapes (plane, cylinder, torus, etc.), follow this pattern:

1. **Define parameter struct**:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SpawnPlaneParams {
    pub position: [f32; 3],
    pub size: [f32; 2],  // width, height
    pub color: [f32; 3],
    pub name: String,
}
```

2. **Implement handler**:

```rust
fn spawn_plane(
    In(params): In<Option<Value>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> BrpResult {
    let params: SpawnPlaneParams = params
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    let entity = commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(params.size[0], params.size[1]))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(params.color[0], params.color[1], params.color[2]),
            ..default()
        })),
        Transform::from_translation(Vec3::from(params.position)),
        Name::new(params.name.clone()),
    )).id();

    Ok(json!({
        "entity": entity.index(),
        "name": params.name,
    }))
}
```

3. **Register method**:

```rust
app.add_plugins(
    RemotePlugin::default()
        .with_method("bevy/spawn_cube", Self::spawn_cube)
        .with_method("bevy/spawn_sphere", Self::spawn_sphere)
        .with_method("bevy/spawn_plane", Self::spawn_plane),  // Add new method
)
```

## Error Handling

If parameters are malformed or missing, the methods use default values defined in the parameter structs. This makes the API forgiving for AI agents that might not provide complete parameters.

If BRP is not enabled (missing `--features brp`), the plugin will compile but do nothing (the methods are behind `#[cfg(feature = "brp")]`).

## Debugging

Enable Bevy logging to see spawned entity information:

```rust
info!("✅ Spawned cube '{}' at entity {:?}", params.name, entity);
```

Check BRP server status:

```bash
# Using MCP tools
mcp__brp__brp_status { app_name: "tui_brp" }

# Using curl
curl http://localhost:15702/methods
```

## Integration with Standard BRP

Custom methods work alongside all standard BRP methods:

- Use `bevy/spawn_cube` to create entities with meshes
- Use `bevy/mutate_component` to modify their transforms:

```javascript
mcp__brp__bevy_mutate_component({
  entity: 4294967330,
  component: "bevy_transform::components::transform::Transform",
  path: ".translation.y",
  value: 5.0
})
```

- Use `bevy/query` to find entities:

```javascript
mcp__brp__bevy_query({
  data: { components: ["bevy_ecs::name::Name"] },
  filter: { with: ["bevy_ecs::name::Name"] }
})
```

## Performance Considerations

Each spawn call:
- Creates new mesh asset (stored in `Assets<Mesh>`)
- Creates new material asset (stored in `Assets<StandardMaterial>`)
- Spawns entity with components

For better performance when spawning many entities:
- Consider reusing materials (requires more complex API)
- Batch spawn operations when possible
- Use simpler materials (lower metallic/roughness complexity)

## See Also

- [Bevy Remote Protocol Documentation](https://docs.rs/bevy/latest/bevy/remote/index.html)
- [bevy_brp_extras](https://github.com/Azorlogh/bevy_brp_extras)
- [Usage Examples](usage-examples.md) - More AI prompt patterns
- [Architecture Documentation](architecture.md) - System design details
