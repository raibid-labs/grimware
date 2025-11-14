# Bevy WASM F# Reference Implementation

[![CI Status](https://github.com/raibid-labs/grimware/actions/workflows/bevy-wasm-fsharp-ci.yml/badge.svg)](https://github.com/raibid-labs/grimware/actions/workflows/bevy-wasm-fsharp-ci.yml)

A reference implementation demonstrating the **F# → Rust → Bevy → WASM** development path for game development. This project showcases how F# game logic can be transpiled to Rust and integrated into a Bevy game engine application that runs both natively and in the browser.

## 🎮 Overview

This project proves out a novel game development workflow:

1. **Write game logic in F#** - Type-safe, functional approach
2. **Transpile to Rust** - Via Fable + fsrs (raibid-labs/fable, raibid-labs/fsrs)
3. **Integrate with Bevy** - Rust ECS game engine
4. **Deploy anywhere** - Native desktop or WebAssembly

**This is a reference integration**, not a complete game. It demonstrates the technical path for future Grimware projects.

## ✨ Features

- **Dual-Target Deployment**: Single codebase runs natively and in browser (WASM)
- **F# Game Logic**: Type-safe functional programming for game rules
- **Bevy Integration**: Modern ECS architecture with Rust performance
- **Simple Combat System**: Player vs monster with stats, abilities, and events
- **Domain-Driven Design**: Clear separation between domain logic and presentation

## 🚀 Quick Start

### Prerequisites

- **Rust** (latest stable) - [Install here](https://www.rust-lang.org/tools/install)
- **F# SDK** (for future fsrs integration) - [Install here](https://dotnet.microsoft.com/download)
- **wasm-bindgen-cli** (for WASM builds) - `cargo install wasm-bindgen-cli`
- **just** (optional, for convenience) - [Install here](https://github.com/casey/just)

### Installation

```bash
# Clone the repository
git clone https://github.com/raibid-labs/grimware.git
cd grimware/bevy-wasm-fsharp-ref

# Run natively
cargo run -p app

# OR with justfile
just run
```

### Controls

- **Space** - Attack on your turn
- **ESC** - Quit

The game features **turn-based combat**:
- Player and monster take turns attacking
- Press Space during your turn to attack
- Monster attacks automatically after a 1-second delay
- Combat events and HP are logged to the console
- Game ends when either combatant reaches 0 HP

## 🎯 Project Structure

```
bevy-wasm-fsharp-ref/
├── crates/
│   ├── app/              # Bevy game application
│   │   ├── src/
│   │   │   └── main.rs   # Main game loop, ECS systems
│   │   └── Cargo.toml    # Bevy dependencies
│   └── logic-fsharp/     # Game logic crate (Rust)
│       ├── src/
│       │   └── lib.rs    # Domain types, combat functions
│       └── Cargo.toml    # Logic crate dependencies
├── fsharp/
│   ├── Domain.fs         # F# domain types (mirror of Rust)
│   └── GameLogic.fs      # F# game logic (future fsrs source)
├── web/
│   ├── index.html        # WASM loader page
│   └── wasm-bindgen.sh   # Build script (to be implemented)
├── docs/
│   ├── architecture.md   # Architecture overview
│   └── fsharp-integration.md  # F# integration details
├── CLAUDE.md             # AI assistant instructions
├── CLAUDE_NOTES.md       # Original design document
├── justfile              # Development commands
└── README.md             # This file
```

## 🧩 Architecture

### The F# → Rust → Bevy Path

```
┌─────────────────┐
│  F# Source Code │  ← Game logic, domain types
│  (fsharp/)      │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Fable + fsrs    │  ← F# to Rust transpiler
│ Transpilation   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Rust Crate     │  ← Generated Rust code
│(logic-fsharp/)  │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│  Bevy App       │  ← ECS systems, rendering
│  (crates/app/)  │
└────────┬────────┘
         │
         ├─────────────┐
         ▼             ▼
    ┌────────┐   ┌──────────┐
    │ Native │   │   WASM   │
    │ Binary │   │  Bundle  │
    └────────┘   └──────────┘
```

### Current State

**Note**: The `logic-fsharp` crate is currently **hand-written Rust** that mirrors expected fsrs output. This allows rapid prototyping of the Bevy integration while fsrs matures.

**Future**: `logic-fsharp` will be replaced by actual fsrs-generated code from F# sources (see issue #5).

### Domain Model

**Character** - Player or monster
```rust
pub struct Character {
    pub name: String,
    pub hp: i32,
    pub stats: Stats,
}
```

**Stats** - Character attributes
```rust
pub struct Stats {
    pub hp: i32,      // Max health
    pub attack: i32,  // Attack power
    pub defense: i32, // Damage reduction
}
```

**Ability** - Attack or skill
```rust
pub struct Ability {
    pub name: String,
    pub power: i32,
}
```

**CombatEvent** - Attack result
```rust
pub struct CombatEvent {
    pub attacker_name: String,
    pub defender_name: String,
    pub damage: i32,
    pub defender_hp_after: i32,
}
```

### Combat System

**Turn-Based Combat** with automatic monster AI:

1. Game starts with **Player's Turn**
2. Player presses Space → attack executes
3. Damage calculated: `(attacker.attack + ability.power - defender.defense).max(1)`
4. HP and events logged to console
5. State switches to **Monster's Turn**
6. After 1 second delay, monster attacks automatically
7. State switches back to **Player's Turn**
8. Repeat until either HP ≤ 0 → **Game Over**

**Combat State Machine:**
- `PlayerTurn` → Player input accepted, monster inactive
- `MonsterTurn` → Monster attacks after timer, player input ignored
- `GameOver { winner }` → Combat ended, winner announced

See [docs/combat-system.md](docs/combat-system.md) for complete details.

## 🔧 Development

### Quick Commands with justfile

```bash
# See all available commands
just

# Run the game natively
just run

# Run with auto-reload on file changes
just watch

# Build for all targets
just build               # Native debug
just build-release       # Native release
just build-wasm          # WASM bundle

# WASM workflow
just wasm               # Build and serve WASM
just serve-wasm         # Serve existing WASM build

# Quality checks
just test               # Run tests
just check              # Check code
just fmt                # Format code
just lint               # Run clippy
just check-all          # Full quality check (fmt + lint + test + build)

# Information
just info               # Show project info
```

### Traditional Cargo Commands

```bash
# Native development
cargo run -p app
cargo build --release -p app

# WASM build (manual)
cargo build -p app --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir web/pkg --target web \
    target/wasm32-unknown-unknown/release/app.wasm

# Testing
cargo test --all-features

# Code quality
cargo fmt
cargo clippy --all-features
```

## 🌐 WASM Deployment

### Building for WASM

```bash
# Option 1: Use justfile (recommended)
just build-wasm

# Option 2: Manual build
cargo build -p app --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir web/pkg --target web \
    target/wasm32-unknown-unknown/release/app.wasm
```

### Testing Locally

```bash
# Serve WASM build locally
just serve-wasm

# Or manually with Python
cd web
python3 -m http.server 8000

# Then open: http://localhost:8000
```

### WASM Limitations

- File system operations not supported
- Some Bevy features may be unavailable
- Audio/input handling differs from native
- See [Bevy WASM guide](https://bevyengine.org/learn/book/getting-started/setup/#webassembly) for details

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p logic-fsharp

# Run with output
cargo test -- --nocapture

# Check code without building
cargo check --all-features
```

## 📚 Documentation

### Project Documentation
- **[CLAUDE.md](CLAUDE.md)** - AI assistant configuration and development guidelines
- **[CLAUDE_NOTES.md](CLAUDE_NOTES.md)** - Original design document and planning notes
- **[docs/architecture.md](docs/architecture.md)** - Architecture overview
- **[docs/combat-system.md](docs/combat-system.md)** - Turn-based combat system details
- **[docs/fsharp-integration.md](docs/fsharp-integration.md)** - F# integration details

### Consolidated Documentation
- **[docs/bevy-wasm-fsharp.md](../docs/bevy-wasm-fsharp.md)** - Comprehensive guide (root docs/)

### External Resources
- [Bevy Documentation](https://bevyengine.org/learn/)
- [Fable F# Compiler](https://fable.io/)
- [fsrs - F# to Rust](https://github.com/raibid-labs/fsrs)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)

## 🐛 Known Issues & Roadmap

### Current Limitations

1. **No Visual Rendering** (Issue #2)
   - Currently no sprites/shapes displayed
   - Combat is text-only (console output)

2. **WASM Build Incomplete** (Issue #3)
   - `web/wasm-bindgen.sh` needs implementation
   - `web/index.html` needs proper WASM loader

3. **F# Logic Not Implemented** (Issue #4)
   - `fsharp/GameLogic.fs` is empty
   - Need to implement combat functions in F#

4. **fsrs Integration Pending** (Issue #5)
   - Currently using hand-written Rust
   - Need automated transpilation pipeline

5. **No Combat UI** (Issue #6)
   - No on-screen HP display
   - No visual combat feedback

6. **Basic Monster AI** (Issue #7)
   - Monster uses simple auto-attack strategy
   - Advanced AI behaviors planned (Issue #11)

### Roadmap

See [GitHub Issues](https://github.com/raibid-labs/grimware/issues?q=is%3Aissue+is%3Aopen+label%3Abevy-wasm-fsharp-ref) for detailed work items.

**Phase 1** (Current):
- ✅ Basic Bevy app with ECS setup
- ✅ Domain types (Character, Stats, Ability, CombatEvent)
- ✅ Turn-based combat system (Issue #14)
- ✅ Basic monster AI (auto-attack)
- ✅ F# type mirrors

**Phase 2** (In Progress):
- 🔄 Visual rendering (sprites/shapes)
- 🔄 Complete WASM build
- 🔄 F# GameLogic.fs implementation

**Phase 3** (Future):
- ⏳ fsrs transpilation pipeline
- ⏳ Combat UI with HP bars
- ⏳ Monster AI logic
- ⏳ Additional abilities and combat mechanics

## 💡 Key Concepts

### F# to Rust Type Mapping

F# records map to Rust structs:

**F# (Domain.fs):**
```fsharp
type Stats =
    { Hp: int
      Attack: int
      Defense: int }
```

**Rust (lib.rs):**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}
```

**Key Differences:**
- F# uses PascalCase, Rust uses snake_case
- F# `int` = Rust `i32`
- Rust needs explicit derives for functionality

### Bevy ECS Pattern

Entities are just IDs with attached components:

```rust
// Spawn player entity
commands.spawn((
    Player,                                    // Tag component
    logic::Character::new_player("Hero"),    // Game logic component
    Transform::from_xyz(-100.0, 0.0, 0.0),   // Position component
));

// Query entities in systems
fn tick_combat(
    mut players: Query<&mut Character, With<Player>>,
    mut monsters: Query<&mut Character, With<Monster>>,
) {
    // Process entities with these components
}
```

### Functional Core, Imperative Shell

- **Functional Core**: Pure logic in `logic-fsharp` crate (no side effects)
- **Imperative Shell**: Bevy systems handle ECS mutations and I/O

This separation makes logic testable and reusable.

## 🤝 Contributing

This is a reference implementation for the Grimware project. Contributions welcome:

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Ensure both native and WASM builds work
5. Update documentation
6. Submit a pull request

Please maintain F# ↔ Rust type alignment and follow repository conventions.

## 📄 License

MIT License - See LICENSE file for details

## 🔗 Related Projects

### Grimware Reference Implementations
- [bevy-mcp-ref](../bevy-mcp-ref/) - AI-assisted game development
- [bevy-mcp-ratatui-ref](../bevy-mcp-ratatui-ref/) - Terminal 3D rendering
- [tauri-ref](../tauri-ref/) - Cross-platform apps
- [webatui-ref](../webatui-ref/) - Terminal UI library

### External Projects
- [raibid-labs/fable](https://github.com/raibid-labs/fable) - Fable fork with Rust backend
- [raibid-labs/fsrs](https://github.com/raibid-labs/fsrs) - F# to Rust transpiler
- [Bevy Engine](https://bevyengine.org/) - Rust game engine
- [Fable](https://fable.io/) - F# to JavaScript compiler

## 🎓 Learning Path

**For F# Developers:**
1. Start with native build: `just run`
2. Review F# types in `fsharp/Domain.fs`
3. Explore Rust equivalents in `crates/logic-fsharp/src/lib.rs`
4. Read `docs/fsharp-integration.md`

**For Rust/Bevy Developers:**
1. Explore ECS setup in `crates/app/src/main.rs`
2. Understand domain types in `crates/logic-fsharp/src/lib.rs`
3. Learn about F# transpilation in `docs/architecture.md`
4. Try modifying combat logic

**For WASM Developers:**
1. Build WASM: `just build-wasm`
2. Serve locally: `just serve-wasm`
3. Inspect browser console output
4. Review `web/index.html` and WASM loading

---

**Built with ❤️ using [Bevy](https://bevyengine.org/), [F#](https://fsharp.org/), and [Rust](https://www.rust-lang.org/)**

*Demonstrating the F# → Rust → Bevy → WASM path for next-generation game development*
