# MCP Integration Guide

Model Context Protocol (MCP) integration patterns across Grimware reference implementations.

## Overview

MCP enables AI assistants like Claude Code to interact with running applications through standardized tool interfaces. Two projects in this repository demonstrate MCP integration:

- **Bevy MCP**: Live game development with BRP
- **Bevy MCP Ratatui**: Terminal-based 3D visualization

## MCP Fundamentals

### What is MCP?

MCP (Model Context Protocol) is a protocol for connecting AI models to external tools and data sources. It provides:

- **Standardized Interface**: Consistent tool calling patterns
- **Type Safety**: Schema-defined parameters and returns
- **Bidirectional Communication**: Tools can stream data
- **Context Awareness**: Tools understand application state

### Architecture

```
Claude Code (AI) → MCP Bridge → Application Tools → Your App
```

## Bevy Remote Protocol (BRP) via MCP

### Available Tools

All tools are prefixed with `mcp__brp__`:

**Entity Management**:
- `bevy_spawn` - Create entities with components
- `bevy_destroy` - Remove entities
- `bevy_query` - Query entities by components

**Component Operations**:
- `bevy_get` - Read component data
- `bevy_insert` - Add components
- `bevy_remove` - Remove components
- `bevy_mutate_component` - Modify fields (bevy_brp_extras)

**Resource Access**:
- `bevy_get_resource` - Read global resources
- `bevy_mutate_resource` - Modify resources

**Discovery**:
- `bevy_list` - List available types
- `bevy_registry_schema` - Get type schemas

**Monitoring**:
- `bevy_get_watch` - Subscribe to changes
- `bevy_list_watch` - List watched entities
- `brp_list_logs` - Read application logs

**App Management**:
- `brp_status` - Check if app is running
- `brp_launch_bevy_app` - Start application

### Example Usage

**Check Application Status**:
```javascript
mcp__brp__brp_status({
  app_name: "bevy-mcp-ref"
})
```

**Query Entities**:
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

**Spawn Entity**:
```javascript
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      translation: [0.0, 5.0, 0.0],
      rotation: [0.0, 0.0, 0.0, 1.0],
      scale: [1.0, 1.0, 1.0]
    },
    "bevy_core::name::Name": "Test Entity"
  }
})
```

**Mutate Component**:
```javascript
mcp__brp__bevy_mutate_component({
  entity: 123,
  component: "bevy_transform::components::transform::Transform",
  path: ".translation.y",
  value: 10.0
})
```

## Custom BRP Methods

Standard BRP has limitations (e.g., cannot spawn entities with meshes). The Bevy MCP Ratatui project extends BRP with custom methods:

### spawn_cube

```javascript
mcp__brp__brp_execute({
  method: "bevy/spawn_cube",
  params: {
    position: [3.0, 1.0, 0.0],
    scale: [1.0, 1.5, 1.0],
    color: [0.8, 0.2, 0.2],
    metallic: 0.7,
    roughness: 0.3,
    name: "Red Cube"
  }
})
```

### spawn_sphere

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

## AI Interaction Patterns

### Pattern 1: Inspection → Modification

```
User: "Make the player jump higher"

Claude:
1. Query for entity named "Player"
2. Get current Transform component
3. Mutate translation.y value
4. Verify change via query
```

### Pattern 2: Spawning with Naming

```
User: "Add three colored cubes in a row"

Claude:
1. Spawn cube at [-2, 0, 0] named "Red Cube"
2. Spawn cube at [0, 0, 0] named "Green Cube"
3. Spawn cube at [2, 0, 0] named "Blue Cube"
4. Query to verify all spawned
```

### Pattern 3: Watching and Reacting

```
User: "Monitor the player's position"

Claude:
1. Query for "Player" entity ID
2. Set up watch on Transform component
3. Report position changes
4. Alert if position becomes invalid
```

## Best Practices

### Entity Naming

Always use descriptive names:

```rust
commands.spawn((
    // ... components
    Name::new("Player Character"),
));
```

Benefits:
- AI can find entities semantically
- Easier debugging and inspection
- Persistent identification across runs

### Component Registration

Register custom components with reflection:

```rust
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// In app setup
app.register_type::<Velocity>();
```

### Error Handling

MCP tools return Results - handle gracefully:

```javascript
try {
  const result = await mcp__brp__bevy_query({ ... });
  // Process result
} catch (error) {
  console.error("Query failed:", error);
  // Fallback behavior
}
```

### Type Names

Use fully qualified type names:

```javascript
// ✓ Correct
"bevy_transform::components::transform::Transform"

// ✗ Incorrect
"Transform"
```

## Debugging MCP Integration

### Check BRP Server Status

```bash
# Check if port is open
lsof -i :15702

# Test with curl
curl -X POST http://localhost:15702 \
  -H "Content-Type: application/json" \
  -d '{"jsonrpc":"2.0","method":"bevy/list","id":1}'
```

### Enable Verbose Logging

```bash
RUST_LOG=debug cargo run --features brp
```

### Inspect MCP Tool Calls

Claude Code shows tool calls in the conversation. Review:
- Tool name and parameters
- Return values and errors
- Timing and performance

## Performance Considerations

### Query Optimization

- Use specific component filters
- Limit query results when possible
- Cache entity IDs when known

### Mutation Frequency

- Batch updates when possible
- Use watches for high-frequency monitoring
- Consider update rate limits

### Resource Management

- Clean up watches when no longer needed
- Remove temporary entities
- Monitor memory usage in long sessions

## Security Considerations

### Local Development Only

BRP listens on `localhost:15702` by default:
- Not exposed to network
- Safe for development
- Do not expose in production

### Validation

All BRP operations are validated:
- Type checking via reflection
- Component existence verification
- Entity validation

### Authorization

Current implementation has no authentication:
- Suitable for local development
- Consider adding auth for remote access
- Use firewall rules for protection

## Example Workflows

See individual project documentation for detailed workflows:
- [Bevy MCP Guide](./bevy-mcp.md) - Game development patterns
- [Bevy MCP Ratatui Guide](./bevy-mcp-ratatui.md) - Terminal visualization
- [BRP Usage](./brp-usage.md) - Low-level BRP details

## Further Reading

- [Model Context Protocol Docs](https://modelcontextprotocol.io/)
- [Bevy Remote Protocol](https://github.com/bevyengine/bevy/tree/main/crates/bevy_remote)
- [bevy_brp_extras](https://crates.io/crates/bevy_brp_extras)
- [Claude Code Documentation](https://docs.claude.com/en/docs/claude-code)
