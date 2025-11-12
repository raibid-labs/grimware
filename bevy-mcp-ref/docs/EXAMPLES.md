# Bevy MCP Examples

This document provides practical examples of using MCP tools to interact with a running Bevy game.

## Prerequisites

Ensure your game is running with BRP enabled:

```bash
cargo run --features brp
# OR run the interactive demo:
cargo run --example brp_demo --features brp
```

## Example 1: Modifying the Green Sphere's Bounce (Using BRP Demo)

### Goal
Use live BRP mutation to modify the green sphere's bounce height while the game is running - no recompilation needed!

### Steps

1. **Launch the BRP demo:**
```bash
cargo run --example brp_demo --features brp
```

2. **Find the green sphere entity:**

Ask Claude Code: "Show me all entities with the BouncingCube component"

Or use direct query:
```javascript
mcp__brp__bevy_query({
  data: { components: ["brp_demo::BouncingCube", "bevy_core::name::Name"] },
  filter: { with: ["brp_demo::BouncingCube"] }
})
```

Response will show entity ID (e.g., 4294967322) and current values:
```json
{
  "bevy_core::name::Name": "Green Sphere",
  "brp_demo::BouncingCube": {
    "base_height": 0.5,
    "height": 5.0,
    "speed": 2.0
  }
}
```

3. **Make it jump higher (live editing):**

Ask Claude Code: "Make the green sphere jump twice as high"

Or use direct mutation:
```javascript
mcp__brp__brp_execute({
  method: "bevy/mutate_component",
  params: {
    entity: 4294967322,
    component: "brp_demo::BouncingCube",
    path: ".height",
    value: 10
  }
})
```

4. **Watch the change happen instantly:**
The sphere immediately starts jumping higher! No restart needed.

5. **Experiment with different values:**
```javascript
// Make it super dramatic
mcp__brp__brp_execute({
  method: "bevy/mutate_component",
  params: {
    entity: 4294967322,
    component: "brp_demo::BouncingCube",
    path: ".height",
    value: 15
  }
})

// Slow it down for cinematic effect
mcp__brp__brp_execute({
  method: "bevy/mutate_component",
  params: {
    entity: 4294967322,
    component: "brp_demo::BouncingCube",
    path: ".speed",
    value: 1.0
  }
})
```

6. **When you find values you like, update the code:**
Once you've experimented and found the perfect bounce, update `examples/brp_demo.rs` with those values and commit.

### Key Takeaway
This demonstrates the power of AI-assisted game development: **test ideas instantly, iterate rapidly, finalize working code.**

## Example 2: Basic Scene Setup

### Goal
Create a simple 3D scene with a cube, light, and camera using MCP tools.

### Steps

1. **Launch the game:**
```bash
cargo run --features brp
```

2. **Spawn a cube:**
```javascript
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      "translation": [0.0, 0.5, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    }
  }
})
```

3. **Spawn a light:**
```javascript
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      "translation": [4.0, 8.0, 4.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    }
  }
})
```

## Example 2: Live Parameter Tuning

### Goal
Adjust game parameters in real-time without recompiling.

### Scenario: Tuning Light Intensity

1. **Find all lights:**
```javascript
mcp__brp__bevy_query({
  data: {
    components: ["bevy_pbr::light::PointLight"]
  },
  filter: {
    with: ["bevy_pbr::light::PointLight"]
  }
})
```

2. **Get current light settings:**
```javascript
mcp__brp__bevy_get({
  entity: 123,  // From query results
  components: ["bevy_pbr::light::PointLight"]
})
```

3. **Adjust intensity:**
```javascript
mcp__brp__bevy_mutate_component({
  entity: 123,
  component: "bevy_pbr::light::PointLight",
  path: ".intensity",
  value: 2000.0
})
```

4. **Test different values:**
```javascript
// Try different intensities
mcp__brp__bevy_mutate_component({
  entity: 123,
  component: "bevy_pbr::light::PointLight",
  path: ".intensity",
  value: 500.0
})

// Once you find the perfect value, update your code
```

## Example 3: Debugging Entity Behavior

### Goal
Monitor an entity's transform to understand its movement.

### Steps

1. **Find the entity:**
```javascript
mcp__brp__bevy_query({
  data: {
    components: [
      "bevy_transform::components::transform::Transform",
      "bevy_core::name::Name"
    ]
  },
  filter: {
    with: ["bevy_core::name::Name"]
  }
})
```

2. **Watch for changes:**
```javascript
mcp__brp__bevy_get_watch({
  entity: 123,
  components: [
    "bevy_transform::components::transform::Transform"
  ]
})
```

3. **Read the watch log:**
```javascript
// After some gameplay
mcp__brp__brp_list_logs({})

mcp__brp__brp_read_log({
  filename: "bevy_brp_mcp_watch_1_get_123_1234567890.log",
  tail_lines: 50
})
```

## Example 4: Rapid Prototyping

### Goal
Quickly test different game object configurations.

### Scenario: Testing Different Enemy Spawns

1. **Spawn multiple test enemies:**
```javascript
// Enemy at position 1
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      "translation": [5.0, 0.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [0.5, 0.5, 0.5]
    }
  }
})

// Enemy at position 2
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      "translation": [-5.0, 0.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [0.5, 0.5, 0.5]
    }
  }
})
```

2. **Test different scales:**
```javascript
// Make enemy 1 bigger
mcp__brp__bevy_mutate_component({
  entity: 456,
  component: "bevy_transform::components::transform::Transform",
  path: ".scale",
  value: [2.0, 2.0, 2.0]
})
```

3. **Clean up test entities:**
```javascript
mcp__brp__bevy_destroy({ entity: 456 })
mcp__brp__bevy_destroy({ entity: 457 })
```

## Example 5: Camera Control

### Goal
Dynamically adjust camera position for perfect framing.

### Steps

1. **Find camera:**
```javascript
mcp__brp__bevy_query({
  data: {
    components: ["bevy_transform::components::transform::Transform"]
  },
  filter: {
    with: ["bevy_render::camera::camera::Camera"]
  }
})
```

2. **Adjust camera position:**
```javascript
mcp__brp__bevy_mutate_component({
  entity: 789,
  component: "bevy_transform::components::transform::Transform",
  path: ".translation",
  value: [-5.0, 5.0, 10.0]
})
```

3. **Fine-tune view:**
```javascript
// Move just Y axis
mcp__brp__bevy_mutate_component({
  entity: 789,
  component: "bevy_transform::components::transform::Transform",
  path: ".translation.y",
  value: 7.5
})
```

## Example 6: Material Experimentation

### Goal
Test different material properties without recompiling.

### Steps

1. **Query entities with materials:**
```javascript
mcp__brp__bevy_query({
  data: {
    components: ["bevy_pbr::pbr_material::StandardMaterial"]
  },
  filter: {
    with: ["bevy_pbr::pbr_material::StandardMaterial"]
  }
})
```

2. **Change material color:**
```javascript
mcp__brp__bevy_mutate_component({
  entity: 234,
  component: "bevy_pbr::pbr_material::StandardMaterial",
  path: ".base_color",
  value: [0.8, 0.2, 0.2, 1.0]  // Reddish
})
```

3. **Adjust metallic/roughness:**
```javascript
mcp__brp__bevy_mutate_component({
  entity: 234,
  component: "bevy_pbr::pbr_material::StandardMaterial",
  path: ".metallic",
  value: 0.8
})

mcp__brp__bevy_mutate_component({
  entity: 234,
  component: "bevy_pbr::pbr_material::StandardMaterial",
  path: ".perceptual_roughness",
  value: 0.2
})
```

## Example 7: Hierarchy Management

### Goal
Organize entities into parent-child relationships.

### Steps

1. **Create parent entity:**
```javascript
const parent = mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      "translation": [0.0, 0.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    }
  }
})
```

2. **Create child entities:**
```javascript
const child1 = mcp__brp__bevy_spawn({...})
const child2 = mcp__brp__bevy_spawn({...})
```

3. **Set up hierarchy:**
```javascript
mcp__brp__bevy_reparent({
  entities: [child1, child2],
  parent: parent
})
```

4. **Move parent (children follow):**
```javascript
mcp__brp__bevy_mutate_component({
  entity: parent,
  component: "bevy_transform::components::transform::Transform",
  path: ".translation",
  value: [5.0, 0.0, 0.0]
})
```

## Example 8: Performance Monitoring

### Goal
Monitor game performance in real-time.

### Steps

1. **Get Time resource:**
```javascript
mcp__brp__bevy_get_resource({
  resource: "bevy_time::time::Time"
})
```

2. **Count entities:**
```javascript
const allEntities = mcp__brp__bevy_query({
  data: {
    components: ["bevy_transform::components::transform::Transform"]
  },
  filter: {}
})

console.log(`Total entities: ${allEntities.length}`)
```

3. **Profile component usage:**
```javascript
// Count cameras
const cameras = mcp__brp__bevy_query({
  filter: { with: ["bevy_render::camera::camera::Camera"] }
})

// Count lights
const lights = mcp__brp__bevy_query({
  filter: { with: ["bevy_pbr::light::PointLight"] }
})
```

## Example 9: Schema Discovery

### Goal
Understand available components and their structure.

### Steps

1. **List all components:**
```javascript
mcp__brp__bevy_list({})
```

2. **Get specific crate schemas:**
```javascript
mcp__brp__bevy_registry_schema({
  with_crates: ["bevy_transform"],
  with_types: ["Component"]
})
```

3. **Explore component structure:**
```javascript
// Find Transform schema
const schemas = mcp__brp__bevy_registry_schema({
  with_crates: ["bevy_transform"]
})

// Look for "Transform" in results
// Use its properties to understand structure
```

## Example 10: AI-Assisted Development Workflow

### Complete Workflow Example

1. **AI Reads Codebase:**
```bash
# Claude reads game code to understand architecture
Read("src/main.rs")
Read("src/systems/player.rs")
```

2. **User Runs Game:**
```bash
cargo run --features brp
```

3. **AI Verifies Connection:**
```javascript
mcp__brp__brp_status({ app_name: "bevy-mcp-ref" })
```

4. **AI Tests Feature Idea:**
```javascript
// Spawn test entity
const testEntity = mcp__brp__bevy_spawn({...})

// Try different parameters
mcp__brp__bevy_mutate_component({...})
```

5. **AI Updates Code:**
```bash
# Based on successful BRP experiments
Edit("src/systems/player.rs")
Write("tests/player_movement.rs")
```

6. **User Recompiles:**
```bash
cargo run --features brp
```

7. **AI Verifies Fix:**
```javascript
// Query for expected behavior
mcp__brp__bevy_query({...})
```

## Tips for Effective Usage

1. **Always name your entities:** Makes them easier to find and identify
2. **Start with queries:** Understand state before making changes
3. **Use watches for dynamic behavior:** Monitor changes over time
4. **Iterate in small steps:** Make one change, observe, repeat
5. **Schema first:** Check component structure before spawning/mutating
6. **Clean up test entities:** Use `bevy_destroy` for temporary objects
7. **Log everything:** Use watches and logs for debugging

## Next Steps

- Explore [BRP MCP Guide](BRP_MCP_GUIDE.md) for detailed API reference
- Check [Component Reference](COMPONENT_REFERENCE.md) for common components
- Try the included examples: `cargo run --example brp_demo --features brp`
- Experiment with your own workflows!

---

*Remember: The power of BRP + MCP is that you can iterate on ideas without waiting for compilation. Experiment freely!*
