# Bevy WASM F# Reference Implementation

## Overview

The Bevy WASM F# reference demonstrates a novel game development workflow: **F# → Rust → Bevy → WASM**. This enables type-safe functional programming (F#) for game logic, Rust performance, Bevy's modern ECS architecture, and multi-platform deployment (native + browser).

## Key Concepts

### The F# → Rust → Bevy Path

**Workflow**:
```
F# Source Code (game logic, domain types)
    ↓
Fable Compiler + fsrs Transpiler
    ↓
Rust Code (generated crate)
    ↓
Bevy Application (ECS systems, rendering)
    ↓
Deployment: Native Binary OR WASM Bundle
```

**Benefits**:
- **F# Strengths**: Type safety, functional patterns, concise code
- **Rust Performance**: Zero-cost abstractions, memory safety
- **Bevy ECS**: Modern game architecture, modular design
- **WASM**: Browser deployment without plugins

### Current Architecture

**Note**: The `logic-fsharp` crate is currently hand-written Rust that mirrors expected fsrs output. This allows rapid iteration while the fsrs transpiler matures.

**Crates**:
- `crates/app` - Bevy application (main game loop, ECS systems)
- `crates/logic-fsharp` - Game logic (domain types, combat functions)

**F# Sources** (future transpilation targets):
- `fsharp/Domain.fs` - Domain types (Character, Stats, Ability, CombatEvent)
- `fsharp/GameLogic.fs` - Game logic functions (to be implemented)

**WASM Deployment**:
- `web/index.html` - WASM loader page
- `web/pkg/` - Generated WASM bundle (via wasm-bindgen)

## Quick Start

```bash
cd bevy-wasm-fsharp-ref

# Run natively
cargo run -p app
# OR: just run

# Build WASM
just build-wasm

# Serve WASM locally
just serve-wasm

# Build and serve in one command
just wasm
```

## Domain Model

### Character
Player or monster entity with stats and current HP.

**F# (Domain.fs)**:
```fsharp
type Character =
    { Name: string
      Hp: int
      Stats: Stats }
```

**Rust (lib.rs)**:
```rust
pub struct Character {
    pub name: String,
    pub hp: i32,
    pub stats: Stats,
}
```

### Stats
Character attributes (HP, attack, defense).

**F# (Domain.fs)**:
```fsharp
type Stats =
    { Hp: int
      Attack: int
      Defense: int }
```

**Rust (lib.rs)**:
```rust
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}
```

### Combat System

**Formula**:
```
damage = max(1, attacker.attack + ability.power - defender.defense)
defender.hp = defender.hp - damage
```

**Implementation** (crates/logic-fsharp/src/lib.rs):
```rust
pub fn compute_attack(
    attacker: &Character,
    defender: &Character,
    ability: &Ability
) -> CombatEvent {
    let raw = attacker.stats.attack + ability.power;
    let dmg = (raw - defender.stats.defense).max(1);
    let hp_after = defender.hp - dmg;

    CombatEvent {
        attacker_name: attacker.name.clone(),
        defender_name: defender.name.clone(),
        damage: dmg,
        defender_hp_after: hp_after,
    }
}
```

## F# to Rust Type Mapping

### Basic Types
| F# Type | Rust Type | Notes |
|---------|-----------|-------|
| `int` | `i32` | 32-bit signed integer |
| `string` | `String` | Owned heap string |
| `float` | `f32` or `f64` | Depends on precision needs |
| `bool` | `bool` | Boolean |

### Naming Conventions
| F# | Rust |
|----|------|
| PascalCase (fields) | snake_case |
| `Hp` | `hp` |
| `AttackerName` | `attacker_name` |

### Records vs Structs
**F# Record**:
```fsharp
type Stats = { Hp: int; Attack: int; Defense: int }
```

**Rust Struct**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}
```

**Key Differences**:
- Rust requires explicit derives (`Debug`, `Clone`, `Serialize`, etc.)
- Rust uses `pub` for public fields
- Rust needs semicolons between fields

### Functions
**F# Function**:
```fsharp
let computeAttack attacker defender ability =
    let raw = attacker.Stats.Attack + ability.Power
    let dmg = max 1 (raw - defender.Stats.Defense)
    let hpAfter = defender.Hp - dmg
    { AttackerName = attacker.Name
      DefenderName = defender.Name
      Damage = dmg
      DefenderHpAfter = hpAfter }
```

**Rust Function**:
```rust
pub fn compute_attack(
    attacker: &Character,
    defender: &Character,
    ability: &Ability
) -> CombatEvent {
    let raw = attacker.stats.attack + ability.power;
    let dmg = (raw - defender.stats.defense).max(1);
    let hp_after = defender.hp - dmg;

    CombatEvent {
        attacker_name: attacker.name.clone(),
        defender_name: defender.name.clone(),
        damage: dmg,
        defender_hp_after: hp_after,
    }
}
```

## Bevy ECS Integration

### Entity-Component-System Pattern

**Entities**: Unique identifiers
```rust
let player_entity = commands.spawn(...).id();
```

**Components**: Data attached to entities
```rust
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Monster;

commands.spawn((
    Player,
    logic::Character::new_player("Hero"),
    Transform::from_xyz(-100.0, 0.0, 0.0),
));
```

**Systems**: Functions that query and process entities
```rust
fn tick_combat(
    keys: Res<Input<KeyCode>>,
    mut players: Query<&mut Character, With<Player>>,
    mut monsters: Query<&mut Character, (With<Monster>, Without<Player>)>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // Player attacks monster
        let mut player = players.single_mut();
        let mut monster = monsters.single_mut();

        let ability = logic::Ability::basic_attack();
        let event = logic::compute_attack(&player, &monster, &ability);

        monster.hp = event.defender_hp_after;
    }
}
```

### Separation of Concerns

**Functional Core** (logic-fsharp crate):
- Pure functions, no side effects
- Domain logic and business rules
- Testable in isolation

**Imperative Shell** (app crate):
- ECS systems and queries
- Input handling, rendering
- Side effects (logging, state mutation)

This pattern keeps game logic portable and maintainable.

## WASM Deployment

### Building for WASM

**Prerequisites**:
```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen
cargo install wasm-bindgen-cli
```

**Build Command**:
```bash
# With justfile
just build-wasm

# Manual
cargo build -p app --target wasm32-unknown-unknown --release
wasm-bindgen --out-dir web/pkg --target web \
    target/wasm32-unknown-unknown/release/app.wasm
```

### Serving Locally

```bash
# With justfile
just serve-wasm

# Manual
cd web
python3 -m http.server 8000

# Open: http://localhost:8000
```

### WASM Considerations

**Limitations**:
- No file system access (use embedded assets)
- Some Bevy plugins unavailable
- Audio/input handling differences
- Performance may vary vs native

**Best Practices**:
- Test WASM builds regularly
- Check browser console for errors
- Use web-friendly asset formats
- Handle async operations carefully

## Future: fsrs Integration

### Planned Workflow

1. **Write F# game logic**:
   ```bash
   vim fsharp/GameLogic.fs
   ```

2. **Transpile to Rust**:
   ```bash
   fable fsharp/GameLogic.fs --lang rust --outDir crates/logic-fsharp/src/
   ```

3. **Build and test**:
   ```bash
   cargo run -p app
   ```

### Benefits of F# for Game Logic

**Type Safety**:
```fsharp
// Discriminated unions for game states
type GameState =
    | Playing
    | Paused
    | GameOver of winner: string

// Pattern matching exhaustiveness checked by compiler
let handleState state =
    match state with
    | Playing -> (* ... *)
    | Paused -> (* ... *)
    | GameOver winner -> (* ... *)
```

**Concise Code**:
```fsharp
// F# - 4 lines
let damageFormula attacker defender ability =
    max 1 (attacker.Stats.Attack + ability.Power - defender.Stats.Defense)

// Rust - 6 lines
pub fn damage_formula(attacker: &Character, defender: &Character, ability: &Ability) -> i32 {
    let raw = attacker.stats.attack + ability.power;
    (raw - defender.stats.defense).max(1)
}
```

**Immutability by Default**:
```fsharp
// F# record update syntax
let damagedCharacter = { character with Hp = character.Hp - damage }

// Rust clone and modify
let mut damaged = character.clone();
damaged.hp = character.hp - damage;
```

## Development Workflow

### Iterative Development

1. **Design in F#** - Write game logic with F# type safety
2. **Mirror in Rust** - Maintain hand-written Rust version
3. **Integrate with Bevy** - Connect logic to ECS systems
4. **Test Native** - Fast iteration with `cargo run`
5. **Test WASM** - Verify browser compatibility
6. **Commit** - Both F# and Rust versions

### Using justfile Commands

```bash
# Development cycle
just run                # Test native
just watch             # Auto-reload on changes
just build-wasm        # Build WASM
just serve-wasm        # Test in browser

# Quality checks
just check-all         # Format, lint, test, build

# Information
just info              # Show project details
just todos             # Show TODO items
```

## Performance Benefits

### Native Performance
- Rust zero-cost abstractions
- No runtime overhead
- Direct hardware access
- Optimal for desktop gaming

### WASM Performance
- Near-native speed in browser
- No JavaScript overhead for game logic
- JIT compilation by browser
- Suitable for web deployment

### F# Transpilation Benefits
- Smaller surface area for bugs (type safety)
- Easier to reason about (functional patterns)
- Faster development iteration
- Maintainable game rules

## Best Practices

### 1. Type Alignment
Always ensure F# and Rust types match exactly:

**F# (Domain.fs)**:
```fsharp
type Ability = { Name: string; Power: int }
```

**Rust (lib.rs)**:
```rust
pub struct Ability {
    pub name: String,
    pub power: i32,
}
```

### 2. Pure Logic Layer
Keep game logic pure (no side effects):

```rust
// ✅ Good - pure function
pub fn compute_attack(attacker: &Character, defender: &Character, ability: &Ability) -> CombatEvent {
    // No state mutation, just computation
}

// ❌ Bad - side effects in logic
pub fn compute_attack_and_log(attacker: &Character, defender: &Character, ability: &Ability) -> CombatEvent {
    println!("Attack!"); // Side effect - don't do this in logic crate
}
```

### 3. Test Both Targets
Always test native and WASM:

```bash
# Test native
cargo run -p app
cargo test

# Test WASM
just build-wasm
just serve-wasm
# Open browser, check console
```

### 4. Document Assumptions
Use comments to explain F# ↔ Rust mappings:

```rust
/// Mirrors F# type: Domain.Character
/// F# fields use PascalCase, Rust uses snake_case
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    /// F#: Name
    pub name: String,
    /// F#: Hp
    pub hp: i32,
    /// F#: Stats
    pub stats: Stats,
}
```

## Troubleshooting

### WASM Build Fails

**Error**: `wasm32-unknown-unknown target not installed`
```bash
rustup target add wasm32-unknown-unknown
```

**Error**: `wasm-bindgen not found`
```bash
cargo install wasm-bindgen-cli
```

### Type Mismatch Errors

Check naming conventions:
- F# uses PascalCase: `Hp`, `AttackerName`
- Rust uses snake_case: `hp`, `attacker_name`

### WASM Not Loading

1. Check browser console for errors
2. Verify `web/pkg/` directory exists
3. Ensure serving from correct directory
4. Check MIME types are correct (`.wasm` = `application/wasm`)

## Technical Requirements

- **Rust**: Latest stable (1.70+)
- **Bevy**: 0.16+
- **F# SDK**: .NET 8.0+ (for future fsrs)
- **wasm-bindgen-cli**: Latest
- **just**: Latest (optional, for convenience)

## Further Reading

- [Full Project README](../bevy-wasm-fsharp-ref/README.md)
- [AI Assistant Guide](../bevy-wasm-fsharp-ref/CLAUDE.md)
- [Architecture Details](../bevy-wasm-fsharp-ref/docs/architecture.md)
- [F# Integration](../bevy-wasm-fsharp-ref/docs/fsharp-integration.md)
- [Bevy WASM Guide](https://bevyengine.org/learn/book/getting-started/setup/#webassembly)
- [Fable Documentation](https://fable.io/)
- [Rust WASM Book](https://rustwasm.github.io/docs/book/)
