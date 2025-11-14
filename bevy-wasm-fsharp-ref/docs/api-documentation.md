# API Documentation - bevy-wasm-fsharp-ref

## Overview

Comprehensive inline documentation and API reference has been added to the `logic-fsharp` crate, providing:

- **Module-level documentation** explaining the crate's purpose and architecture
- **Detailed type documentation** for all public items
- **F# equivalents** showing how Rust types map to F# source
- **Usage examples** demonstrating practical applications
- **Doc tests** validating code examples

## Documentation Structure

### Crate-Level Documentation

The crate root (`lib.rs`) includes comprehensive module documentation covering:

1. **Purpose**: Explanation of the F# → Rust → Bevy → WASM workflow
2. **F# Integration**: How types mirror F# definitions
3. **Architecture**: Visual diagram of the transpilation pipeline
4. **Core Concepts**: Combat system overview
5. **Bevy Integration**: ECS usage patterns
6. **F# Type Mappings**: Translation table for common types
7. **Design Principles**: Functional core philosophy

### Type Documentation

Each public type is fully documented with:

#### Stats
- **Purpose**: Character combat statistics (HP, attack, defense)
- **F# Equivalent**: Complete F# type definition
- **Field Documentation**: Purpose of each field
- **Examples**: Creating player and monster stats

#### Character
- **Purpose**: Core game entity (player or monster)
- **F# Equivalent**: F# record type
- **Design Notes**: HP mechanics, defeat conditions, ECS integration
- **Examples**: Character creation, Bevy spawning
- **Methods**:
  - `new_player(name)`: Creates player with balanced stats
  - `new_monster(name)`: Creates weaker monster

#### Ability
- **Purpose**: Combat actions with power values
- **F# Equivalent**: F# record type
- **Design Notes**: Factory pattern usage
- **Examples**: Built-in and custom abilities
- **Methods**:
  - `basic_attack()`: Standard attack ability

#### CombatEvent
- **Purpose**: Immutable record of combat outcome
- **F# Equivalent**: F# record type
- **Design Notes**: Event-driven design, HP update pattern
- **Examples**: Event logging and UI display

### Function Documentation

#### compute_attack

Comprehensive documentation including:

- **Purpose**: Core combat resolution function
- **Combat Formula**: Mathematical breakdown with example
- **F# Equivalent**: Complete F# implementation
- **Arguments**: Parameter descriptions
- **Returns**: Result structure
- **Examples**:
  1. Basic attack scenario with damage calculation
  2. Minimum damage edge case (defense > attack)
  3. Defeating an enemy (combat loop)
- **Design Notes**: Purity, immutability, minimum damage rule

## Doc Tests

All examples are executable doc tests that verify:

- Character creation works correctly
- Combat damage calculations are accurate
- F# type mappings are correct
- Edge cases (minimum damage, negative HP) behave as expected

### Test Results

```
✓ 11 doc tests passed
✓ 33 unit tests passed
✓ 8 integration tests passed
✓ 0 failures
✓ 2 ignored (Bevy integration examples marked as `rust,ignore`)
```

## Generating Documentation

### Native Target (Recommended)

```bash
cargo doc --no-deps -p bevy-wasm-fsharp-ref-logic --target aarch64-apple-darwin
```

Output: `/Users/beengud/.cargo/target/aarch64-apple-darwin/doc/bevy_wasm_fsharp_ref_logic/index.html`

### Running Doc Tests

```bash
cargo test --doc -p bevy-wasm-fsharp-ref-logic
```

### Running All Tests

```bash
cargo test -p bevy-wasm-fsharp-ref-logic
```

## Documentation Features

### 1. F# Type Alignment

Every Rust type includes its F# equivalent:

```rust
/// # F# Equivalent
///
/// ```fsharp
/// type Stats =
///     { Hp: int
///       Attack: int
///       Defense: int }
/// ```
```

This helps developers:
- Understand the transpilation mapping
- Verify type consistency
- Learn both languages simultaneously

### 2. Practical Examples

Each type includes runnable examples:

```rust
/// # Examples
///
/// ```
/// use bevy_wasm_fsharp_ref_logic::Character;
///
/// let player = Character::new_player("Hero");
/// assert_eq!(player.hp, 30);
/// ```
```

### 3. Bevy Integration Patterns

Bevy-specific usage is documented:

```rust
/// # Bevy Integration
///
/// ```rust,ignore
/// fn spawn_player(mut commands: Commands) {
///     commands.spawn((
///         Player,
///         Character::new_player("Hero"),
///     ));
/// }
/// ```
```

### 4. Design Philosophy

Documentation explains the "why" not just the "what":

```rust
/// # Design Notes
///
/// - This function is **pure** - it has no side effects
/// - The defender's HP is **not modified** - the caller must apply the new HP
/// - The minimum damage rule prevents invulnerable scenarios
```

## Documentation Metrics

- **Crate-level docs**: 120 lines (purpose, architecture, integration)
- **Type docs**: ~400 lines (Stats, Character, Ability, CombatEvent)
- **Function docs**: ~120 lines (compute_attack with examples)
- **Field docs**: ~25 lines (inline field documentation)
- **Total documentation**: ~665 lines
- **Code-to-docs ratio**: ~1.1:1 (665 doc lines : 575 code/test lines)

## Coverage

### Public Items Documented ✓

- [x] Module (crate root)
- [x] Stats struct
- [x] Stats fields (hp, attack, defense)
- [x] Character struct
- [x] Character fields (name, hp, stats)
- [x] Character::new_player
- [x] Character::new_monster
- [x] Ability struct
- [x] Ability fields (name, power)
- [x] Ability::basic_attack
- [x] CombatEvent struct
- [x] CombatEvent fields (attacker_name, defender_name, damage, defender_hp_after)
- [x] compute_attack function

### Documentation Quality ✓

- [x] Purpose clearly stated
- [x] F# equivalents provided
- [x] Usage examples included
- [x] Doc tests pass
- [x] Design notes explain rationale
- [x] Parameters documented
- [x] Return values documented
- [x] Edge cases covered
- [x] Bevy integration shown

## Future Enhancements

Potential documentation improvements:

1. **Performance Notes**: Add complexity analysis for combat calculations
2. **Migration Guide**: Document transition from F# to Rust
3. **Error Handling**: Document potential error states (future work)
4. **Advanced Examples**: Multi-character combat scenarios
5. **Video Tutorial**: Screen recording of doc generation and usage

## Related Issues

- **Issue #21**: Add comprehensive inline documentation (COMPLETED ✓)
- Issue #4: Implement F# GameLogic.fs (future - will update docs)
- Issue #5: Integrate fsrs transpilation (future - will add generation notes)

## Conclusion

The `logic-fsharp` crate now has **production-quality documentation** that:

1. Explains the F# → Rust integration clearly
2. Provides runnable examples for all public APIs
3. Documents design decisions and trade-offs
4. Passes all doc tests without warnings
5. Follows Rust documentation best practices
6. Serves as a reference for future Grimware projects

Documentation can be viewed at:
```
/Users/beengud/.cargo/target/aarch64-apple-darwin/doc/bevy_wasm_fsharp_ref_logic/index.html
```

Or generated with:
```bash
cargo doc --no-deps -p bevy-wasm-fsharp-ref-logic --target aarch64-apple-darwin --open
```
