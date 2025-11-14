//! # Bevy WASM F# Logic Crate
//!
//! This crate contains game logic types and functions for the Bevy WASM F# reference implementation.
//! It serves as the domain model and game rules layer in the F# → Rust → Bevy → WASM workflow.
//!
//! ## Purpose
//!
//! This crate demonstrates how F# functional programming patterns can be transpiled to Rust
//! and integrated with the Bevy game engine. Currently, this is a hand-written Rust implementation
//! that mirrors F# source code. In the future, it will be replaced by code generated via the
//! [fsrs](https://github.com/raibid-labs/fsrs) transpiler.
//!
//! ## F# Integration
//!
//! Types and functions in this crate mirror F# definitions in:
//! - `fsharp/Domain.fs` - Domain types (Stats, Character, Ability, CombatEvent)
//! - `fsharp/GameLogic.fs` - Game logic functions (compute_attack)
//!
//! This dual implementation enables:
//! 1. **Type safety**: F# and Rust both enforce strong typing
//! 2. **Functional patterns**: Pure functions with no side effects
//! 3. **Cross-compilation**: Same logic works in native and WASM builds
//!
//! ## Architecture
//!
//! ```text
//! F# Source (fsharp/)
//!     ↓
//! Fable + fsrs transpilation
//!     ↓
//! Rust Code (this crate)
//!     ↓
//! Bevy ECS Integration (app crate)
//!     ↓
//!     ├─→ Native Binary (desktop)
//!     └─→ WASM Bundle (web)
//! ```
//!
//! ## Core Concepts
//!
//! ### Combat System
//!
//! The combat system is built around simple, deterministic rules:
//!
//! - Each [`Character`] has [`Stats`] (hp, attack, defense)
//! - Characters use [`Ability`] actions to attack
//! - Combat is resolved via [`compute_attack`], producing a [`CombatEvent`]
//! - Damage formula: `max(1, attacker.attack + ability.power - defender.defense)`
//!
//! ### Example Usage
//!
//! ```
//! use bevy_wasm_fsharp_ref_logic::*;
//!
//! // Create characters
//! let player = Character::new_player("Hero");
//! let monster = Character::new_monster("Slime");
//!
//! // Execute an attack
//! let ability = Ability::basic_attack();
//! let event = compute_attack(&player, &monster, &ability);
//!
//! println!("{} attacked {} for {} damage!",
//!          event.attacker_name,
//!          event.defender_name,
//!          event.damage);
//! ```
//!
//! ## Bevy Integration
//!
//! Types in this crate are designed to work seamlessly with Bevy's ECS:
//!
//! - [`Character`] derives `Component` for use in Bevy entities
//! - All types derive `Serialize` and `Deserialize` for WASM bindings
//! - Pure functions (like [`compute_attack`]) are called from Bevy systems
//!
//! Example Bevy system:
//!
//! ```rust,ignore
//! fn combat_system(
//!     mut players: Query<&mut Character, With<Player>>,
//!     mut monsters: Query<&mut Character, With<Monster>>,
//! ) {
//!     let player = players.single();
//!     let mut monster = monsters.single_mut();
//!
//!     let ability = Ability::basic_attack();
//!     let event = compute_attack(&player, &monster, &ability);
//!
//!     monster.hp = event.defender_hp_after;
//! }
//! ```
//!
//! ## F# Type Mappings
//!
//! | F# Type | Rust Type | Notes |
//! |---------|-----------|-------|
//! | `int` | `i32` | Signed 32-bit integer |
//! | `string` | `String` | Heap-allocated string |
//! | Record `{ Field: T }` | Struct `{ field: T }` | F# uses PascalCase, Rust uses snake_case |
//! | Record with members | Struct with impl block | Methods become associated functions |
//!
//! ## Design Principles
//!
//! 1. **Functional Core**: All logic functions are pure (no side effects)
//! 2. **Immutability**: Functions return new values rather than mutating
//! 3. **Type Safety**: Leverage Rust's type system to prevent invalid states
//! 4. **Simplicity**: Keep game rules clear and testable
//!
//! ## Future Enhancements
//!
//! Planned features include:
//! - Automatic fsrs transpilation from F# source
//! - More complex combat abilities (multi-target, status effects)
//! - Character progression system
//! - Monster AI behaviors
//!
//! See the [GitHub issues](https://github.com/raibid-labs/grimware) for active development work.

use serde::{Deserialize, Serialize};

/// Character combat statistics.
///
/// Stats define a character's combat capabilities: maximum health, offensive power,
/// and defensive resilience. These values are used in damage calculations via [`compute_attack`].
///
/// # F# Equivalent
///
/// ```fsharp
/// type Stats =
///     { Hp: int
///       Attack: int
///       Defense: int }
/// ```
///
/// # Fields
///
/// - `hp`: Maximum hit points (how much damage can be taken before defeat)
/// - `attack`: Attack power (added to damage calculations)
/// - `defense`: Damage reduction (subtracted from incoming damage)
///
/// # Examples
///
/// ```
/// use bevy_wasm_fsharp_ref_logic::Stats;
///
/// // Typical player stats
/// let player_stats = Stats {
///     hp: 30,
///     attack: 10,
///     defense: 2,
/// };
///
/// // Weaker monster stats
/// let monster_stats = Stats {
///     hp: 20,
///     attack: 6,
///     defense: 1,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    /// Maximum hit points. When current HP reaches 0, the character is defeated.
    pub hp: i32,

    /// Attack power. Added to ability power when calculating damage.
    pub attack: i32,

    /// Damage reduction. Subtracted from incoming damage (minimum 1 damage always applies).
    pub defense: i32,
}

/// A game character (player or monster).
///
/// Characters are the core entities in the combat system. Each character has a name,
/// current health, and base statistics. Characters can be either player-controlled
/// or AI-controlled monsters.
///
/// # F# Equivalent
///
/// ```fsharp
/// type Character =
///     { Name: string
///       Hp: int
///       Stats: Stats }
/// ```
///
/// # Design Notes
///
/// - Current HP (`hp`) can differ from max HP (`stats.hp`) after taking damage
/// - Characters are defeated when `hp <= 0`
/// - The `Stats` field contains base values that don't change during combat
/// - This type derives `Component` for use in Bevy's ECS
///
/// # Examples
///
/// ```
/// use bevy_wasm_fsharp_ref_logic::Character;
///
/// // Create a player character
/// let player = Character::new_player("Hero");
/// assert_eq!(player.name, "Hero");
/// assert_eq!(player.hp, 30);
/// assert_eq!(player.stats.attack, 10);
///
/// // Create a monster
/// let monster = Character::new_monster("Slime");
/// assert_eq!(monster.name, "Slime");
/// assert_eq!(monster.hp, 20);
/// assert_eq!(monster.stats.attack, 6);
/// ```
///
/// # Bevy Integration
///
/// ```rust,ignore
/// use bevy::prelude::*;
/// use bevy_wasm_fsharp_ref_logic::Character;
///
/// #[derive(Component)]
/// struct Player;
///
/// fn spawn_player(mut commands: Commands) {
///     commands.spawn((
///         Player,
///         Character::new_player("Hero"),
///         Transform::from_xyz(-100.0, 0.0, 0.0),
///     ));
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, bevy::prelude::Component)]
pub struct Character {
    /// Display name of the character.
    pub name: String,

    /// Current hit points. When this reaches 0 or below, the character is defeated.
    pub hp: i32,

    /// Base combat statistics (max HP, attack, defense).
    pub stats: Stats,
}

impl Character {
    /// Creates a new player character with default stats.
    ///
    /// Player characters have balanced stats suitable for the protagonist:
    /// - 30 HP (both current and max)
    /// - 10 Attack
    /// - 2 Defense
    ///
    /// # Arguments
    ///
    /// * `name` - The display name for the player character
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_wasm_fsharp_ref_logic::Character;
    ///
    /// let player = Character::new_player("Hero");
    /// assert_eq!(player.name, "Hero");
    /// assert_eq!(player.hp, 30);
    /// assert_eq!(player.stats.hp, 30);
    /// assert_eq!(player.stats.attack, 10);
    /// assert_eq!(player.stats.defense, 2);
    /// ```
    pub fn new_player(name: &str) -> Self {
        Self {
            name: name.into(),
            hp: 30,
            stats: Stats {
                hp: 30,
                attack: 10,
                defense: 2,
            },
        }
    }

    /// Creates a new monster character with default stats.
    ///
    /// Monsters are weaker than players but can be spawned in greater numbers:
    /// - 20 HP (both current and max)
    /// - 6 Attack
    /// - 1 Defense
    ///
    /// # Arguments
    ///
    /// * `name` - The display name for the monster (e.g., "Slime", "Goblin")
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_wasm_fsharp_ref_logic::Character;
    ///
    /// let monster = Character::new_monster("Slime");
    /// assert_eq!(monster.name, "Slime");
    /// assert_eq!(monster.hp, 20);
    /// assert_eq!(monster.stats.hp, 20);
    /// assert_eq!(monster.stats.attack, 6);
    /// assert_eq!(monster.stats.defense, 1);
    /// ```
    pub fn new_monster(name: &str) -> Self {
        Self {
            name: name.into(),
            hp: 20,
            stats: Stats {
                hp: 20,
                attack: 6,
                defense: 1,
            },
        }
    }
}

/// A combat ability or attack.
///
/// Abilities represent actions that characters can perform in combat. Each ability
/// has a name (for display purposes) and a power value (used in damage calculations).
///
/// # F# Equivalent
///
/// ```fsharp
/// type Ability =
///     { Name: string
///       Power: int }
/// ```
///
/// # Design Notes
///
/// - Abilities are typically created via factory functions like [`Ability::basic_attack`]
/// - Power is added to the attacker's attack stat when calculating damage
/// - Future enhancements may add cooldowns, MP costs, or status effects
///
/// # Examples
///
/// ```
/// use bevy_wasm_fsharp_ref_logic::Ability;
///
/// // Use the built-in basic attack
/// let attack = Ability::basic_attack();
/// assert_eq!(attack.name, "Basic Attack");
/// assert_eq!(attack.power, 5);
///
/// // Create a custom ability
/// let fireball = Ability {
///     name: "Fireball".to_string(),
///     power: 15,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    /// Display name of the ability.
    pub name: String,

    /// Base power of the ability. Added to the attacker's attack stat.
    pub power: i32,
}

impl Ability {
    /// Creates the standard basic attack ability.
    ///
    /// The basic attack is available to all characters and has no special requirements.
    /// It provides a moderate power boost (5) to the attacker's base attack stat.
    ///
    /// # F# Equivalent
    ///
    /// ```fsharp
    /// let basicAttack =
    ///     { Name = "Basic Attack"
    ///       Power = 5 }
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use bevy_wasm_fsharp_ref_logic::Ability;
    ///
    /// let ability = Ability::basic_attack();
    /// assert_eq!(ability.name, "Basic Attack");
    /// assert_eq!(ability.power, 5);
    /// ```
    pub fn basic_attack() -> Self {
        Self {
            name: "Basic Attack".into(),
            power: 5,
        }
    }
}

/// The result of a combat action.
///
/// A `CombatEvent` records what happened when one character attacked another.
/// It contains the names of both participants, the damage dealt, and the defender's
/// remaining HP after the attack.
///
/// # F# Equivalent
///
/// ```fsharp
/// type CombatEvent =
///     { AttackerName: string
///       DefenderName: string
///       Damage: int
///       DefenderHpAfter: int }
/// ```
///
/// # Design Notes
///
/// - Events are immutable records of what happened
/// - The defender's HP must be manually updated based on `defender_hp_after`
/// - Events can be logged, displayed in UI, or used for AI decision-making
///
/// # Examples
///
/// ```
/// use bevy_wasm_fsharp_ref_logic::*;
///
/// let player = Character::new_player("Hero");
/// let monster = Character::new_monster("Slime");
/// let ability = Ability::basic_attack();
///
/// let event = compute_attack(&player, &monster, &ability);
///
/// println!("{} attacked {} for {} damage! ({} HP remaining)",
///          event.attacker_name,
///          event.defender_name,
///          event.damage,
///          event.defender_hp_after);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatEvent {
    /// Name of the character who performed the attack.
    pub attacker_name: String,

    /// Name of the character who received the attack.
    pub defender_name: String,

    /// Amount of damage dealt (after defense calculation).
    pub damage: i32,

    /// Defender's remaining HP after taking damage.
    /// If this is <= 0, the defender is defeated.
    pub defender_hp_after: i32,
}

/// Computes the result of an attack between two characters.
///
/// This is the core combat resolution function. It takes an attacker, defender, and ability,
/// then calculates damage and produces a [`CombatEvent`] describing the outcome.
///
/// # Combat Formula
///
/// ```text
/// raw_damage = attacker.stats.attack + ability.power
/// actual_damage = max(1, raw_damage - defender.stats.defense)
/// new_hp = defender.hp - actual_damage
/// ```
///
/// **Note**: Damage is always at least 1, even if defense exceeds attack power.
///
/// # F# Equivalent
///
/// ```fsharp
/// let computeAttack (attacker: Character) (defender: Character) (ability: Ability) : CombatEvent =
///     let raw = attacker.Stats.Attack + ability.Power
///     let dmg = max 1 (raw - defender.Stats.Defense)
///     let hpAfter = defender.Hp - dmg
///     { AttackerName = attacker.Name
///       DefenderName = defender.Name
///       Damage = dmg
///       DefenderHpAfter = hpAfter }
/// ```
///
/// # Arguments
///
/// * `attacker` - The character performing the attack
/// * `defender` - The character receiving the attack
/// * `ability` - The ability being used
///
/// # Returns
///
/// A [`CombatEvent`] containing:
/// - Attacker and defender names
/// - Damage dealt
/// - Defender's HP after the attack
///
/// # Examples
///
/// ## Basic Attack Example
///
/// ```
/// use bevy_wasm_fsharp_ref_logic::*;
///
/// let player = Character::new_player("Hero");
/// let monster = Character::new_monster("Slime");
/// let ability = Ability::basic_attack();
///
/// let event = compute_attack(&player, &monster, &ability);
///
/// // Player (attack=10) + BasicAttack (power=5) - Monster (defense=1) = 14 damage
/// assert_eq!(event.damage, 14);
/// assert_eq!(event.attacker_name, "Hero");
/// assert_eq!(event.defender_name, "Slime");
/// assert_eq!(event.defender_hp_after, 20 - 14); // Monster starts with 20 HP
/// ```
///
/// ## Minimum Damage Example
///
/// ```
/// use bevy_wasm_fsharp_ref_logic::*;
///
/// // Create a weak attacker
/// let weak = Character {
///     name: "Weak Fighter".to_string(),
///     hp: 10,
///     stats: Stats { hp: 10, attack: 1, defense: 0 },
/// };
///
/// // Create a highly defensive target
/// let tank = Character {
///     name: "Tank".to_string(),
///     hp: 50,
///     stats: Stats { hp: 50, attack: 5, defense: 20 },
/// };
///
/// let ability = Ability::basic_attack();
/// let event = compute_attack(&weak, &tank, &ability);
///
/// // Even though attack (1) + power (5) - defense (20) = -14,
/// // damage is always at least 1
/// assert_eq!(event.damage, 1);
/// assert_eq!(event.defender_hp_after, 49);
/// ```
///
/// ## Defeating an Enemy
///
/// ```
/// use bevy_wasm_fsharp_ref_logic::*;
///
/// let player = Character::new_player("Hero");
/// let mut monster = Character::new_monster("Slime");
/// let ability = Ability::basic_attack();
///
/// // Attack until defeated
/// while monster.hp > 0 {
///     let event = compute_attack(&player, &monster, &ability);
///     monster.hp = event.defender_hp_after;
///
///     println!("Dealt {} damage! Monster HP: {}", event.damage, monster.hp);
/// }
///
/// assert!(monster.hp <= 0);
/// println!("Monster defeated!");
/// ```
///
/// # Design Notes
///
/// - This function is **pure** - it has no side effects
/// - The defender's HP is **not modified** - the caller must apply the new HP
/// - Events can be logged, animated, or used for AI without affecting game state
/// - The minimum damage rule prevents invulnerable scenarios
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

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Character::new_player Tests ====================

    #[test]
    fn test_new_player_creates_correct_stats() {
        let player = Character::new_player("Hero");

        assert_eq!(player.name, "Hero");
        assert_eq!(player.hp, 30);
        assert_eq!(player.stats.hp, 30);
        assert_eq!(player.stats.attack, 10);
        assert_eq!(player.stats.defense, 2);
    }

    #[test]
    fn test_new_player_with_empty_name() {
        let player = Character::new_player("");

        assert_eq!(player.name, "");
        assert_eq!(player.hp, 30);
    }

    #[test]
    fn test_new_player_with_unicode_name() {
        let player = Character::new_player("英雄");

        assert_eq!(player.name, "英雄");
        assert_eq!(player.hp, 30);
    }

    #[test]
    fn test_new_player_hp_matches_stats_hp() {
        let player = Character::new_player("Test");

        // HP and stats.hp should be identical for a new player
        assert_eq!(player.hp, player.stats.hp);
    }

    // ==================== Character::new_monster Tests ====================

    #[test]
    fn test_new_monster_creates_correct_stats() {
        let monster = Character::new_monster("Slime");

        assert_eq!(monster.name, "Slime");
        assert_eq!(monster.hp, 20);
        assert_eq!(monster.stats.hp, 20);
        assert_eq!(monster.stats.attack, 6);
        assert_eq!(monster.stats.defense, 1);
    }

    #[test]
    fn test_new_monster_with_empty_name() {
        let monster = Character::new_monster("");

        assert_eq!(monster.name, "");
        assert_eq!(monster.hp, 20);
    }

    #[test]
    fn test_new_monster_hp_matches_stats_hp() {
        let monster = Character::new_monster("Test");

        // HP and stats.hp should be identical for a new monster
        assert_eq!(monster.hp, monster.stats.hp);
    }

    #[test]
    fn test_player_vs_monster_stat_differences() {
        let player = Character::new_player("Hero");
        let monster = Character::new_monster("Slime");

        // Player should have more HP
        assert!(player.hp > monster.hp);
        assert!(player.stats.hp > monster.stats.hp);

        // Player should have higher attack
        assert!(player.stats.attack > monster.stats.attack);

        // Player should have higher defense
        assert!(player.stats.defense > monster.stats.defense);
    }

    // ==================== Ability::basic_attack Tests ====================

    #[test]
    fn test_basic_attack_creates_correct_ability() {
        let ability = Ability::basic_attack();

        assert_eq!(ability.name, "Basic Attack");
        assert_eq!(ability.power, 5);
    }

    #[test]
    fn test_basic_attack_is_clonable() {
        let ability1 = Ability::basic_attack();
        let ability2 = ability1.clone();

        assert_eq!(ability1.name, ability2.name);
        assert_eq!(ability1.power, ability2.power);
    }

    // ==================== compute_attack Tests ====================

    #[test]
    fn test_compute_attack_basic_damage() {
        let attacker = Character::new_player("Hero");
        let defender = Character::new_monster("Slime");
        let ability = Ability::basic_attack();

        let event = compute_attack(&attacker, &defender, &ability);

        // Expected: 10 (attack) + 5 (ability) - 1 (defense) = 14 damage
        assert_eq!(event.damage, 14);
        assert_eq!(event.attacker_name, "Hero");
        assert_eq!(event.defender_name, "Slime");
        assert_eq!(event.defender_hp_after, 6); // 20 - 14
    }

    #[test]
    fn test_compute_attack_monster_vs_player() {
        let attacker = Character::new_monster("Slime");
        let defender = Character::new_player("Hero");
        let ability = Ability::basic_attack();

        let event = compute_attack(&attacker, &defender, &ability);

        // Expected: 6 (attack) + 5 (ability) - 2 (defense) = 9 damage
        assert_eq!(event.damage, 9);
        assert_eq!(event.attacker_name, "Slime");
        assert_eq!(event.defender_name, "Hero");
        assert_eq!(event.defender_hp_after, 21); // 30 - 9
    }

    #[test]
    fn test_compute_attack_minimum_damage() {
        // Create a weak attacker with very low attack
        let weak_attacker = Character {
            name: "Weak".into(),
            hp: 10,
            stats: Stats {
                hp: 10,
                attack: 0,
                defense: 0,
            },
        };

        // Create a defender with very high defense
        let strong_defender = Character {
            name: "Tank".into(),
            hp: 100,
            stats: Stats {
                hp: 100,
                attack: 5,
                defense: 50,
            },
        };

        let ability = Ability::basic_attack();
        let event = compute_attack(&weak_attacker, &strong_defender, &ability);

        // Even if (attack + power - defense) is negative, damage should be at least 1
        assert_eq!(event.damage, 1);
        assert_eq!(event.defender_hp_after, 99); // 100 - 1
    }

    #[test]
    fn test_compute_attack_zero_defense() {
        let attacker = Character::new_player("Hero");
        let defender = Character {
            name: "NoArmor".into(),
            hp: 15,
            stats: Stats {
                hp: 15,
                attack: 5,
                defense: 0,
            },
        };
        let ability = Ability::basic_attack();

        let event = compute_attack(&attacker, &defender, &ability);

        // Expected: 10 (attack) + 5 (ability) - 0 (defense) = 15 damage
        assert_eq!(event.damage, 15);
        assert_eq!(event.defender_hp_after, 0); // 15 - 15
    }

    #[test]
    fn test_compute_attack_high_power_ability() {
        let attacker = Character::new_player("Hero");
        let defender = Character::new_monster("Slime");
        let powerful_ability = Ability {
            name: "Mega Attack".into(),
            power: 20,
        };

        let event = compute_attack(&attacker, &defender, &powerful_ability);

        // Expected: 10 (attack) + 20 (ability) - 1 (defense) = 29 damage
        assert_eq!(event.damage, 29);
        assert_eq!(event.defender_hp_after, -9); // 20 - 29 (negative HP allowed)
    }

    #[test]
    fn test_compute_attack_negative_hp_after() {
        let attacker = Character::new_player("Hero");
        let defender = Character {
            name: "Wounded".into(),
            hp: 5,
            stats: Stats {
                hp: 20,
                attack: 3,
                defense: 1,
            },
        };
        let ability = Ability::basic_attack();

        let event = compute_attack(&attacker, &defender, &ability);

        // Damage should exceed defender's current HP
        assert_eq!(event.damage, 14);
        assert_eq!(event.defender_hp_after, -9); // 5 - 14
    }

    #[test]
    fn test_compute_attack_exact_lethal_damage() {
        let attacker = Character::new_player("Hero");
        let defender = Character {
            name: "Exact".into(),
            hp: 14,
            stats: Stats {
                hp: 20,
                attack: 3,
                defense: 1,
            },
        };
        let ability = Ability::basic_attack();

        let event = compute_attack(&attacker, &defender, &ability);

        // Damage should exactly match defender's HP
        assert_eq!(event.damage, 14);
        assert_eq!(event.defender_hp_after, 0);
    }

    #[test]
    fn test_compute_attack_preserves_attacker_and_defender_names() {
        let attacker = Character::new_player("Alice");
        let defender = Character::new_monster("Bob");
        let ability = Ability::basic_attack();

        let event = compute_attack(&attacker, &defender, &ability);

        assert_eq!(event.attacker_name, "Alice");
        assert_eq!(event.defender_name, "Bob");
    }

    // ==================== Edge Case Tests ====================

    #[test]
    fn test_character_with_max_stats() {
        let max_char = Character {
            name: "MaxPower".into(),
            hp: i32::MAX,
            stats: Stats {
                hp: i32::MAX,
                attack: i32::MAX,
                defense: 0,
            },
        };

        // Should not panic
        assert_eq!(max_char.hp, i32::MAX);
        assert_eq!(max_char.stats.attack, i32::MAX);
    }

    #[test]
    fn test_ability_with_zero_power() {
        let zero_power = Ability {
            name: "Weak".into(),
            power: 0,
        };

        let attacker = Character::new_player("Hero");
        let defender = Character::new_monster("Slime");

        let event = compute_attack(&attacker, &defender, &zero_power);

        // Expected: 10 (attack) + 0 (ability) - 1 (defense) = 9 damage
        assert_eq!(event.damage, 9);
    }

    #[test]
    fn test_ability_with_negative_power() {
        let negative_power = Ability {
            name: "Debuff".into(),
            power: -5,
        };

        let attacker = Character::new_player("Hero");
        let defender = Character::new_monster("Slime");

        let event = compute_attack(&attacker, &defender, &negative_power);

        // Expected: 10 (attack) + (-5) (ability) - 1 (defense) = 4 damage
        // But minimum damage is 1, so if result would be < 1, it's clamped to 1
        assert_eq!(event.damage, 4);
    }

    #[test]
    fn test_character_with_zero_current_hp() {
        let dead_char = Character {
            name: "Dead".into(),
            hp: 0,
            stats: Stats {
                hp: 20,
                attack: 5,
                defense: 1,
            },
        };

        let attacker = Character::new_player("Hero");
        let ability = Ability::basic_attack();

        let event = compute_attack(&attacker, &dead_char, &ability);

        // Attacking a dead character should still work
        assert_eq!(event.defender_hp_after, -14); // 0 - 14
    }

    #[test]
    fn test_character_with_negative_hp() {
        let overkill = Character {
            name: "Overkill".into(),
            hp: -10,
            stats: Stats {
                hp: 20,
                attack: 5,
                defense: 1,
            },
        };

        let attacker = Character::new_player("Hero");
        let ability = Ability::basic_attack();

        let event = compute_attack(&attacker, &overkill, &ability);

        // Should handle negative HP without panic
        assert_eq!(event.defender_hp_after, -24); // -10 - 14
    }

    // ==================== Serialization Tests ====================

    #[test]
    fn test_stats_serialization() {
        let stats = Stats {
            hp: 30,
            attack: 10,
            defense: 2,
        };

        // Test that Stats can be serialized
        let json = serde_json::to_string(&stats).expect("Should serialize");

        // Test that it can be deserialized back
        let deserialized: Stats = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.hp, stats.hp);
        assert_eq!(deserialized.attack, stats.attack);
        assert_eq!(deserialized.defense, stats.defense);
    }

    #[test]
    fn test_character_serialization() {
        let character = Character::new_player("Hero");

        let json = serde_json::to_string(&character).expect("Should serialize");
        let deserialized: Character = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.name, character.name);
        assert_eq!(deserialized.hp, character.hp);
        assert_eq!(deserialized.stats.attack, character.stats.attack);
    }

    #[test]
    fn test_ability_serialization() {
        let ability = Ability::basic_attack();

        let json = serde_json::to_string(&ability).expect("Should serialize");
        let deserialized: Ability = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.name, ability.name);
        assert_eq!(deserialized.power, ability.power);
    }

    #[test]
    fn test_combat_event_serialization() {
        let attacker = Character::new_player("Hero");
        let defender = Character::new_monster("Slime");
        let ability = Ability::basic_attack();

        let event = compute_attack(&attacker, &defender, &ability);

        let json = serde_json::to_string(&event).expect("Should serialize");
        let deserialized: CombatEvent = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.attacker_name, event.attacker_name);
        assert_eq!(deserialized.defender_name, event.defender_name);
        assert_eq!(deserialized.damage, event.damage);
        assert_eq!(deserialized.defender_hp_after, event.defender_hp_after);
    }

    // ==================== Clone Tests ====================

    #[test]
    fn test_character_clone() {
        let original = Character::new_player("Original");
        let cloned = original.clone();

        assert_eq!(original.name, cloned.name);
        assert_eq!(original.hp, cloned.hp);
        assert_eq!(original.stats.attack, cloned.stats.attack);
        assert_eq!(original.stats.defense, cloned.stats.defense);
    }

    #[test]
    fn test_ability_clone() {
        let original = Ability::basic_attack();
        let cloned = original.clone();

        assert_eq!(original.name, cloned.name);
        assert_eq!(original.power, cloned.power);
    }

    #[test]
    fn test_combat_event_clone() {
        let event = CombatEvent {
            attacker_name: "Attacker".into(),
            defender_name: "Defender".into(),
            damage: 10,
            defender_hp_after: 5,
        };

        let cloned = event.clone();

        assert_eq!(event.attacker_name, cloned.attacker_name);
        assert_eq!(event.defender_name, cloned.defender_name);
        assert_eq!(event.damage, cloned.damage);
        assert_eq!(event.defender_hp_after, cloned.defender_hp_after);
    }

    // ==================== Property-Based Tests ====================

    #[test]
    fn test_damage_always_positive() {
        // Test with various random-ish combinations
        let test_cases = vec![
            (0, 0, 0),
            (1, 1, 1),
            (10, 5, 2),
            (100, 50, 25),
            (5, 20, 10),
        ];

        for (attack, power, defense) in test_cases {
            let attacker = Character {
                name: "Attacker".into(),
                hp: 100,
                stats: Stats {
                    hp: 100,
                    attack,
                    defense: 0,
                },
            };

            let defender = Character {
                name: "Defender".into(),
                hp: 100,
                stats: Stats {
                    hp: 100,
                    attack: 0,
                    defense,
                },
            };

            let ability = Ability {
                name: "Test".into(),
                power,
            };

            let event = compute_attack(&attacker, &defender, &ability);

            // Damage must always be at least 1
            assert!(
                event.damage >= 1,
                "Damage must be >= 1, got {} for attack={}, power={}, defense={}",
                event.damage,
                attack,
                power,
                defense
            );
        }
    }

    #[test]
    fn test_hp_after_consistency() {
        let attacker = Character::new_player("Hero");
        let defender = Character::new_monster("Slime");
        let ability = Ability::basic_attack();

        let event = compute_attack(&attacker, &defender, &ability);

        // HP after should equal current HP minus damage
        assert_eq!(event.defender_hp_after, defender.hp - event.damage);
    }

    #[test]
    fn test_attack_formula_consistency() {
        let attacker = Character {
            name: "A".into(),
            hp: 50,
            stats: Stats {
                hp: 50,
                attack: 15,
                defense: 0,
            },
        };

        let defender = Character {
            name: "D".into(),
            hp: 50,
            stats: Stats {
                hp: 50,
                attack: 0,
                defense: 3,
            },
        };

        let ability = Ability {
            name: "Test".into(),
            power: 7,
        };

        let event = compute_attack(&attacker, &defender, &ability);

        // Manually calculate expected damage
        let expected_raw = attacker.stats.attack + ability.power; // 15 + 7 = 22
        let expected_damage = (expected_raw - defender.stats.defense).max(1); // (22 - 3).max(1) = 19

        assert_eq!(event.damage, expected_damage);
    }
}
