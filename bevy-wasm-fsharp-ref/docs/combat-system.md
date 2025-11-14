# Turn-Based Combat System

This document describes the turn-based combat system implemented in bevy-wasm-fsharp-ref.

## Overview

The game features a proper turn-based combat system where the player and monster take turns attacking each other. The system is built using Bevy's ECS (Entity-Component-System) architecture with state management, timers, and event logging.

## Architecture

### Core Components

#### CombatState (Resource)

Tracks the current state of combat:

```rust
#[derive(Resource, Debug, Clone, PartialEq)]
enum CombatState {
    PlayerTurn,    // Player can input actions
    MonsterTurn,   // Monster acts automatically
    GameOver { winner: String },  // Combat ended
}
```

**State Transitions:**
- Game starts in `PlayerTurn`
- Player attack → `MonsterTurn`
- Monster attack → `PlayerTurn` (if player alive)
- Either HP ≤ 0 → `GameOver { winner }`

#### CombatLog (Resource)

Tracks and displays combat events:

```rust
#[derive(Resource, Default)]
struct CombatLog {
    events: VecDeque<String>,  // Recent events
    max_events: usize,         // Maximum stored events (10)
}
```

**Methods:**
- `add(message)` - Adds event and prints to console
- `get_recent(count)` - Retrieves recent events (for future UI)

#### MonsterTurnTimer (Resource)

Controls the delay before monster attacks:

```rust
#[derive(Resource)]
struct MonsterTurnTimer {
    timer: Timer,  // 1.0 second delay
}
```

This creates a natural pause that makes turns feel distinct.

## Systems

### 1. handle_player_turn

**Responsibilities:**
- Listen for Space key during `PlayerTurn` only
- Execute player attack against monster
- Apply damage and update HP
- Log attack events
- Check for monster defeat
- Transition to `MonsterTurn` or `GameOver`

**Flow:**
```
Check state == PlayerTurn
  ↓
Wait for Space key
  ↓
Get player & monster entities
  ↓
Compute attack damage
  ↓
Log attack event
  ↓
Apply damage to monster
  ↓
Check if monster.hp <= 0
  ↓ YES              ↓ NO
GameOver         MonsterTurn
```

### 2. handle_monster_turn

**Responsibilities:**
- Automatically execute monster attack after delay
- Apply damage to player
- Log attack events
- Check for player defeat
- Transition to `PlayerTurn` or `GameOver`

**Flow:**
```
Check state == MonsterTurn
  ↓
Tick timer with delta time
  ↓
Wait for timer.finished()
  ↓
Reset timer
  ↓
Get player & monster entities
  ↓
Compute attack damage
  ↓
Log attack event
  ↓
Apply damage to player
  ↓
Check if player.hp <= 0
  ↓ YES              ↓ NO
GameOver         PlayerTurn
```

### 3. check_game_over

**Responsibilities:**
- Detect when combat enters `GameOver` state
- Log final victory messages
- Announce winner

**Trigger:**
- Only runs when `CombatState` changes using `is_changed()`
- Prevents duplicate messages

### 4. display_turn_indicator

**Responsibilities:**
- Show current turn in console
- Provide player instructions
- Announce game over

**Output Examples:**
```
>>> YOUR TURN <<<
Press SPACE to attack!

>>> MONSTER'S TURN <<<
Monster is preparing to attack...

>>> GAME OVER <<<
Winner: Hero
```

### 5. display_combat_log

**Responsibilities:**
- Display combat events as they happen
- Currently logs to console
- Future: Can integrate with UI system

**Note:** Events are already printed in `CombatLog::add()`, but this system exists for future UI integration.

## System Ordering

Systems run in a **chain** to ensure proper execution order:

```rust
.add_systems(
    Update,
    (
        handle_player_turn,
        handle_monster_turn,
        check_game_over,
        display_turn_indicator,
        display_combat_log,
    ).chain(),
)
```

This ensures:
1. Player input is processed first
2. Monster AI runs second
3. Game over is checked
4. UI updates happen last

## Combat Flow Example

### Turn 1: Player Attack

```
>>> YOUR TURN <<<
Press SPACE to attack!

[Player presses Space]

Hero attacks Slime for 14 damage!
Slime HP: 6 / 20
--- Monster's Turn ---
```

### Turn 2: Monster Attack

```
>>> MONSTER'S TURN <<<
Monster is preparing to attack...

[1 second delay]

Slime attacks Hero for 9 damage!
Hero HP: 21 / 30
--- Player's Turn ---
```

### Turn 3: Player Finishes Monster

```
>>> YOUR TURN <<<
Press SPACE to attack!

[Player presses Space]

Hero attacks Slime for 14 damage!
Slime HP: -8 / 20
Slime has been defeated!
=== GAME OVER ===
Hero wins!
Close the window to exit.

>>> GAME OVER <<<
Winner: Hero
```

## Damage Calculation

Damage is computed in the `logic` crate:

```rust
let raw = attacker.stats.attack + ability.power;
let damage = (raw - defender.stats.defense).max(1);  // Minimum 1 damage
let defender_hp_after = defender.hp - damage;
```

### Default Stats

**Player (Hero):**
- HP: 30
- Attack: 10
- Defense: 2
- Damage dealt: (10 + 5 - 1) = **14 per hit**

**Monster (Slime):**
- HP: 20
- Attack: 6
- Defense: 1
- Damage dealt: (6 + 5 - 2) = **9 per hit**

### Battle Math

**Hits to victory:**
- Player needs: 20 / 14 = **2 hits** to defeat monster
- Monster needs: 30 / 9 = **4 hits** to defeat player

**Player has advantage** and will win if they attack every turn.

## Combat States

### PlayerTurn

**Characteristics:**
- Player can press Space to attack
- Monster is inactive
- No timer running
- Turn indicator shows ">>> YOUR TURN <<<"

**Exit Conditions:**
- Player attacks → `MonsterTurn`
- (Never exits to GameOver directly, always transitions through attack first)

### MonsterTurn

**Characteristics:**
- Player input is ignored
- Monster timer is ticking
- After 1 second, monster attacks automatically
- Turn indicator shows ">>> MONSTER'S TURN <<<"

**Exit Conditions:**
- Monster attacks & player alive → `PlayerTurn`
- Monster attacks & player dead → `GameOver { winner: "Slime" }`

### GameOver

**Characteristics:**
- No more turns execute
- Player input is ignored
- Monster doesn't attack
- Turn indicator shows ">>> GAME OVER <<<"
- Winner is announced

**Exit Conditions:**
- None (terminal state)

## Testing

### Manual Test Checklist

See `crates/app/tests/combat_system_test.rs` for complete test cases.

**Quick Tests:**

1. **Turn Alternation:** Attack once, verify monster attacks back
2. **Player Victory:** Attack twice, verify "Hero wins!"
3. **Monster Victory:** Don't attack, let monster win (4 hits)
4. **Input Restriction:** Try pressing Space during monster turn (should do nothing)
5. **Auto-Attack:** Verify monster attacks after 1 second without input
6. **Game Over:** Verify no more turns after someone dies

### Expected Console Output

```
=== Combat Start ===
Press SPACE to attack on your turn!

>>> YOUR TURN <<<
Press SPACE to attack!
Hero attacks Slime for 14 damage!
Slime HP: 6 / 20
--- Monster's Turn ---

>>> MONSTER'S TURN <<<
Monster is preparing to attack...
Slime attacks Hero for 9 damage!
Hero HP: 21 / 30
--- Player's Turn ---

>>> YOUR TURN <<<
Press SPACE to attack!
Hero attacks Slime for 14 damage!
Slime HP: -8 / 20
Slime has been defeated!
=== GAME OVER ===
Hero wins!
Close the window to exit.

>>> GAME OVER <<<
Winner: Hero
```

## Future Enhancements

### Planned Improvements (see GitHub issues)

1. **Issue #11: Advanced Monster AI**
   - Multiple attack strategies
   - Ability selection logic
   - Defensive behaviors

2. **Issue #6: Combat UI Feedback**
   - Health bars above entities
   - Damage numbers floating up
   - Turn indicator on screen
   - Combat log overlay

3. **Multiple Abilities**
   - Special attacks
   - Defensive moves
   - Healing abilities

4. **Status Effects**
   - Poison, stun, buffs
   - Duration tracking
   - Effect resolution system

### Extension Points

The current system is designed to be extended:

- **CombatLog**: Already has `get_recent()` for UI integration
- **State Pattern**: Easy to add new states (e.g., `Ability Selection`)
- **Ability System**: Just add more `Ability` variants
- **AI**: Replace simple auto-attack in `handle_monster_turn`

## Code Organization

```
crates/app/src/main.rs
├── Resources
│   ├── CombatState       (turn tracking)
│   ├── CombatLog         (event history)
│   └── MonsterTurnTimer  (AI timing)
├── Components
│   ├── Player            (marker)
│   └── Monster           (marker)
└── Systems
    ├── handle_player_turn
    ├── handle_monster_turn
    ├── check_game_over
    ├── display_turn_indicator
    └── display_combat_log
```

## Design Decisions

### Why separate player and monster systems?

- **Clarity:** Each system has a single responsibility
- **Extensibility:** Easy to add different AI behaviors
- **Testing:** Can test player and monster logic independently

### Why use a timer for monster turn?

- **UX:** Creates a natural pause between turns
- **Feedback:** Player can see their attack result before monster responds
- **Pacing:** Prevents combat from feeling too fast

### Why chain the systems?

- **Consistency:** Ensures deterministic execution order
- **State Safety:** Combat state always updates before UI
- **Predictability:** Easy to reason about system execution

### Why separate CombatLog from console?

- **Future-Proofing:** Easy to add UI display later
- **Flexibility:** Can show different information in different places
- **Reusability:** Log can be saved, replayed, or analyzed

## Related Files

- **Implementation:** `/Users/beengud/raibid-labs/grimware/bevy-wasm-fsharp-ref/crates/app/src/main.rs`
- **Logic Layer:** `/Users/beengud/raibid-labs/grimware/bevy-wasm-fsharp-ref/crates/logic-fsharp/src/lib.rs`
- **Tests:** `/Users/beengud/raibid-labs/grimware/bevy-wasm-fsharp-ref/crates/app/tests/combat_system_test.rs`
- **Architecture:** `/Users/beengud/raibid-labs/grimware/bevy-wasm-fsharp-ref/docs/architecture.md`

## Troubleshooting

### Monster attacks immediately without delay

**Cause:** Timer already finished from previous turn
**Fix:** Ensure `timer.reset()` is called after monster attacks

### Player can attack during monster turn

**Cause:** State check is missing or incorrect
**Fix:** Verify `*combat_state != CombatState::PlayerTurn` guard

### Combat doesn't end when HP ≤ 0

**Cause:** Game over transition is missing
**Fix:** Check that both `handle_player_turn` and `handle_monster_turn` set `CombatState::GameOver`

### Events logged multiple times

**Cause:** System running on every frame instead of state change
**Fix:** Use `is_changed()` or similar guards

---

**Last Updated:** 2025-11-14
**Issue:** #14
**Status:** Complete
