# Bevy MCP Reference - AI Assistant Configuration

## 🎮 Project Overview

This is a reference implementation for developing Bevy games with AI assistance using the Bevy Remote Protocol (BRP) MCP server. The project demonstrates how to integrate Claude Code with a live Bevy game engine instance for interactive development, debugging, and live editing.

## 🚀 Quick Start

### Running the Game with BRP

```bash
# Run with Bevy Remote Protocol enabled for MCP integration
cargo run --features brp

# Or run a specific example
cargo run --example brp_demo --features brp
```

The game will start with BRP enabled on port 15702, allowing MCP tools to interact with it.

### MCP Integration

The BRP MCP server is already configured in this project. Available tools include:

- **Entity Management**: `mcp__brp__bevy_spawn`, `mcp__brp__bevy_destroy`, `mcp__brp__bevy_query`
- **Component Operations**: `mcp__brp__bevy_get`, `mcp__brp__bevy_insert`, `mcp__brp__bevy_remove`
- **Live Editing**: `mcp__brp__bevy_mutate_component`, `mcp__brp__bevy_mutate_resource`
- **Inspection**: `mcp__brp__bevy_list`, `mcp__brp__bevy_registry_schema`
- **Monitoring**: `mcp__brp__bevy_get_watch`, `mcp__brp__bevy_list_watch`
- **App Management**: `mcp__brp__brp_launch_bevy_app`, `mcp__brp__brp_status`

## 🎯 AI-Assisted Development Workflow

### 1. Launch and Monitor

```bash
# Launch the game (automatically detected by MCP)
cargo run --features brp

# Check if the game is running and BRP is active
# Claude can use: mcp__brp__brp_status
```

### 2. Inspect and Understand

```bash
# Query entities in the running game
# List all components
# Get component schemas
# Watch entities for changes
```

### 3. Live Edit and Debug

```bash
# Modify component values in real-time
# Spawn new entities
# Change materials, transforms, etc.
# All without recompiling!
```

### 4. Iterate Rapidly

The AI assistant can:
- Modify game code in the editor
- Update components in the running game instantly via BRP
- Test changes without restart
- Document what works

## 📋 Development Guidelines for AI Assistants

### File Organization

- **NEVER save to root**: Use organized directories
  - `/src` - Main game code
  - `/examples` - Example scenes and demos
  - `/assets` - Game assets (models, textures, audio)
  - `/docs` - Documentation and guides
  - `/tests` - Integration and unit tests

### Bevy-Specific Best Practices

1. **ECS Architecture**: Use Entity-Component-System patterns
2. **Plugin System**: Organize features as plugins
3. **Resource Management**: Use Bevy's asset system
4. **Performance**: Profile before optimizing
5. **BRP Integration**: Always add `RemotePlugin` for AI assistance

### Live Development Pattern

```rust
// 1. Add RemotePlugin to your app
app.add_plugins(RemotePlugin::default());

// 2. Name your entities for easy identification
commands.spawn((
    // ... components
    Name::new("Player Character"),
));

// 3. Use descriptive component names
// 4. Structure code for easy MCP access
```

## 🔧 Common AI-Assisted Tasks

### Spawning Entities via MCP

The AI can spawn entities in your running game:

```javascript
// Example: Spawn a new cube at a specific position
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      translation: [5.0, 2.0, 0.0],
      rotation: [0.0, 0.0, 0.0, 1.0],
      scale: [1.0, 1.0, 1.0]
    }
  }
})
```

### Querying Game State

```javascript
// Find all entities with Transform and Camera
mcp__brp__bevy_query({
  data: {
    components: ["bevy_transform::components::transform::Transform"]
  },
  filter: {
    with: ["bevy_render::camera::camera::Camera"]
  }
})
```

### Live Editing Transforms

```javascript
// Move an entity in real-time
mcp__brp__bevy_mutate_component({
  entity: 123,
  component: "bevy_transform::components::transform::Transform",
  path: ".translation.y",
  value: 5.0
})
```

## 🎮 Example Workflows

### 1. Creating a New Game Feature

1. AI reads existing code to understand architecture
2. AI creates new component/system code
3. User runs game with BRP enabled
4. AI spawns test entities via MCP
5. AI adjusts values in real-time to perfect the feature
6. Code is finalized based on live testing

### 2. Debugging Gameplay

1. User reports unexpected behavior
2. AI queries entities to inspect state
3. AI watches components for changes
4. AI identifies the issue
5. AI suggests and tests fix via BRP
6. Code is updated with the solution

### 3. Rapid Prototyping

1. AI creates prototype system
2. Game runs with BRP
3. AI spawns various test scenarios
4. AI tunes parameters in real-time
5. Best values are captured and committed

## 📚 Additional Resources

- [Bevy Documentation](https://bevyengine.org/learn/)
- [Bevy Remote Protocol Guide](https://github.com/bevyengine/bevy/tree/main/crates/bevy_remote)
- [MCP BRP Documentation](docs/BRP_MCP_GUIDE.md)
- [Example Gallery](docs/EXAMPLES.md)

## 🚨 Important Notes

- Always run with `--features brp` for MCP integration
- BRP listens on port 15702 by default
- Entity IDs are session-specific (reset on restart)
- Use `Name` component for persistent entity identification
- Component type names must be fully qualified for BRP operations

## 💡 Tips for Effective AI Collaboration

1. **Name Everything**: Use the `Name` component extensively
2. **Document Intent**: Add comments explaining game design decisions
3. **Small Iterations**: Make small changes and test via BRP
4. **Query First**: Always query state before making assumptions
5. **Watch for Changes**: Use watch tools to understand dynamic behavior
6. **Schema Discovery**: Use registry tools to explore available components

---

**Remember**: This project demonstrates the power of AI-assisted game development. The BRP MCP integration allows Claude to interact with your running game in real-time, enabling a development workflow that's faster and more intuitive than traditional edit-compile-run cycles.
