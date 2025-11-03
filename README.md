# Bevy MCP Reference Implementation

A reference implementation demonstrating AI-assisted game development with Bevy game engine using the Bevy Remote Protocol (BRP) MCP server and Claude Code.

## 🎮 Overview

This project showcases how to integrate **Claude Code** with a **Bevy game engine** instance for interactive development, live debugging, and real-time entity manipulation. Using the Bevy Remote Protocol (BRP), you can query, inspect, and modify your running game without recompiling.

## ✨ Features

- **Live Game Inspection**: Query entities, components, and resources in real-time
- **Real-time Editing**: Modify transforms, materials, and game state without recompiling
- **AI-Assisted Development**: Claude Code can interact with your running game via MCP tools
- **Comprehensive Examples**: Includes demos showing BRP capabilities
- **Full Documentation**: Detailed guides for effective AI collaboration

## 🚀 Quick Start

### Prerequisites

- Rust (latest stable) - [Install here](https://www.rust-lang.org/tools/install)
- Claude Code CLI - [Install guide](https://docs.claude.com/en/docs/claude-code)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/bevy-mcp-ref.git
cd bevy-mcp-ref

# Run the game with BRP enabled
cargo run --features brp
```

The game will start with BRP listening on `localhost:15702`.

### Running Examples

```bash
# Basic scene without BRP
cargo run --example basic_scene

# Interactive BRP demo
cargo run --example brp_demo --features brp
```

## 🤖 AI-Assisted Development

### MCP Integration

The Bevy Remote Protocol (BRP) MCP server provides Claude Code with direct access to your running game. Available tools include:

- **Entity Management**: `bevy_spawn`, `bevy_destroy`, `bevy_query`
- **Component Operations**: `bevy_get`, `bevy_insert`, `bevy_mutate_component`
- **Resource Access**: `bevy_get_resource`, `bevy_mutate_resource`
- **Discovery**: `bevy_list`, `bevy_registry_schema`
- **Monitoring**: `bevy_get_watch`, `brp_list_logs`

### Example Workflow

1. **Start your game:**
   ```bash
   cargo run --features brp
   ```

2. **Ask Claude to inspect it:**
   > "Show me all entities with Transform components"

3. **Make live changes:**
   > "Move the camera to position (10, 10, 10)"

4. **Test ideas without recompiling:**
   > "Spawn a red cube at position (5, 0, 0)"

5. **Finalize code:**
   > "Update the code with the changes that worked"

## 📚 Documentation

- **[CLAUDE.md](CLAUDE.md)** - AI assistant configuration and guidelines
- **[docs/BRP_MCP_GUIDE.md](docs/BRP_MCP_GUIDE.md)** - Complete BRP MCP reference
- **[docs/EXAMPLES.md](docs/EXAMPLES.md)** - Practical usage examples
- **[Bevy Documentation](https://bevyengine.org/learn/)** - Official Bevy resources

## 🎯 Project Structure

```
bevy-mcp-ref/
├── src/
│   └── main.rs              # Main game with BRP enabled
├── examples/
│   ├── basic_scene.rs       # Simple Bevy scene
│   └── brp_demo.rs          # Interactive BRP demonstration
├── docs/
│   ├── BRP_MCP_GUIDE.md     # Complete MCP tools reference
│   └── EXAMPLES.md          # Practical examples
├── assets/                  # Game assets (empty initially)
├── CLAUDE.md                # AI assistant instructions
├── Cargo.toml               # Project dependencies
└── README.md                # This file
```

## 🔧 Development

### Building

```bash
# Debug build (with dynamic linking for faster compile)
cargo build

# Debug build with BRP
cargo build --features brp

# Release build
cargo build --release --features brp
```

### Testing

```bash
# Run tests
cargo test

# Check code
cargo check --all-features
```

### Linting

```bash
# Format code
cargo fmt

# Lint
cargo clippy --all-features
```

## 🎨 Examples Gallery

### Basic Scene
A simple 3D scene with rotating cube, ground plane, and camera.
```bash
cargo run --example basic_scene
```

### BRP Interactive Demo
Multi-cube scene with orbiting camera, demonstrating BRP capabilities:
- Named entities for easy identification
- Multiple colored cubes with different rotation speeds
- Dynamic lighting setup
- Real-time instructions display

```bash
cargo run --example brp_demo --features brp
```

## 🛠️ MCP Tools Quick Reference

### Check if game is running
```javascript
mcp__brp__brp_status({ app_name: "bevy-mcp-ref" })
```

### Query all entities
```javascript
mcp__brp__bevy_query({
  data: { components: ["bevy_transform::components::transform::Transform"] },
  filter: {}
})
```

### Spawn a cube
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

### Move an entity
```javascript
mcp__brp__bevy_mutate_component({
  entity: 123,
  component: "bevy_transform::components::transform::Transform",
  path: ".translation.y",
  value: 10.0
})
```

See [docs/EXAMPLES.md](docs/EXAMPLES.md) for more examples!

## 🔍 Key Concepts

### Bevy Remote Protocol (BRP)

BRP is a JSON-RPC interface that allows external tools to:
- Query entity-component data
- Modify component values
- Spawn and destroy entities
- Access global resources
- Monitor changes in real-time

### Entity-Component-System (ECS)

Bevy uses ECS architecture:
- **Entities**: Unique identifiers for game objects
- **Components**: Data attached to entities (Transform, Camera, etc.)
- **Systems**: Functions that process entities with specific components

### MCP Integration

The MCP server bridges Claude Code and BRP:
1. You run your Bevy game with `RemotePlugin`
2. BRP listens on port 15702
3. Claude Code uses MCP tools to send BRP commands
4. Your game responds with data or applies changes

## 💡 Why This Matters

Traditional game development workflow:
1. Write code
2. Compile (can take minutes)
3. Run game
4. Test
5. Repeat

With BRP + MCP workflow:
1. Run game once
2. Test ideas via live editing
3. Finalize code based on what works
4. Minimal compilation needed

**Result**: Faster iteration, more experimentation, better games!

## 🤝 Contributing

This is a reference implementation. Feel free to:
- Fork and customize for your projects
- Add examples demonstrating new BRP capabilities
- Improve documentation
- Share your AI-assisted development workflows

## 📄 License

MIT License - See LICENSE file for details

## 🔗 Resources

- [Bevy Game Engine](https://bevyengine.org/)
- [Bevy Remote Protocol](https://github.com/bevyengine/bevy/tree/main/crates/bevy_remote)
- [Claude Code Documentation](https://docs.claude.com/en/docs/claude-code)
- [Model Context Protocol](https://modelcontextprotocol.io/)

## 🎓 Learning Path

1. **Start here**: Run `cargo run --example brp_demo --features brp`
2. **Read**: [docs/BRP_MCP_GUIDE.md](docs/BRP_MCP_GUIDE.md)
3. **Try**: Examples from [docs/EXAMPLES.md](docs/EXAMPLES.md)
4. **Experiment**: Ask Claude to help you build features
5. **Build**: Create your own game with AI assistance!

---

**Built with ❤️ using [Bevy](https://bevyengine.org/) and [Claude Code](https://claude.com/claude-code)**

*Happy AI-assisted game development! 🎮🤖*