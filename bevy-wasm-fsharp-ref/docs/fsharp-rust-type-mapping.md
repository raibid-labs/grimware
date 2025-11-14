# F# ↔ Rust Type Mapping Reference

This document defines the **exact type alignment** between F# domain types and their Rust equivalents for the bevy-wasm-fsharp-ref project.

## Overview

The F# types in `fsharp/Domain.fs` serve as the canonical source of truth. Rust types in `crates/logic-fsharp/src/lib.rs` must mirror these exactly to ensure successful transpilation via fsrs.

## Naming Conventions

| F# | Rust | Example |
|----|------|---------|
| **Type names** | PascalCase → PascalCase | `Character` → `Character` |
| **Field names** | PascalCase → snake_case | `AttackerName` → `attacker_name` |
| **Function names** | camelCase → snake_case | `computeAttack` → `compute_attack` |

## Primitive Types

| F# | Rust | Notes |
|----|------|-------|
| `int` | `i32` | 32-bit signed integer |
| `string` | `String` | Heap-allocated UTF-8 string |
| `float` | `f64` | Double-precision float (if needed) |
| `bool` | `bool` | Boolean value |

## Domain Types

### Stats

**F# Definition** (`fsharp/Domain.fs`):
```fsharp
type Stats =
    { Hp: int
      Attack: int
      Defense: int }
```

**Rust Equivalent** (`crates/logic-fsharp/src/lib.rs`):
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}
```

**Field Mapping**:
- `Hp` → `hp`
- `Attack` → `attack`
- `Defense` → `defense`

---

### Character

**F# Definition**:
```fsharp
type Character =
    { Name: string
      Hp: int
      Stats: Stats }
```

**Rust Equivalent**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, bevy::prelude::Component)]
pub struct Character {
    pub name: String,
    pub hp: i32,
    pub stats: Stats,
}
```

**Field Mapping**:
- `Name` → `name`
- `Hp` → `hp`
- `Stats` → `stats`

**Notes**:
- Rust version includes `bevy::prelude::Component` derive for ECS integration
- Serde derives enable serialization for potential network play or save systems

---

### Ability

**F# Definition**:
```fsharp
type Ability =
    { Name: string
      Power: int }
```

**Rust Equivalent**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub power: i32,
}
```

**Field Mapping**:
- `Name` → `name`
- `Power` → `power`

---

### CombatEvent

**F# Definition**:
```fsharp
type CombatEvent =
    { AttackerName: string
      DefenderName: string
      Damage: int
      DefenderHpAfter: int }
```

**Rust Equivalent**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatEvent {
    pub attacker_name: String,
    pub defender_name: String,
    pub damage: i32,
    pub defender_hp_after: i32,
}
```

**Field Mapping**:
- `AttackerName` → `attacker_name`
- `DefenderName` → `defender_name`
- `Damage` → `damage`
- `DefenderHpAfter` → `defender_hp_after`

---

## Function Signatures

### computeAttack / compute_attack

**F# Signature** (`fsharp/GameLogic.fs`):
```fsharp
let computeAttack (attacker: Character) (defender: Character) (ability: Ability) : CombatEvent
```

**Rust Signature** (`crates/logic-fsharp/src/lib.rs`):
```rust
pub fn compute_attack(
    attacker: &Character,
    defender: &Character,
    ability: &Ability,
) -> CombatEvent
```

**Key Differences**:
- **Function name**: `computeAttack` → `compute_attack`
- **Parameter passing**: F# passes by value (immutable), Rust uses references (`&`) for efficiency
- **Return type**: Both return owned `CombatEvent`

**Implementation Alignment**:

F# implementation:
```fsharp
let computeAttack (attacker: Character) (defender: Character) (ability: Ability) : CombatEvent =
    let raw = attacker.Stats.Attack + ability.Power
    let dmg = max 1 (raw - defender.Stats.Defense)
    let hpAfter = defender.Hp - dmg
    { AttackerName = attacker.Name
      DefenderName = defender.Name
      Damage = dmg
      DefenderHpAfter = hpAfter }
```

Rust implementation:
```rust
pub fn compute_attack(
    attacker: &Character,
    defender: &Character,
    ability: &Ability,
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

---

## Constructor Functions

### F# Pattern

F# typically uses static factory functions or module-level functions:

```fsharp
// Not yet implemented in F#, but planned:
module Character =
    let newPlayer name =
        { Name = name
          Hp = 30
          Stats = { Hp = 30; Attack = 10; Defense = 2 } }

    let newMonster name =
        { Name = name
          Hp = 20
          Stats = { Hp = 20; Attack = 6; Defense = 1 } }
```

### Rust Pattern

Rust uses `impl` blocks with associated functions:

```rust
impl Character {
    pub fn new_player(name: &str) -> Self {
        Self {
            name: name.into(),
            hp: 30,
            stats: Stats { hp: 30, attack: 10, defense: 2 },
        }
    }

    pub fn new_monster(name: &str) -> Self {
        Self {
            name: name.into(),
            hp: 20,
            stats: Stats { hp: 20, attack: 6, defense: 1 },
        }
    }
}
```

---

## Common Patterns

### Immutability

**F# (default immutable)**:
```fsharp
let updatedCharacter = { character with Hp = newHp }
```

**Rust (explicit mutability)**:
```rust
// Option 1: Clone and modify
let mut updated = character.clone();
updated.hp = new_hp;

// Option 2: Struct update syntax
let updated = Character { hp: new_hp, ..character.clone() };
```

### Option/Result Types

**F# (Option, Result)**:
```fsharp
type FindResult = Character option
type ValidationResult = Result<Character, string>
```

**Rust (Option, Result)**:
```rust
type FindResult = Option<Character>;
type ValidationResult = Result<Character, String>;
```

These map directly!

---

## Testing Alignment

### Verify Type Alignment

To ensure F# and Rust types remain aligned:

1. **Manual verification**: Compare field names and types visually
2. **Round-trip serialization test** (future): Serialize F# → JSON → Rust
3. **fsrs output inspection** (when integrated): Check generated Rust code

### Example Test Pattern (Future)

F# test:
```fsharp
let player = { Name = "Hero"; Hp = 30; Stats = { Hp = 30; Attack = 10; Defense = 2 } }
let json = JsonSerializer.Serialize(player)
// json should match Rust's serde output
```

Rust test:
```rust
#[test]
fn test_combat_event_serialization() {
    let event = CombatEvent {
        attacker_name: "Hero".into(),
        defender_name: "Goblin".into(),
        damage: 14,
        defender_hp_after: 6,
    };
    let json = serde_json::to_string(&event).unwrap();
    // json should match F# output
}
```

---

## Future Extensions

When adding new types, follow this checklist:

- [ ] Define F# type in `fsharp/Domain.fs`
- [ ] Mirror in Rust at `crates/logic-fsharp/src/lib.rs`
- [ ] Verify field name conversion (PascalCase → snake_case)
- [ ] Verify primitive type mapping (int → i32, etc.)
- [ ] Add serde derives if serialization needed
- [ ] Add Component derive if used in Bevy ECS
- [ ] Document in this file
- [ ] Update `docs/fsharp-integration.md` if needed

---

## Quick Reference Table

| Concept | F# Example | Rust Example |
|---------|------------|--------------|
| Record type | `type Foo = { Bar: int }` | `struct Foo { bar: i32 }` |
| Field access | `foo.Bar` | `foo.bar` |
| Record update | `{ foo with Bar = 42 }` | `Foo { bar: 42, ..foo }` |
| Function | `let compute x = x + 1` | `fn compute(x: i32) -> i32 { x + 1 }` |
| Module | `module GameLogic` | `mod game_logic { ... }` |
| Public | `let compute` (default public) | `pub fn compute` |
| Private | `let private compute` | `fn compute` (default private) |

---

**Last Updated**: 2025-11-14
**Maintained By**: AI assistants working on issue #10 (F# game logic implementation)
