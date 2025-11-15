# Chapter 2: Understanding F# for Game Logic

## What You'll Learn

In this chapter, you'll discover why F# excels at domain modeling and learn the key concepts that make it perfect for defining game logic. We'll cover F# basics through the lens of game development.

**Time Required**: 30 minutes

## Why F# for Game Logic?

Traditional game development often struggles with:
- Complex state management
- Bug-prone combat calculations
- Difficult-to-test game rules
- Unpredictable side effects

F# solves these problems through:
- **Immutability**: Data doesn't change unexpectedly
- **Type Safety**: Invalid states are impossible
- **Pure Functions**: Predictable, testable logic
- **Pattern Matching**: Handle all cases explicitly

## F# Core Concepts

### 1. Immutability by Default

In F#, values don't change after creation:

```fsharp
// F# - Immutable by default
let health = 100
// health = 50  // ERROR! Can't change immutable value

// Create new value instead
let damagedHealth = health - 20  // 80
// Original health is still 100
```

Compare to typical mutable approach:

```csharp
// C# - Mutable by default (bug-prone)
int health = 100;
health = health - 20;  // Original value lost
// What was the original health? We don't know!
```

**Game Benefit**: Track state changes explicitly, making combat replay and debugging trivial.

### 2. Record Types

Records are F#'s way of grouping related data:

```fsharp
// Define a character type
type Character = {
    Name: string
    Hp: int
    MaxHp: int
    Level: int
}

// Create a character (immutable)
let hero = {
    Name = "Aria"
    Hp = 100
    MaxHp = 100
    Level = 1
}

// "Update" by creating new record with changes
let damagedHero = { hero with Hp = 80 }
// Original hero still has 100 HP!
```

**Game Benefit**: Characters can't be accidentally modified during combat calculations.

### 3. Discriminated Unions (Sum Types)

Model different possibilities explicitly:

```fsharp
// Combat can only have these outcomes
type CombatResult =
    | Victory of experienceGained: int
    | Defeat
    | Fled

// Abilities can be in different states
type AbilityState =
    | Ready
    | Cooldown of turnsRemaining: int
    | Disabled of reason: string

// Handle all cases
let describeCombat result =
    match result with
    | Victory xp -> sprintf "You won! Gained %d XP" xp
    | Defeat -> "You were defeated..."
    | Fled -> "You ran away!"
    // Compiler ensures we handle ALL cases
```

**Game Benefit**: Impossible to forget edge cases - compiler enforces completeness.

### 4. Pattern Matching

F#'s pattern matching is like a switch statement on steroids:

```fsharp
type Enemy =
    | Slime of level: int
    | Goblin of weapon: string
    | Dragon of element: string * ancient: bool

let getAttackPower enemy =
    match enemy with
    | Slime level -> level * 2
    | Goblin "sword" -> 15
    | Goblin "bow" -> 12
    | Goblin _ -> 10  // Any other weapon
    | Dragon (element, true) -> 100  // Ancient dragon
    | Dragon (element, false) -> 50  // Young dragon
```

**Game Benefit**: Complex combat rules become clear and maintainable.

### 5. Pure Functions

Functions that always return the same output for the same input:

```fsharp
// Pure function - no side effects
let calculateDamage attacker defender =
    let baseDamage = attacker.Attack - defender.Defense
    max 1 baseDamage  // Minimum 1 damage

// NOT pure - has side effects
let mutableHP = ref 100
let applyDamageImpure damage =
    mutableHP := !mutableHP - damage  // Modifies external state
    printfn "Took %d damage!" damage  // I/O side effect
```

**Game Benefit**: Pure functions are trivial to test and debug.

## F# vs Imperative Game Logic

Let's compare approaches to a common game scenario:

### Imperative Approach (C#-style)

```csharp
class CombatSystem {
    void ExecuteTurn(Character attacker, Character defender) {
        // Multiple state changes, hard to track
        int damage = attacker.Attack - defender.Defense;
        if (damage < 1) damage = 1;

        defender.Hp -= damage;  // Mutates defender

        if (defender.Hp <= 0) {
            defender.Hp = 0;  // Ensure non-negative
            defender.IsAlive = false;  // Another mutation
            attacker.Experience += defender.ExpReward;  // Mutates attacker
        }

        LogCombat(/* ... */);  // Side effect
        UpdateUI();  // Another side effect
        SaveGame();  // Yet another side effect
    }
}
```

Problems:
- State changes everywhere
- Hard to test (needs mocks for UI, save system)
- Difficult to replay or undo
- Side effects mixed with logic

### Functional Approach (F#)

```fsharp
// Pure combat logic - no side effects
let executeTurn attacker defender =
    let damage = calculateDamage attacker defender
    let updatedDefender = applyDamage damage defender

    if not (isAlive updatedDefender) then
        let updatedAttacker = gainExperience defender.ExpReward attacker
        Victory (updatedAttacker, updatedDefender)
    else
        Continue (attacker, updatedDefender)

// Side effects handled separately
let processCombatTurn attacker defender =
    let result = executeTurn attacker defender  // Pure

    // Side effects isolated here
    logCombat result
    updateUI result
    saveGame result

    result
```

Benefits:
- Logic is pure and testable
- State changes are explicit
- Easy to replay/undo
- Side effects isolated

## Practical F# Patterns for Games

### Pattern 1: State Machines

```fsharp
type BattleState =
    | PlayerTurn of player: Character * enemy: Character
    | EnemyTurn of player: Character * enemy: Character
    | BattleEnd of result: CombatResult

let transitionBattle state action =
    match state, action with
    | PlayerTurn (p, e), Attack ->
        let newEnemy = applyPlayerAttack p e
        if isAlive newEnemy then
            EnemyTurn (p, newEnemy)
        else
            BattleEnd (Victory 100)

    | EnemyTurn (p, e), _ ->
        let newPlayer = applyEnemyAttack e p
        if isAlive newPlayer then
            PlayerTurn (newPlayer, e)
        else
            BattleEnd Defeat

    | BattleEnd _, _ ->
        state  // No transitions from end state
```

### Pattern 2: Event Sourcing

```fsharp
type GameEvent =
    | DamageDealt of target: string * amount: int
    | CharacterDefeated of name: string
    | ItemUsed of item: string * user: string
    | AbilityCast of ability: string * caster: string

// Game state is result of applying all events
let rec applyEvents events state =
    match events with
    | [] -> state
    | event :: rest ->
        let newState = applyEvent event state
        applyEvents rest newState

// Easy to implement features like:
// - Replay system
// - Undo/redo
// - Network synchronization
// - Save games
```

### Pattern 3: Composition

```fsharp
// Small, composable functions
let isLowHealth character = character.Hp < character.MaxHp / 4
let hasStatusEffect effect character =
    List.contains effect character.StatusEffects
let canAct character =
    isAlive character && not (hasStatusEffect Stunned character)

// Combine for complex logic
let shouldUseHealingPotion character =
    canAct character &&
    isLowHealth character &&
    character.Potions > 0

// AI decision making
let chooseAIAction character enemy =
    if shouldUseHealingPotion character then
        UsePotion
    elif not (canAct character) then
        Skip
    elif isLowHealth enemy then
        FinishingBlow
    else
        NormalAttack
```

## Key Differences from Rust

While both F# and Rust have strong type systems, they differ in approach:

| Aspect | F# | Rust |
|--------|----|----|
| Memory | Garbage collected | Manual (ownership) |
| Null Safety | Option type | Option type |
| Mutability | Immutable default | Immutable default with `mut` |
| Side Effects | IO monad (advanced) | No special handling |
| Syntax | Lightweight, minimal | More explicit |
| Compilation | To .NET IL | To native code |

## Exercises

### Exercise 1: Model a Spell System

Create F# types for a spell system:

```fsharp
type Element = Fire | Water | Earth | Air

type Spell = {
    Name: string
    Element: Element
    ManaCost: int
    Damage: int
}

// Your task: Create a function that calculates
// bonus damage when elements interact
// (Fire beats Earth, Earth beats Air, etc.)
```

### Exercise 2: Inventory Management

Design an inventory system with these requirements:
- Items have weight and value
- Inventory has max weight capacity
- Items can stack (potions, arrows)
- Some items are unique (legendary weapons)

```fsharp
type Item =
    | Stackable of name: string * count: int * weight: float
    | Unique of name: string * weight: float * bonuses: Map<string, int>

type Inventory = {
    Items: Item list
    MaxWeight: float
}

// Your task: Write functions to:
// 1. Add items (respecting weight limit)
// 2. Calculate total weight
// 3. Find most valuable items
```

### Exercise 3: Combat Modifiers

Extend the combat system with modifiers:

```fsharp
type CombatModifier =
    | AttackBoost of percentage: int
    | DefenseBoost of flat: int
    | CriticalHit of multiplier: float
    | Dodge

// Your task: Modify calculateDamage to account
// for a list of active modifiers
```

## Common Patterns Cheat Sheet

```fsharp
// Creating records
let character = { Name = "Hero"; Hp = 100 }

// Updating records
let hurt = { character with Hp = 80 }

// Pattern matching
match value with
| Pattern1 -> result1
| Pattern2 x -> result2 x
| _ -> defaultResult

// Option handling
let safeDivide x y =
    if y = 0 then None
    else Some (x / y)

// Piping
value
|> function1
|> function2
|> function3

// List operations
[1..10]
|> List.filter (fun x -> x % 2 = 0)
|> List.map (fun x -> x * 2)
|> List.sum
```

## Summary

F# provides powerful tools for game logic:

- **Immutability** prevents accidental state corruption
- **Record Types** model game entities clearly
- **Discriminated Unions** represent all possible states
- **Pattern Matching** handles complex rules elegantly
- **Pure Functions** make logic testable and debuggable

These concepts eliminate entire categories of bugs common in game development while making your code more maintainable and easier to reason about.

## Next Steps

Now that you understand F# fundamentals, you're ready to design a complete game domain. In the next chapter, we'll create the type model for our combat system.

[Next: Domain Modeling →](03-domain-modeling.md)

[← Previous: Getting Started](01-getting-started.md) | [Tutorial Index](README.md)