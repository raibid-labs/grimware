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
- **Bevy 0.16+** - This project requires Bevy 0.16 for full BRP support

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/bevy-mcp-ref.git
cd bevy-mcp-ref

# Run the game with BRP enabled
cargo run --features brp

# OR use the justfile for convenience (if you have just installed)
just demo
```

The game will start with BRP listening on `localhost:15702`.

**💡 Tip:** This project includes a `justfile` with convenient commands. Install [just](https://github.com/casey/just) and run `just` to see all available commands.

### Running Examples

```bash
# Basic scene without BRP
cargo run --example basic_scene
# OR: just example-basic

# Interactive BRP demo
cargo run --example brp_demo --features brp
# OR: just demo
```

## 🤖 AI-Assisted Development

### MCP Integration

The Bevy Remote Protocol (BRP) MCP server provides Claude Code with direct access to your running game. This project uses **bevy_brp_extras** for extended functionality including component mutation. Available tools include:

- **Entity Management**: `bevy_spawn`, `bevy_destroy`, `bevy_query`
- **Component Operations**: `bevy_get`, `bevy_insert`, `bevy_mutate_component` (via bevy_brp_extras)
- **Resource Access**: `bevy_get_resource`, `bevy_mutate_resource` (via bevy_brp_extras)
- **Discovery**: `bevy_list`, `bevy_registry_schema`
- **Monitoring**: `bevy_get_watch`, `brp_list_logs`

**Note**: The `bevy_brp_extras` plugin includes `RemotePlugin` and `RemoteHttpPlugin` internally, providing full mutation support beyond base BRP capabilities.

### Example Workflow

1. **Start your game:**
   ```bash
   cargo run --features brp
   ```

2. **Ask Claude to inspect it:**
   > "Show me all entities with Transform components"

3. **Make live changes:**
   > "Make the green sphere jump twice as high"

4. **Test ideas without recompiling:**
   > "Spawn a golden sphere at position (5, 0, 0) with a metallic material"

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

### Quick Commands with justfile

This project includes a [justfile](https://github.com/casey/just) for convenient development commands:

```bash
# See all available commands
just

# Run the interactive demo
just demo

# Run with auto-reload on file changes
just watch-demo

# Full quality check (format, lint, test, build)
just check-all

# Production build with all checks
just prod
```

### Building

```bash
# Debug build (with dynamic linking for faster compile)
cargo build
# OR: just build

# Debug build with BRP
cargo build --features brp
# OR: just build-brp

# Release build
cargo build --release --features brp
# OR: just build-release
```

### Testing

```bash
# Run tests
cargo test
# OR: just test

# Check code
cargo check --all-features
# OR: just check
```

### Linting

```bash
# Format code
cargo fmt
# OR: just fmt

# Lint
cargo clippy --all-features
# OR: just lint
```

## 🎨 Examples Gallery

### Basic Scene
A simple 3D scene with rotating cube, ground plane, and camera.
```bash
cargo run --example basic_scene
```

### BRP Interactive Demo
Immersive 3D park scene with interactive camera controls, demonstrating full BRP capabilities:

**Visual Features:**
- 3 rotating spheres (red, green bouncing, blue) with smooth animations
- Green sphere performs spectacular parabolic arc trajectory (jumps over blue sphere)
- 5 trees with brown trunks and green foliage in circular arrangement
- 8 decorative rocks scattered naturally around the scene
- Grass-colored 25x25 ground plane
- Bright ambient lighting (500 brightness) + 3 point lights for even illumination

**Interactive Controls:**
- **Press 'C'** to grab cursor and enable mouse look
- **WASD** - First-person movement (forward/left/back/right)
- **Mouse** - Free-look camera rotation (when cursor grabbed)
- **Space** - Fly up
- **Shift** - Fly down
- **ESC** - Release cursor

**BRP Features Demonstrated:**
- Live component mutation (modify sphere bounce height in real-time)
- Entity querying and inspection
- Transform manipulation
- Component registration with reflection system
- 30+ named entities for easy MCP interaction

```bash
cargo run --example brp_demo --features brp
```

**Pro Tip:** Use the camera controls to explore the scene from different angles and watch the green sphere's parabolic jump from various perspectives!

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

### Spawn a sphere
```javascript
mcp__brp__bevy_spawn({
  components: {
    "bevy_transform::components::transform::Transform": {
      "translation": [0.0, 5.0, 0.0],
      "rotation": [0.0, 0.0, 0.0, 1.0],
      "scale": [1.0, 1.0, 1.0]
    },
    "bevy_core::name::Name": "My Sphere"
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
- Modify component values (requires bevy_brp_extras for full mutation support)
- Spawn and destroy entities
- Access global resources
- Monitor changes in real-time

This project uses **bevy_brp_extras** (v0.1+) which extends base BRP with:
- `bevy/mutate_component` - Modify specific component fields without replacing entire components
- `bevy/mutate_resource` - Modify specific resource fields
- Full reflection support for custom components

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
2. Test ideas via live editing (change sphere height from 5.0 → 12.0 in seconds!)
3. Experiment freely (move objects, change colors, spawn entities)
4. Finalize code based on what works
5. Minimal compilation needed

**Real Example:** In this demo, we modified the green sphere's bounce height from 5.0 to 8.0 to 12.0 **all while the game was running** - no recompilation required!

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