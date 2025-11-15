# Chapter 4: Rust Implementation

## What You'll Learn

In this chapter, you'll translate the F# domain model into idiomatic Rust code. You'll learn the mapping patterns, handle Rust's ownership system, and create a fully tested logic layer.

**Time Required**: 45 minutes

## From F# to Rust: Key Principles

When translating F# to Rust, remember:

1. **Ownership Instead of GC**: Rust manages memory through ownership
2. **Explicit Cloning**: Make copies explicit with `.clone()`
3. **Error Handling**: Use `Result<T, E>` for fallible operations
4. **Traits for Behavior**: Like F# interfaces but more powerful
5. **Lifetimes When Sharing**: Reference data without copying

## Project Structure

Our Rust implementation lives in a separate crate:

```
crates/logic-fsharp/
├── Cargo.toml         # Dependencies
├── src/
│   ├── lib.rs        # Main logic implementation
│   ├── combat.rs     # Combat system
│   ├── abilities.rs  # Ability system
│   └── ai.rs         # AI decision making
└── tests/
    └── integration.rs # Integration tests
```

## Step 1: Basic Type Translation

### Translating Records to Structs

**F# Version:**
```fsharp
type Stats = {
    Hp: int
    Attack: int
    Defense: int
    Speed: int
}

type Character = {
    Id: string
    Name: string
    CurrentHp: int
    Stats: Stats
    Level: int
}
```

**Rust Translation:**
```rust
// crates/logic-fsharp/src/lib.rs

/// Core character statistics
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub speed: i32,
}

/// Represents any combatant
#[derive(Debug, Clone)]
pub struct Character {
    pub id: String,
    pub name: String,
    pub current_hp: i32,
    pub stats: Stats,
    pub level: i32,
}
```

**Key Points:**
- `#[derive(...)]` adds common functionality automatically
- `pub` makes fields accessible outside the module
- Field names use `snake_case` convention
- `String` for owned strings, `&str` for borrowed

### Implementing Methods

Rust uses `impl` blocks for methods:

```rust
impl Stats {
    /// Create new stats with validation
    pub fn new(hp: i32, attack: i32, defense: i32, speed: i32) -> Result<Self, String> {
        if hp <= 0 {
            return Err("HP must be positive".to_string());
        }
        if attack < 0 || defense < 0 || speed <= 0 {
            return Err("Stats cannot be negative".to_string());
        }

        Ok(Stats {
            hp,
            attack,
            defense,
            speed,
        })
    }

    /// Calculate effective power level
    pub fn power_level(&self) -> i32 {
        self.hp + (self.attack * 2) + self.defense + self.speed
    }
}

impl Character {
    /// Smart constructor ensures valid character
    pub fn new(id: String, name: String, stats: Stats, level: i32) -> Result<Self, String> {
        if level < 1 {
            return Err("Level must be at least 1".to_string());
        }

        Ok(Character {
            id,
            name,
            current_hp: stats.hp,
            stats,
            level,
        })
    }

    /// Check if character is still alive
    pub fn is_alive(&self) -> bool {
        self.current_hp > 0
    }

    /// Get health percentage
    pub fn health_percentage(&self) -> f32 {
        (self.current_hp as f32 / self.stats.hp as f32) * 100.0
    }
}
```

## Step 2: Translating Discriminated Unions

### Simple Enums

**F# Version:**
```fsharp
type AbilityType =
    | Physical
    | Magical
    | Healing
    | Buff
    | Debuff
```

**Rust Translation:**
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AbilityType {
    Physical,
    Magical,
    Healing,
    Buff,
    Debuff,
}
```

### Enums with Data

**F# Version:**
```fsharp
type StatusEffect =
    | Poisoned of damagePerTurn: int
    | Burning of damagePerTurn: int * duration: int
    | Stunned of turnsRemaining: int
    | Shielded of damageReduction: int
```

**Rust Translation:**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum StatusEffect {
    Poisoned { damage_per_turn: i32 },
    Burning { damage_per_turn: i32, duration: i32 },
    Stunned { turns_remaining: i32 },
    Shielded { damage_reduction: i32 },
}

impl StatusEffect {
    /// Get remaining duration if applicable
    pub fn remaining_duration(&self) -> Option<i32> {
        match self {
            StatusEffect::Poisoned { .. } => None, // Permanent
            StatusEffect::Burning { duration, .. } => Some(*duration),
            StatusEffect::Stunned { turns_remaining } => Some(*turns_remaining),
            StatusEffect::Shielded { .. } => None, // Until broken
        }
    }

    /// Apply effect for one turn
    pub fn tick(&mut self) -> Option<i32> {
        match self {
            StatusEffect::Poisoned { damage_per_turn } => Some(*damage_per_turn),
            StatusEffect::Burning { damage_per_turn, duration } => {
                *duration -= 1;
                Some(*damage_per_turn)
            }
            StatusEffect::Stunned { turns_remaining } => {
                *turns_remaining -= 1;
                None
            }
            StatusEffect::Shielded { .. } => None,
        }
    }
}
```

## Step 3: Combat Logic Implementation

### Pure Functions

**F# Version:**
```fsharp
let calculateDamage attacker defender =
    let baseDamage = attacker.Stats.Attack - defender.Stats.Defense
    max 1 baseDamage
```

**Rust Translation:**
```rust
// crates/logic-fsharp/src/combat.rs

use crate::{Character, AbilityType};

/// Calculate damage with defense applied
pub fn calculate_damage(attacker: &Character, defender: &Character) -> i32 {
    let base_damage = attacker.stats.attack - defender.stats.defense;
    base_damage.max(1) // Minimum 1 damage
}

/// Calculate damage with ability modifiers
pub fn calculate_ability_damage(
    attacker: &Character,
    defender: &Character,
    ability_type: AbilityType,
    ability_power: i32,
) -> i32 {
    let base_damage = match ability_type {
        AbilityType::Physical => {
            attacker.stats.attack + ability_power - defender.stats.defense
        }
        AbilityType::Magical => {
            // Magic ignores defense
            (attacker.level * 2) + ability_power
        }
        _ => ability_power, // Healing, buffs, etc.
    };

    // Critical hit chance based on speed difference
    let crit_chance = (attacker.stats.speed - defender.stats.speed).max(0) as f32 / 100.0;
    let is_critical = crit_chance > 0.1; // Simplified - would be random in real game

    if is_critical {
        (base_damage as f32 * 1.5) as i32
    } else {
        base_damage.max(1)
    }
}
```

### State Transformations

```rust
/// Apply damage and return updated character
pub fn apply_damage(character: &Character, damage: i32) -> Character {
    Character {
        current_hp: (character.current_hp - damage).max(0),
        ..character.clone()
    }
}

/// Heal character up to max HP
pub fn apply_healing(character: &Character, amount: i32) -> Character {
    Character {
        current_hp: (character.current_hp + amount).min(character.stats.hp),
        ..character.clone()
    }
}

/// Execute a basic attack
pub fn execute_attack(attacker: &Character, defender: &Character) -> (Character, Vec<CombatEvent>) {
    let damage = calculate_damage(attacker, defender);
    let updated_defender = apply_damage(defender, damage);

    let mut events = vec![
        CombatEvent::DamageDealt {
            attacker: attacker.name.clone(),
            target: defender.name.clone(),
            amount: damage,
            damage_type: AbilityType::Physical,
        },
    ];

    if !updated_defender.is_alive() {
        events.push(CombatEvent::CharacterDefeated {
            name: defender.name.clone(),
        });
    }

    (updated_defender, events)
}
```

## Step 4: Event System

### Combat Events

```rust
#[derive(Debug, Clone)]
pub enum CombatEvent {
    // Damage events
    DamageDealt {
        attacker: String,
        target: String,
        amount: i32,
        damage_type: AbilityType,
    },
    DamageBlocked {
        target: String,
        amount: i32,
    },
    CriticalHit {
        attacker: String,
        multiplier: f32,
    },

    // Healing events
    HealingDone {
        healer: String,
        target: String,
        amount: i32,
    },

    // Status events
    StatusApplied {
        target: String,
        effect: StatusEffect,
    },
    StatusRemoved {
        target: String,
        effect: StatusEffect,
    },

    // Character events
    CharacterDefeated {
        name: String,
    },
    CharacterRevived {
        name: String,
        hp: i32,
    },

    // Turn events
    TurnStarted {
        character: String,
    },
    TurnEnded {
        character: String,
    },
}

impl CombatEvent {
    /// Convert event to human-readable string
    pub fn to_string(&self) -> String {
        match self {
            CombatEvent::DamageDealt { attacker, target, amount, damage_type } => {
                match damage_type {
                    AbilityType::Physical => {
                        format!("{} strikes {} for {} damage!", attacker, target, amount)
                    }
                    AbilityType::Magical => {
                        format!("{} blasts {} with magic for {} damage!", attacker, target, amount)
                    }
                    _ => format!("{} deals {} damage to {}!", attacker, amount, target),
                }
            }
            CombatEvent::CharacterDefeated { name } => {
                format!("{} has been defeated!", name)
            }
            CombatEvent::CriticalHit { attacker, multiplier } => {
                format!("{} scores a CRITICAL HIT! ({:.1}x damage)", attacker, multiplier)
            }
            CombatEvent::HealingDone { healer, target, amount } if healer == target => {
                format!("{} heals themselves for {} HP!", healer, amount)
            }
            CombatEvent::HealingDone { healer, target, amount } => {
                format!("{} heals {} for {} HP!", healer, target, amount)
            }
            _ => String::new(), // Handle other events as needed
        }
    }
}
```

## Step 5: Testing the Logic

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_creation() {
        let stats = Stats::new(100, 10, 5, 8).unwrap();
        let character = Character::new(
            "hero1".to_string(),
            "Hero".to_string(),
            stats,
            1,
        ).unwrap();

        assert_eq!(character.current_hp, 100);
        assert!(character.is_alive());
    }

    #[test]
    fn test_invalid_stats() {
        let result = Stats::new(-10, 5, 5, 5);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "HP must be positive");
    }

    #[test]
    fn test_damage_calculation() {
        let attacker = create_test_character("Attacker", 100, 15, 5);
        let defender = create_test_character("Defender", 100, 10, 8);

        let damage = calculate_damage(&attacker, &defender);
        assert_eq!(damage, 7); // 15 attack - 8 defense
    }

    #[test]
    fn test_minimum_damage() {
        let weak_attacker = create_test_character("Weak", 100, 3, 0);
        let strong_defender = create_test_character("Tank", 100, 0, 10);

        let damage = calculate_damage(&weak_attacker, &strong_defender);
        assert_eq!(damage, 1); // Minimum damage is 1
    }

    #[test]
    fn test_apply_damage() {
        let character = create_test_character("Hero", 100, 10, 5);
        let damaged = apply_damage(&character, 30);

        assert_eq!(damaged.current_hp, 70);
        assert!(damaged.is_alive());

        let defeated = apply_damage(&damaged, 80);
        assert_eq!(defeated.current_hp, 0);
        assert!(!defeated.is_alive());
    }

    #[test]
    fn test_healing_caps_at_max() {
        let mut character = create_test_character("Hero", 100, 10, 5);
        character.current_hp = 50;

        let healed = apply_healing(&character, 30);
        assert_eq!(healed.current_hp, 80);

        let overhealed = apply_healing(&healed, 50);
        assert_eq!(overhealed.current_hp, 100); // Capped at max
    }

    // Helper function for tests
    fn create_test_character(name: &str, hp: i32, attack: i32, defense: i32) -> Character {
        let stats = Stats::new(hp, attack, defense, 10).unwrap();
        Character::new(
            format!("{}_id", name.to_lowercase()),
            name.to_string(),
            stats,
            1,
        ).unwrap()
    }
}
```

### Integration Tests

```rust
// tests/integration.rs

use logic_fsharp::{*, combat::*};

#[test]
fn test_full_combat_round() {
    let hero = create_hero();
    let slime = create_slime();

    // Hero attacks slime
    let (damaged_slime, events) = execute_attack(&hero, &slime);
    assert!(damaged_slime.current_hp < slime.current_hp);
    assert!(!events.is_empty());

    // Slime counterattacks
    let (damaged_hero, events) = execute_attack(&slime, &hero);
    assert!(damaged_hero.current_hp < hero.current_hp);

    // Continue until one is defeated
    let mut current_hero = hero.clone();
    let mut current_slime = slime.clone();
    let mut round = 0;

    while current_hero.is_alive() && current_slime.is_alive() && round < 20 {
        let (new_slime, _) = execute_attack(&current_hero, &current_slime);
        current_slime = new_slime;

        if current_slime.is_alive() {
            let (new_hero, _) = execute_attack(&current_slime, &current_hero);
            current_hero = new_hero;
        }
        round += 1;
    }

    // Battle should end within reasonable rounds
    assert!(round < 20, "Combat took too long");
    assert!(
        !current_hero.is_alive() || !current_slime.is_alive(),
        "Someone should be defeated"
    );
}
```

## Step 6: Performance Considerations

### Avoiding Unnecessary Allocations

```rust
// BAD: Creates new String every time
pub fn get_display_name(character: &Character) -> String {
    format!("{} (Level {})", character.name, character.level)
}

// BETTER: Return borrowed string when possible
pub fn get_name(&self) -> &str {
    &self.name
}

// BEST: Use Cow for flexibility
use std::borrow::Cow;

pub fn get_display_name(&self) -> Cow<str> {
    if self.level == 1 {
        Cow::Borrowed(&self.name)
    } else {
        Cow::Owned(format!("{} (Level {})", self.name, self.level))
    }
}
```

### Using References vs Ownership

```rust
// Taking ownership (moves the value)
pub fn process_character(character: Character) -> Character {
    // character is consumed here
    character
}

// Borrowing (temporary access)
pub fn process_character_ref(character: &Character) -> i32 {
    character.stats.attack * 2
}

// Mutable borrowing (can modify)
pub fn level_up(character: &mut Character) {
    character.level += 1;
    character.stats.hp += 10;
    character.current_hp = character.stats.hp;
}
```

## Common Patterns and Best Practices

### Builder Pattern for Complex Types

```rust
pub struct CharacterBuilder {
    id: String,
    name: String,
    stats: Option<Stats>,
    level: i32,
}

impl CharacterBuilder {
    pub fn new(id: String, name: String) -> Self {
        CharacterBuilder {
            id,
            name,
            stats: None,
            level: 1,
        }
    }

    pub fn with_stats(mut self, stats: Stats) -> Self {
        self.stats = Some(stats);
        self
    }

    pub fn with_level(mut self, level: i32) -> Self {
        self.level = level;
        self
    }

    pub fn build(self) -> Result<Character, String> {
        let stats = self.stats.ok_or("Stats are required")?;
        Character::new(self.id, self.name, stats, self.level)
    }
}

// Usage
let hero = CharacterBuilder::new("hero1".to_string(), "Hero".to_string())
    .with_stats(Stats::new(100, 15, 10, 8)?)
    .with_level(5)
    .build()?;
```

### Type State Pattern

```rust
// Enforce state transitions at compile time
pub struct NotStarted;
pub struct InProgress;
pub struct Completed;

pub struct Battle<State> {
    participants: Vec<Character>,
    _state: std::marker::PhantomData<State>,
}

impl Battle<NotStarted> {
    pub fn new(participants: Vec<Character>) -> Self {
        Battle {
            participants,
            _state: std::marker::PhantomData,
        }
    }

    pub fn start(self) -> Battle<InProgress> {
        Battle {
            participants: self.participants,
            _state: std::marker::PhantomData,
        }
    }
}

impl Battle<InProgress> {
    pub fn complete(self) -> Battle<Completed> {
        Battle {
            participants: self.participants,
            _state: std::marker::PhantomData,
        }
    }
}

// Can only call start() on NotStarted battles
// Can only call complete() on InProgress battles
```

## Exercises

### Exercise 1: Add Ability System

Implement the ability system from the F# model:

```rust
pub struct Ability {
    pub id: String,
    pub name: String,
    pub ability_type: AbilityType,
    pub power: i32,
    pub mana_cost: i32,
    pub cooldown: i32,
}

// Implement:
// 1. execute_ability() function
// 2. Cooldown tracking
// 3. Mana consumption
```

### Exercise 2: Implement Status Effects

Add status effect handling:

```rust
impl Character {
    pub fn apply_status(&mut self, effect: StatusEffect) {
        // Add to character's status list
    }

    pub fn process_status_effects(&mut self) -> Vec<CombatEvent> {
        // Process all active effects for one turn
    }
}
```

### Exercise 3: Create Combat Simulator

Build a complete combat simulator:

```rust
pub struct CombatSimulator {
    hero: Character,
    enemy: Character,
    turn_count: i32,
    events: Vec<CombatEvent>,
}

impl CombatSimulator {
    pub fn simulate_battle(&mut self) -> CombatResult {
        // Run complete battle to completion
    }
}
```

## Summary

You've successfully translated F# domain models to Rust:

✅ Records → Structs with impl blocks
✅ Discriminated Unions → Enums with data
✅ Pattern Matching → match expressions
✅ Immutability → Ownership and borrowing
✅ Pure Functions → Functions with references
✅ Smart Constructors → Result types

## Next Steps

Now that we have a working logic layer in Rust, we'll integrate it with the Bevy game engine to create an interactive game.

[Next: Bevy Integration →](05-bevy-integration.md)

[← Previous: Domain Modeling](03-domain-modeling.md) | [Tutorial Index](README.md)