# Chapter 3: Domain Modeling in F#

## What You'll Learn

In this chapter, you'll design a complete game domain in F# using type-driven development. We'll create a combat system that's impossible to misuse, with invalid states unrepresentable at the type level.

**Time Required**: 45 minutes

## The Power of Type-Driven Design

Before writing any game logic, we define our domain through types. This approach:
- Catches errors at compile time, not runtime
- Documents the system through types
- Makes illegal states impossible
- Guides implementation naturally

## Our Game Domain

We're building a turn-based RPG combat system with:
- Characters (heroes and monsters)
- Stats (HP, attack, defense)
- Abilities with cooldowns
- Status effects
- Combat events for logging

Let's build this incrementally.

## Step 1: Core Types

### Basic Statistics

```fsharp
// File: fsharp/Domain.fs

module Domain

/// Core character statistics
type Stats = {
    Hp: int          // Maximum health points
    Attack: int      // Base attack power
    Defense: int     // Damage reduction
    Speed: int       // Turn order priority
}

/// Validate stats are reasonable
let validateStats stats =
    stats.Hp > 0 &&
    stats.Attack >= 0 &&
    stats.Defense >= 0 &&
    stats.Speed > 0
```

### Character Definition

```fsharp
/// Represents any combatant
type Character = {
    Id: string       // Unique identifier
    Name: string     // Display name
    CurrentHp: int   // Current health
    Stats: Stats     // Base statistics
    Level: int       // Character level
}

/// Smart constructor ensures valid character
let createCharacter id name stats level =
    if not (validateStats stats) then
        Error "Invalid stats"
    elif level < 1 then
        Error "Level must be at least 1"
    else
        Ok {
            Id = id
            Name = name
            CurrentHp = stats.Hp
            Stats = stats
            Level = level
        }
```

**Key Design Decision**: Using a Result type for character creation prevents invalid characters from existing.

## Step 2: Combat Actions

### Modeling Abilities

```fsharp
/// Types of abilities
type AbilityType =
    | Physical
    | Magical
    | Healing
    | Buff
    | Debuff

/// Targeting rules
type TargetType =
    | Self
    | SingleEnemy
    | AllEnemies
    | SingleAlly
    | AllAllies

/// Complete ability definition
type Ability = {
    Id: string
    Name: string
    Description: string
    Type: AbilityType
    Target: TargetType
    Power: int           // Base power/healing
    ManaCost: int        // Resource cost
    Cooldown: int        // Turns before reuse
}

/// Track ability availability
type AbilityState = {
    Ability: Ability
    CurrentCooldown: int  // 0 means ready
}
```

### Status Effects

```fsharp
/// Status effects that modify combat
type StatusEffect =
    | Poisoned of damagePerTurn: int
    | Burning of damagePerTurn: int * duration: int
    | Stunned of turnsRemaining: int
    | Shielded of damageReduction: int
    | Strengthened of attackBonus: int * duration: int
    | Weakened of attackPenalty: int * duration: int

/// Get remaining duration of effect
let getEffectDuration effect =
    match effect with
    | Poisoned _ -> None  // Permanent until cured
    | Burning (_, d) -> Some d
    | Stunned t -> Some t
    | Shielded _ -> None  // Permanent until broken
    | Strengthened (_, d) -> Some d
    | Weakened (_, d) -> Some d
```

## Step 3: Combat State

### Battle State Machine

```fsharp
/// Possible battle states
type BattleState =
    | NotStarted
    | SelectingAction of activeCharacter: Character
    | ExecutingAction of action: CombatAction
    | ProcessingEffects of character: Character
    | CheckingVictory
    | BattleOver of result: BattleResult

and CombatAction =
    | BasicAttack of attacker: Character * target: Character
    | UseAbility of user: Character * ability: Ability * targets: Character list
    | Defend of character: Character
    | UseItem of user: Character * item: Item * target: Character option
    | Flee of character: Character

and BattleResult =
    | Victory of rewards: Rewards
    | Defeat
    | Draw
    | Fled

and Rewards = {
    Experience: int
    Gold: int
    Items: Item list
}

and Item = {
    Id: string
    Name: string
    Description: string
}
```

**Design Note**: The state machine ensures battles progress through valid states only.

## Step 4: Combat Events

### Event Sourcing Pattern

```fsharp
/// Everything that can happen in combat
type CombatEvent =
    // Damage events
    | DamageDealt of attacker: string * target: string * amount: int * damageType: AbilityType
    | DamageBlocked of target: string * amount: int
    | CriticalHit of attacker: string * multiplier: float

    // Healing events
    | HealingDone of healer: string * target: string * amount: int
    | Overhealing of target: string * excess: int

    // Status events
    | StatusApplied of target: string * effect: StatusEffect
    | StatusRemoved of target: string * effect: StatusEffect
    | StatusTicked of target: string * effect: StatusEffect * damage: int

    // Ability events
    | AbilityUsed of user: string * ability: string
    | AbilityCooldownStarted of ability: string * turns: int
    | AbilityFailed of ability: string * reason: string

    // Character events
    | CharacterDefeated of name: string
    | CharacterRevived of name: string * hp: int
    | CharacterFled of name: string

    // Turn events
    | TurnStarted of character: string
    | TurnEnded of character: string
    | BattleStarted of participants: string list
    | BattleEnded of result: BattleResult

/// Convert events to readable combat log
let formatEvent event =
    match event with
    | DamageDealt (attacker, target, amount, Physical) ->
        sprintf "%s strikes %s for %d damage!" attacker target amount
    | DamageDealt (attacker, target, amount, Magical) ->
        sprintf "%s blasts %s with magic for %d damage!" attacker target amount
    | HealingDone (healer, target, amount) when healer = target ->
        sprintf "%s heals themselves for %d HP!" healer amount
    | HealingDone (healer, target, amount) ->
        sprintf "%s heals %s for %d HP!" healer target amount
    | StatusApplied (target, Poisoned dmg) ->
        sprintf "%s is poisoned! Takes %d damage per turn." target dmg
    | CharacterDefeated name ->
        sprintf "%s has been defeated!" name
    | CriticalHit (attacker, mult) ->
        sprintf "%s scores a CRITICAL HIT! (%.1fx damage)" attacker mult
    | _ -> ""  // Handle other events as needed
```

## Step 5: Pure Combat Logic

### Damage Calculation

```fsharp
module Combat

open Domain

/// Calculate damage with all modifiers
let calculateDamage attacker defender ability =
    // Base damage
    let baseDamage =
        match ability.Type with
        | Physical -> attacker.Stats.Attack + ability.Power
        | Magical -> (attacker.Level * 2) + ability.Power
        | _ -> ability.Power

    // Apply defense
    let afterDefense =
        match ability.Type with
        | Physical -> max 1 (baseDamage - defender.Stats.Defense)
        | Magical -> baseDamage  // Magic ignores armor
        | _ -> baseDamage

    // Check for critical hit (10% chance)
    let criticalMultiplier = 1.5  // Could be random in real game
    let finalDamage =
        if attacker.Stats.Speed > defender.Stats.Speed * 2 then
            int (float afterDefense * criticalMultiplier)
        else
            afterDefense

    finalDamage

/// Apply damage and generate events
let applyDamage damage target =
    let newHp = max 0 (target.CurrentHp - damage)
    let updatedTarget = { target with CurrentHp = newHp }

    let events = [
        DamageDealt (target.Name, target.Name, damage, Physical)
        if newHp = 0 then
            CharacterDefeated target.Name
    ]

    updatedTarget, events
```

### Complete Turn Execution

```fsharp
/// Execute a complete combat turn
let executeTurn action state =
    match action with
    | BasicAttack (attacker, defender) ->
        // Create dummy ability for basic attack
        let basicAttack = {
            Id = "basic-attack"
            Name = "Attack"
            Description = "Basic physical attack"
            Type = Physical
            Target = SingleEnemy
            Power = 0
            ManaCost = 0
            Cooldown = 0
        }

        let damage = calculateDamage attacker defender basicAttack
        let updatedDefender, damageEvents = applyDamage damage defender

        let allEvents = [
            TurnStarted attacker.Name
            AbilityUsed (attacker.Name, "Attack")
        ] @ damageEvents @ [
            TurnEnded attacker.Name
        ]

        let newState =
            if updatedDefender.CurrentHp = 0 then
                BattleOver (Victory { Experience = 100; Gold = 50; Items = [] })
            else
                SelectingAction updatedDefender

        newState, updatedDefender, allEvents

    | UseAbility (user, ability, targets) ->
        // Complex ability logic here
        state, user, []  // Simplified for tutorial

    | Defend character ->
        let events = [
            TurnStarted character.Name
            StatusApplied (character.Name, Shielded 5)
            TurnEnded character.Name
        ]
        SelectingAction character, character, events

    | _ ->
        state, attacker, []
```

## Step 6: Mapping F# to Rust

Now we need to translate our F# types to Rust. Here's the systematic approach:

### F# to Rust Type Mapping

| F# Type | Rust Type | Notes |
|---------|-----------|-------|
| `int` | `i32` | 32-bit signed integer |
| `string` | `String` | Heap-allocated string |
| `float` | `f32` or `f64` | 32 or 64-bit float |
| `bool` | `bool` | Same in both |
| `option<'a>` | `Option<T>` | Maybe/nullable type |
| `Result<'a,'b>` | `Result<T,E>` | Error handling |
| Record `{}` | Struct `{}` | Named fields |
| Discriminated Union | Enum | Sum types |
| `list<'a>` | `Vec<T>` | Dynamic array |
| `'a * 'b` (tuple) | `(A, B)` | Tuple type |

### Example Translation

**F# Domain:**
```fsharp
type Stats = {
    Hp: int
    Attack: int
    Defense: int
}

type Character = {
    Name: string
    CurrentHp: int
    Stats: Stats
}

type CombatResult =
    | Victory of xp: int
    | Defeat
    | Draw
```

**Rust Implementation:**
```rust
// In crates/logic-fsharp/src/lib.rs

#[derive(Clone, Debug)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub current_hp: i32,
    pub stats: Stats,
}

#[derive(Clone, Debug)]
pub enum CombatResult {
    Victory { xp: i32 },
    Defeat,
    Draw,
}
```

### Key Differences to Remember

1. **Naming Conventions**:
   - F#: PascalCase for types and fields
   - Rust: snake_case for fields, PascalCase for types

2. **Mutability**:
   - F#: Immutable by default
   - Rust: Immutable by default, use `mut` for mutable

3. **Memory Management**:
   - F#: Garbage collected
   - Rust: Ownership system

4. **Pattern Matching**:
   - F#: `match x with`
   - Rust: `match x {}`

## Exercises

### Exercise 1: Add Mana System

Extend the character type to include mana:

```fsharp
type Character = {
    // Existing fields...
    CurrentMana: int
    MaxMana: int
}

// Create functions to:
// 1. Check if character has enough mana for ability
// 2. Consume mana when using ability
// 3. Regenerate mana each turn
```

### Exercise 2: Implement Equipment

Design an equipment system:

```fsharp
type EquipmentSlot =
    | Weapon
    | Armor
    | Accessory

type Equipment = {
    Slot: EquipmentSlot
    StatBonuses: Stats
    SpecialEffects: StatusEffect list
}

// Tasks:
// 1. Add Equipment list to Character
// 2. Calculate total stats including equipment
// 3. Ensure only one item per slot
```

### Exercise 3: Create AI Decision Tree

Design AI behavior types:

```fsharp
type AIBehavior =
    | Aggressive  // Always attack
    | Defensive   // Heal when low, defend often
    | Balanced    // Mix of strategies
    | Support     // Heal allies, buff team

type AIDecision = {
    Behavior: AIBehavior
    State: BattleState
    Action: CombatAction
}

// Implement decision logic for each behavior
```

## Design Patterns Summary

### 1. Smart Constructors
```fsharp
let createCharacter name stats =
    if validateStats stats then
        Ok { Name = name; Stats = stats; CurrentHp = stats.Hp }
    else
        Error "Invalid character data"
```

### 2. State Machines
```fsharp
type State = A | B | C
let transition state input =
    match state, input with
    | A, X -> B
    | B, Y -> C
    | _ -> state  // No transition
```

### 3. Event Sourcing
```fsharp
let applyEvent state event =
    match event with
    | DamageDealt (_, target, amount, _) ->
        updateCharacterHp target (-amount) state
    | HealingDone (_, target, amount) ->
        updateCharacterHp target amount state
    | _ -> state
```

## Complete Domain Checklist

Your domain model should define:

✅ Core types (Stats, Character)
✅ Actions (abilities, items)
✅ State transitions (battle states)
✅ Events (for logging/replay)
✅ Validation (smart constructors)
✅ Business rules (damage formulas)
✅ Error handling (Result types)

## What You've Learned

- Type-driven design prevents bugs before they happen
- Discriminated unions model all possible states
- Events provide audit trail and replay capability
- Pure functions make logic testable
- Smart constructors enforce invariants

## Next Steps

Now that we have a complete domain model in F#, we need to implement it in Rust. The next chapter will show you how to translate F# types and functions into idiomatic Rust code.

[Next: Rust Implementation →](04-rust-implementation.md)

[← Previous: Understanding F#](02-understanding-fsharp.md) | [Tutorial Index](README.md)