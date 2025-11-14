module GameLogic

open Domain

/// Basic attack ability with power of 5.
/// Corresponds to Rust: Ability::basic_attack()
let basicAttack =
    { Name = "Basic Attack"
      Power = 5 }

/// Computes the result of an attack between two characters using a specified ability.
///
/// Damage calculation: damage = max(1, attacker.attack + ability.power - defender.defense)
/// - Ensures minimum 1 damage even if defender's defense is high
/// - Returns a CombatEvent containing the combat result
///
/// Type Mapping (F# → Rust):
/// - F# `int` → Rust `i32`
/// - F# `Character` → Rust `Character`
/// - F# `Ability` → Rust `Ability`
/// - F# `CombatEvent` → Rust `CombatEvent`
///
/// Naming Convention:
/// - F# PascalCase fields → Rust snake_case fields
/// - F# `computeAttack` → Rust `compute_attack`
///
/// Corresponds to Rust:
/// ```rust
/// pub fn compute_attack(
///     attacker: &Character,
///     defender: &Character,
///     ability: &Ability,
/// ) -> CombatEvent
/// ```
///
/// Example:
/// ```fsharp
/// let player = { Name = "Hero"; Hp = 30; Stats = { Hp = 30; Attack = 10; Defense = 2 } }
/// let monster = { Name = "Goblin"; Hp = 20; Stats = { Hp = 20; Attack = 6; Defense = 1 } }
/// let result = computeAttack player monster basicAttack
/// // result.Damage will be max(1, 10 + 5 - 1) = 14
/// // result.DefenderHpAfter will be 20 - 14 = 6
/// ```
let computeAttack (attacker: Character) (defender: Character) (ability: Ability) : CombatEvent =
    let raw = attacker.Stats.Attack + ability.Power
    let dmg = max 1 (raw - defender.Stats.Defense)
    let hpAfter = defender.Hp - dmg
    { AttackerName = attacker.Name
      DefenderName = defender.Name
      Damage = dmg
      DefenderHpAfter = hpAfter }
