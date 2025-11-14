# Bevy WASM F# Reference - AI Assistant Configuration

## 🎮 Project Overview

This is a reference implementation demonstrating the **F# → Rust → Bevy → WASM** development path. The project showcases how F# game logic can be transpiled to Rust via Fable + fsrs, integrated into a Bevy game engine application, and deployed both natively and as WebAssembly.

## 🚀 Quick Start

### Running Natively

```bash
# Run the game (desktop)
cd bevy-wasm-fsharp-ref
cargo run -p app

# Or with justfile
just run
```

### Building for WASM

```bash
# Build WASM bundle
just build-wasm

# Serve locally
just serve-wasm

# Build and serve in one command
just wasm
```

## 🎯 Project Goals

This is a **reference integration experiment**, not a full game. The goals are:

1. **Prove the F# → Rust → Bevy path works**
   - F# defines game rules and logic
   - Fable + fsrs transpile F# to Rust
   - Rust code integrates seamlessly with Bevy

2. **Dual-target deployment**
   - Native desktop (fast iteration, debugging)
   - WebAssembly (browser deployment)

3. **Minimal but realistic example**
   - Simple combat system (player vs monster)
   - Clear domain model (Character, Stats, Ability, CombatEvent)
   - Demonstrates F# strengths: type safety, functional patterns

## 📋 Development Guidelines for AI Assistants

### File Organization

**NEVER save to repository root** unless it's README, CLAUDE.md, or LICENSE.

Organize by purpose:
- **`/crates/app`** - Bevy application (main game loop, ECS systems)
- **`/crates/logic-fsharp`** - Rust domain/logic crate (currently hand-written, will be fsrs-generated)
- **`/fsharp`** - F# source files (Domain.fs, GameLogic.fs)
- **`/web`** - WASM deployment (index.html, build scripts)
- **`/docs`** - Project-specific documentation

### Architecture Understanding

```
F# Source Code (fsharp/)
    ↓
Fable Compiler + fsrs
    ↓
Rust Code (crates/logic-fsharp/)
    ↓
Bevy App (crates/app/)
    ↓
    ├─→ Native Binary
    └─→ WASM Bundle (web/)
```

**Current State**: The `logic-fsharp` crate is **hand-written Rust** that mirrors what fsrs output would look like. This is intentional - it allows the Bevy integration to be developed and tested while fsrs matures.

**Future State**: `logic-fsharp` will be replaced by actual fsrs-generated code from F# sources.

### Bevy-Specific Patterns

1. **Entity-Component-System (ECS)**
   ```rust
   // Components are data
   #[derive(Component)]
   struct Player;

   // Entities are IDs with components
   commands.spawn((
       Player,
       logic::Character::new_player("Hero"),
       Transform::from_xyz(-100.0, 0.0, 0.0),
   ));

   // Systems query and process entities
   fn tick_combat(
       mut players: Query<&mut Character, With<Player>>,
       mut monsters: Query<&mut Character, With<Monster>>,
   ) { /* ... */ }
   ```

2. **Keep Systems Simple**
   - Systems handle ECS orchestration
   - Delegate complex logic to the `logic` crate
   - Example: `tick_combat` calls `logic::compute_attack()`

3. **WASM Considerations**
   - Use `bevy_window::WindowPlugin` with WASM-friendly defaults
   - Avoid file system operations in WASM builds
   - Use feature flags for WASM-specific code

### F# to Rust Alignment

The F# types in `fsharp/Domain.fs` must align **exactly** with Rust types in `crates/logic-fsharp/src/lib.rs`:

**F# Domain.fs:**
```fsharp
type Stats =
    { Hp: int
      Attack: int
      Defense: int }
```

**Rust lib.rs:**
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}
```

**Key Differences:**
- F# uses PascalCase fields, Rust uses snake_case
- F# `int` = Rust `i32`
- F# records = Rust structs with serde support

### Common AI-Assisted Tasks

#### 1. Adding a New F# Function

1. Define in `fsharp/GameLogic.fs`:
   ```fsharp
   let healCharacter character amount =
       { character with Hp = min (character.Hp + amount) character.Stats.Hp }
   ```

2. Mirror in `crates/logic-fsharp/src/lib.rs`:
   ```rust
   pub fn heal_character(character: &Character, amount: i32) -> Character {
       let new_hp = (character.hp + amount).min(character.stats.hp);
       Character { hp: new_hp, ..character.clone() }
   }
   ```

3. Call from Bevy system:
   ```rust
   fn heal_system(mut query: Query<&mut Character>) {
       for mut character in query.iter_mut() {
           *character = logic::heal_character(&character, 5);
       }
   }
   ```

#### 2. Testing WASM Builds

```bash
# Build WASM
just build-wasm

# Serve locally (opens browser)
just serve-wasm

# Test in browser:
# - Open developer console
# - Check for errors
# - Test game controls (Space to attack)
```

#### 3. Modifying Combat Logic

Always update **both** F# and Rust versions:

1. Update `fsharp/GameLogic.fs` (canonical source)
2. Update `crates/logic-fsharp/src/lib.rs` (current implementation)
3. Test with `cargo run -p app`
4. Test WASM with `just wasm`

## 🔧 Common Commands

### With justfile (recommended)

```bash
# See all available commands
just

# Development
just run              # Run native build
just watch           # Auto-reload on changes

# Building
just build           # Build all crates
just build-wasm      # Build WASM bundle
just build-release   # Release build

# Testing & Quality
just test            # Run tests
just check           # Check code
just fmt             # Format code
just lint            # Run clippy
just check-all       # Format, lint, test, build

# WASM Workflow
just wasm            # Build and serve WASM
just serve-wasm      # Serve existing WASM build
```

### With cargo (traditional)

```bash
# Native development
cargo run -p app
cargo build --release

# WASM (requires wasm-bindgen-cli)
cargo build -p app --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir web/pkg --target web target/wasm32-unknown-unknown/release/app.wasm
```

## 📚 Key Files

### Bevy Application
- **`crates/app/src/main.rs`** - Main game loop, ECS setup
- **`crates/app/Cargo.toml`** - Bevy dependencies

### Game Logic
- **`crates/logic-fsharp/src/lib.rs`** - Domain types and combat functions (Rust)
- **`crates/logic-fsharp/Cargo.toml`** - Logic crate dependencies
- **`fsharp/Domain.fs`** - Domain types (F# mirror)
- **`fsharp/GameLogic.fs`** - Game logic (F# mirror, not yet implemented)

### WASM Deployment
- **`web/index.html`** - WASM loader page
- **`web/wasm-bindgen.sh`** - Build script (to be implemented)

### Documentation
- **`CLAUDE_NOTES.md`** - Original planning document
- **`docs/architecture.md`** - Architecture overview
- **`docs/fsharp-integration.md`** - F# integration details

## 🎯 Domain Model

### Types

**Character** - Player or monster entity
- `name: String` - Display name
- `hp: i32` - Current health
- `stats: Stats` - Base attributes

**Stats** - Character attributes
- `hp: i32` - Maximum health
- `attack: i32` - Attack power
- `defense: i32` - Damage reduction

**Ability** - Special move or attack
- `name: String` - Ability name
- `power: i32` - Base power

**CombatEvent** - Result of an attack
- `attacker_name: String`
- `defender_name: String`
- `damage: i32` - Damage dealt
- `defender_hp_after: i32` - Remaining HP

### Combat Formula

```rust
// In logic-fsharp/src/lib.rs
pub fn compute_attack(attacker: &Character, defender: &Character, ability: &Ability) -> CombatEvent {
    let raw = attacker.stats.attack + ability.power;
    let dmg = (raw - defender.stats.defense).max(1); // Minimum 1 damage
    let hp_after = defender.hp - dmg;
    // ...
}
```

## 💡 Best Practices

### 1. Type Safety First
- F# types are the **source of truth**
- Rust types must match exactly
- Use serde for serialization consistency

### 2. Functional Core, Imperative Shell
- **F#/Rust logic**: Pure functions, no side effects
- **Bevy systems**: Handle ECS mutations and I/O

### 3. WASM Testing
- Test native first (faster iteration)
- Test WASM before committing
- Check browser console for errors

### 4. Documentation
- Update architecture.md when changing structure
- Document F# → Rust type mappings
- Explain any workarounds for fsrs limitations

## 🚨 Important Notes

### Current Limitations

1. **fsrs Integration Not Yet Complete**
   - `logic-fsharp` is hand-written, not generated
   - See issue #5 for fsrs integration work
   - This is intentional for rapid prototyping

2. **Minimal Visuals**
   - Currently no sprites/shapes rendered
   - Combat is text-based (console output)
   - See issue #2 for visual rendering work

3. **WASM Build Incomplete**
   - `web/wasm-bindgen.sh` needs implementation
   - See issue #3 for WASM completion work

### F# Transpilation Workflow (Future)

When fsrs is integrated:

```bash
# 1. Edit F# source
vim fsharp/GameLogic.fs

# 2. Transpile to Rust
fable fsharp/GameLogic.fs --lang rust --outDir crates/logic-fsharp/src/

# 3. Build and test
cargo run -p app
```

## 🔗 Related Resources

### Project Documentation
- [CLAUDE_NOTES.md](CLAUDE_NOTES.md) - Original design document
- [docs/architecture.md](docs/architecture.md) - Architecture details
- [docs/fsharp-integration.md](docs/fsharp-integration.md) - F# integration guide

### External References
- [Bevy Documentation](https://bevyengine.org/learn/)
- [Fable F# to JavaScript Compiler](https://fable.io/)
- [fsrs - F# to Rust transpiler](https://github.com/raibid-labs/fsrs)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)

### GitHub Issues
- Issue #2: Add visual rendering
- Issue #3: Complete WASM build support
- Issue #4: Implement F# GameLogic.fs
- Issue #5: Integrate fsrs transpilation
- Issue #6: Add combat UI feedback
- Issue #7: Implement monster AI

## 🤝 Contributing

This is a reference implementation for Grimware. When contributing:

1. **Maintain F# ↔ Rust alignment**: Types must match exactly
2. **Test both native and WASM**: Both targets must work
3. **Update documentation**: Keep architecture.md current
4. **Follow repository conventions**: See root CLAUDE.md

---

**Remember**: This project demonstrates the F# → Rust → Bevy → WASM path. It's a proof-of-concept for future Grimware games, not a complete game itself.
