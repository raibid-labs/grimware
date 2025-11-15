module GameLogic

open Domain

/// Ability type enumeration for AI decision making
type AbilityType =
    | BasicAttack
    | PowerfulAttack
    | Heal

/// Extended ability with type and cooldown support
type AbilityWithMeta =
    { Ability: Ability
      AbilityType: AbilityType
      Cooldown: int
      CurrentCooldown: int }

/// Basic attack ability with power of 5.
/// Corresponds to Rust: Ability::basic_attack()
let basicAttack =
    { Name = "Basic Attack"
      Power = 5 }

/// Powerful attack ability with higher damage but longer cooldown.
/// Corresponds to Rust: Ability::powerful_attack()
let powerfulAttack =
    { Name = "Powerful Attack"
      Power = 12 }

/// Heal ability that restores HP instead of dealing damage.
/// Corresponds to Rust: Ability::heal()
let healAbility =
    { Name = "Heal"
      Power = 10 }

/// Creates a basic attack with metadata
let basicAttackWithMeta =
    { Ability = basicAttack
      AbilityType = BasicAttack
      Cooldown = 0
      CurrentCooldown = 0 }

/// Creates a powerful attack with metadata (3-turn cooldown)
let powerfulAttackWithMeta =
    { Ability = powerfulAttack
      AbilityType = PowerfulAttack
      Cooldown = 3
      CurrentCooldown = 0 }

/// Creates a heal ability with metadata (4-turn cooldown)
let healAbilityWithMeta =
    { Ability = healAbility
      AbilityType = Heal
      Cooldown = 4
      CurrentCooldown = 0 }

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

/// AI decision-making function for monster combat behavior.
///
/// Strategy:
/// - If monster HP < 30%: Prioritize healing (if available and off cooldown)
/// - If monster HP > 70%: Use powerful attack (if available and off cooldown)
/// - Otherwise: Use basic attack (always available)
///
/// This creates intelligent but beatable AI that adapts to the combat situation.
///
/// Type Mapping (F# → Rust):
/// - F# `chooseMonsterAction` → Rust `choose_monster_action`
/// - F# `Character` → Rust `Character`
/// - F# `Ability list` → Rust `Vec<Ability>`
///
/// Corresponds to Rust:
/// ```rust
/// pub fn choose_monster_action(
///     monster: &Character,
///     player: &Character,
///     available_abilities: &[AbilityWithMeta],
/// ) -> Ability
/// ```
///
/// Example:
/// ```fsharp
/// let monster = { Name = "Goblin"; Hp = 5; Stats = { Hp = 20; Attack = 6; Defense = 1 } }
/// let player = { Name = "Hero"; Hp = 25; Stats = { Hp = 30; Attack = 10; Defense = 2 } }
/// let abilities = [basicAttackWithMeta; powerfulAttackWithMeta; healAbilityWithMeta]
/// let chosen = chooseMonsterAction monster player abilities
/// // Since monster HP is 5/20 = 25% (< 30%), AI will choose heal if available
/// ```
let chooseMonsterAction (monster: Character) (_player: Character) (availableAbilities: AbilityWithMeta list) : Ability =
    // Calculate monster's HP percentage
    let hpPercent = float monster.Hp / float monster.Stats.Hp

    // Filter abilities that are not on cooldown
    let usableAbilities = availableAbilities |> List.filter (fun a -> a.CurrentCooldown = 0)

    // Defensive strategy: HP < 30%, try to heal
    if hpPercent < 0.3 then
        match usableAbilities |> List.tryFind (fun a -> a.AbilityType = Heal) with
        | Some healMeta -> healMeta.Ability
        | None ->
            // No heal available, fall back to basic attack
            match usableAbilities |> List.tryFind (fun a -> a.AbilityType = BasicAttack) with
            | Some basic -> basic.Ability
            | None -> basicAttack // Emergency fallback

    // Aggressive strategy: HP > 70%, use powerful attack
    elif hpPercent > 0.7 then
        match usableAbilities |> List.tryFind (fun a -> a.AbilityType = PowerfulAttack) with
        | Some powerfulMeta -> powerfulMeta.Ability
        | None ->
            // Powerful attack on cooldown, use basic attack
            match usableAbilities |> List.tryFind (fun a -> a.AbilityType = BasicAttack) with
            | Some basic -> basic.Ability
            | None -> basicAttack // Emergency fallback

    // Balanced strategy: 30% <= HP <= 70%, use basic attack
    else
        match usableAbilities |> List.tryFind (fun a -> a.AbilityType = BasicAttack) with
        | Some basic -> basic.Ability
        | None -> basicAttack // Emergency fallback
