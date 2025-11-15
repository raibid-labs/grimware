# Ability System with Cooldowns

## Quick Start

The ability system provides multiple abilities per character with cooldown management, healing mechanics, and visual feedback.

### Player Controls

- **Key 1**: Basic Attack (5 power, 0.5s cooldown)
- **Key 2**: Powerful Attack (12 power, 3s cooldown)
- **Key 3**: Heal (-8 power, 5s cooldown) - Heals yourself
- **Key 4**: Quick Strike (3 power, 0.2s cooldown)

### Visual Indicators

- **Green Button**: Ability is ready to use
- **Gray Button**: Ability is on cooldown
- **Cooldown Bar**: Fills from left as cooldown decreases
- **Timer Text**: Shows remaining cooldown ("CD: 2.5s") or "READY"

## Architecture

### Type Hierarchy

```
AbilitySet (Component)
    └─→ Vec<AbilitySlot>
            ├─→ Ability (name, power)
            ├─→ cooldown_max: f32
            └─→ cooldown_current: f32
```

### F# to Rust Mapping

| F# Type | Rust Type | Purpose |
|---------|-----------|---------|
| `AbilitySlot` | `AbilitySlot` | Single ability with cooldown |
| `AbilitySet` | `AbilitySet` | Collection of abilities |
| `CombatEvent.AbilityUsed` | `CombatEvent.ability_used` | Track which ability was used |

## Abilities

### Basic Attack
- **Power**: 5
- **Cooldown**: 0.5 seconds
- **Strategy**: Reliable damage, short cooldown
- **Best For**: Consistent damage output

### Powerful Attack
- **Power**: 12
- **Cooldown**: 3.0 seconds
- **Strategy**: High burst damage, long cooldown
- **Best For**: Finishing enemies, burst damage windows

### Heal
- **Power**: -8 (negative = healing)
- **Cooldown**: 5.0 seconds
- **Strategy**: Self-healing, bypasses defense
- **Best For**: Survival, long battles

### Quick Strike
- **Power**: 3
- **Cooldown**: 0.2 seconds
- **Strategy**: Spammable low damage
- **Best For**: Cooldown filling, chip damage

## Healing Mechanics

Healing abilities use **negative power values**:

1. **Bypass Defense**: Healing ignores the defender's defense stat
2. **Self-Target**: Healing abilities automatically target the caster
3. **Direct Application**: Healing amount = `-ability.power`
4. **HP Cap**: HP cannot exceed `stats.hp` (enforced in app layer)

Example:
```rust
let heal = Ability::heal(); // power = -8
let event = compute_attack(&player, &player, &heal);
// event.damage = -8 (negative damage = healing)
// player.hp increases by 8
```

## Implementation Details

### Cooldown System

```rust
// Tick all cooldowns each frame
fn tick_ability_cooldowns(time: Res<Time>, mut query: Query<&mut AbilitySet>) {
    let delta = time.delta_secs();
    for mut ability_set in query.iter_mut() {
        ability_set.tick_all(delta);
    }
}
```

### Ability Selection

```rust
// Player selects ability with keyboard (1-4)
fn handle_player_turn(keys: Res<ButtonInput<KeyCode>>, ...) {
    let ability_index = if keys.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if keys.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keys.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else if keys.just_pressed(KeyCode::Digit4) {
        Some(3)
    } else {
        None
    };

    if let Some(idx) = ability_index {
        if let Some(slot) = ability_set.get_ready_ability(idx) {
            let ability = slot.ability.clone();
            slot.use_ability(); // Start cooldown
            // ... use ability
        }
    }
}
```

### Visual Feedback

```rust
// Update UI every frame
fn update_ability_ui(
    players: Query<&AbilitySet, With<Player>>,
    mut buttons: Query<(&AbilityButton, &mut Sprite)>,
    mut cooldown_bars: Query<(&CooldownBar, &mut Sprite, &mut Transform)>,
    mut texts: Query<(&AbilityButton, &mut Text2d)>,
) {
    // Update button colors (green = ready, gray = cooldown)
    // Update cooldown overlay bars (fill based on progress)
    // Update text labels (show timer or "READY")
}
```

## API Reference

### AbilitySlot

```rust
impl AbilitySlot {
    pub fn new(ability: Ability, cooldown_max: f32) -> Self
    pub fn is_ready(&self) -> bool
    pub fn use_ability(&mut self)
    pub fn tick(&mut self, delta: f32)
    pub fn cooldown_progress(&self) -> f32  // 0.0-1.0
}
```

### AbilitySet

```rust
impl AbilitySet {
    pub fn player_default() -> Self        // 4 abilities
    pub fn monster_default() -> Self       // 2 abilities
    pub fn tick_all(&mut self, delta: f32)
    pub fn get_ready_ability(&mut self, index: usize) -> Option<&mut AbilitySlot>
}
```

### Ability

```rust
impl Ability {
    pub fn basic_attack() -> Self       // power: 5
    pub fn powerful_attack() -> Self    // power: 12
    pub fn heal() -> Self               // power: -8
    pub fn quick_strike() -> Self       // power: 3
}
```

## Testing

Run tests:
```bash
cargo test -p bevy-wasm-fsharp-ref-logic
```

Test coverage:
- **75 total tests** (all passing)
- **23 cooldown tests** (lifecycle, ticking, ready state)
- **Healing tests** (negative damage, defense bypass)
- **Integration tests** (full combat sequences)

## File Organization

```
bevy-wasm-fsharp-ref/
├── fsharp/
│   └── Domain.fs                          # F# type definitions
├── crates/
│   ├── logic-fsharp/
│   │   ├── src/lib.rs                     # Rust logic implementation
│   │   └── tests/
│   │       ├── ability_cooldown_tests.rs  # Cooldown tests
│   │       └── combat_tests.rs            # Combat tests
│   └── app/
│       └── src/
│           ├── lib.rs                     # Bevy integration
│           └── cooldown_system.rs         # Standalone cooldown module
└── docs/
    ├── ability-system-implementation.md    # Implementation guide
    ├── ability-system-completion-report.md # Completion report
    └── ABILITY_SYSTEM_README.md            # This file
```

## Common Patterns

### Adding a New Ability

1. **Define in logic crate**:
```rust
impl Ability {
    pub fn new_ability() -> Self {
        Self {
            name: "New Ability".into(),
            power: 10,
        }
    }
}
```

2. **Add to AbilitySet**:
```rust
AbilitySlot::new(Ability::new_ability(), 2.0)
```

3. **Update UI** (automatic via `update_ability_ui`)

### Checking if Ability is Ready

```rust
if ability_set.abilities[index].is_ready() {
    // Can use ability
}
```

### Using an Ability

```rust
if let Some(slot) = ability_set.get_ready_ability(index) {
    let ability = slot.ability.clone();
    slot.use_ability();  // Trigger cooldown

    // Execute ability effect
    let event = compute_attack(&attacker, &target, &ability);
}
```

## Performance

- **Cooldown Ticking**: O(n) per frame, n = number of ability sets
- **UI Updates**: O(m) per frame, m = UI elements (constant)
- **No Allocations**: Uses query iteration only
- **Frame-Rate Independent**: Delta time-based cooldowns

## Troubleshooting

### Ability Not Working

1. **Check Cooldown**: Is `is_ready()` returning true?
2. **Check Index**: Is the ability index valid (0-3)?
3. **Check Input**: Are you pressing the correct key (1-4)?

### UI Not Updating

1. **Check System Order**: `update_ability_ui` must run after `tick_ability_cooldowns`
2. **Check Entity Query**: Does player entity have `AbilitySet` component?
3. **Check UI Spawning**: Was `spawn_ability_ui()` called in setup?

### Healing Not Working

1. **Check Power**: Is ability power negative?
2. **Check Target**: Healing targets self, not enemy
3. **Check HP Cap**: HP capped at `stats.hp` in application logic

## Future Enhancements

- [ ] Mouse click support for abilities
- [ ] Ability tooltips on hover
- [ ] Cooldown reduction buffs
- [ ] Ability upgrades/progression
- [ ] Combo system
- [ ] Status effects
- [ ] Mana/resource system
- [ ] Multi-target abilities
- [ ] Monster AI cooldown awareness
- [ ] Sound and particle effects

## Credits

- **Designed by**: beengud
- **Implementation**: Claude Code (AI Assistant)
- **Issue**: #15
- **Date**: 2025-11-14

## License

Same as parent project (bevy-wasm-fsharp-ref)
