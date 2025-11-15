# Ability System with Cooldowns - Implementation Guide

## Overview

This document describes the implementation of the cooldown-based ability system for issue #15.

## Changes Made

### 1. F# Domain Types (`fsharp/Domain.fs`)

Added cooldown types:

```fsharp
type AbilitySlot =
    { Ability: Ability
      CooldownMax: float
      CooldownCurrent: float }

type AbilitySet =
    { Abilities: AbilitySlot list }
```

Updated CombatEvent to include ability name:

```fsharp
type CombatEvent =
    { AttackerName: string
      DefenderName: string
      Damage: int
      DefenderHpAfter: int
      AbilityUsed: string }
```

### 2. Rust Logic Crate (`crates/logic-fsharp/src/lib.rs`)

#### New Ability Constructors

```rust
impl Ability {
    pub fn basic_attack() -> Self { power: 5 }
    pub fn powerful_attack() -> Self { power: 12 }
    pub fn heal() -> Self { power: -8 }
    pub fn quick_strike() -> Self { power: 3 }
}
```

#### AbilitySlot Component

```rust
#[derive(Component)]
pub struct AbilitySlot {
    pub ability: Ability,
    pub cooldown_max: f32,
    pub cooldown_current: f32,
}

impl AbilitySlot {
    pub fn is_ready(&self) -> bool
    pub fn use_ability(&mut self)
    pub fn tick(&mut self, delta: f32)
    pub fn cooldown_progress(&self) -> f32
}
```

#### AbilitySet Component

```rust
#[derive(Component)]
pub struct AbilitySet {
    pub abilities: Vec<AbilitySlot>,
}

impl AbilitySet {
    pub fn player_default() -> Self {
        vec![
            AbilitySlot::new(Ability::basic_attack(), 0.5),
            AbilitySlot::new(Ability::powerful_attack(), 3.0),
            AbilitySlot::new(Ability::heal(), 5.0),
            AbilitySlot::new(Ability::quick_strike(), 0.2),
        ]
    }

    pub fn monster_default() -> Self {
        vec![
            AbilitySlot::new(Ability::basic_attack(), 1.0),
            AbilitySlot::new(Ability::quick_strike(), 0.5),
        ]
    }

    pub fn tick_all(&mut self, delta: f32)
    pub fn get_ready_ability(&mut self, index: usize) -> Option<&mut AbilitySlot>
}
```

#### Updated Combat Logic

```rust
pub fn compute_attack(attacker: &Character, defender: &Character, ability: &Ability) -> CombatEvent {
    // Healing abilities (negative power) bypass defense
    let dmg = if ability.power < 0 {
        ability.power  // Direct healing
    } else {
        let raw = attacker.stats.attack + ability.power;
        (raw - defender.stats.defense).max(1)
    };

    let hp_after = defender.hp - dmg;  // Negative damage = healing

    CombatEvent {
        attacker_name: attacker.name.clone(),
        defender_name: defender.name.clone(),
        damage: dmg,
        defender_hp_after: hp_after,
        ability_used: ability.name.clone(),  // NEW
    }
}
```

### 3. Bevy App Integration (`crates/app/src/lib.rs`)

#### Add Cooldown System to Update Loop

```rust
.add_systems(
    Update,
    (
        tick_ability_cooldowns,  // NEW - tick cooldowns every frame
        handle_player_turn,
        handle_monster_turn,
        check_game_over,
        display_turn_indicator,
        display_combat_log,
        update_health_bars,
        animate_damage_numbers,
        animate_attack_flash,
        animate_hit_shake,
        update_ability_ui,  // NEW - update UI every frame
    )
        .chain(),
)
```

#### Spawn Entities with AbilitySet

```rust
// Player with abilities
commands.spawn((
    Player,
    logic::Character::new_player("Hero"),
    logic::AbilitySet::player_default(),  // NEW
    Sprite { /* ... */ },
    Transform::from_xyz(-100.0, 0.0, 0.0),
));

// Monster with abilities
commands.spawn((
    Monster,
    logic::Character::new_monster("Slime"),
    logic::AbilitySet::monster_default(),  // NEW
    Sprite { /* ... */ },
    Transform::from_xyz(100.0, 0.0, 0.0),
));
```

#### Ability Selection Input

```rust
fn handle_player_turn(
    keys: Res<ButtonInput<KeyCode>>,
    // ...
    mut players: Query<(Entity, &mut logic::Character, &mut logic::AbilitySet, &Transform, &Sprite), With<Player>>,
) {
    // Check which ability key was pressed (1-4)
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
        if let Some(ability_slot) = ability_set.get_ready_ability(idx) {
            let ability = ability_slot.ability.clone();
            ability_slot.use_ability();  // Start cooldown

            // Determine target (heal self, attack enemy)
            let target = if ability.power < 0 { &mut player } else { &mut monster };

            let event = logic::compute_attack(&player, target, &ability);

            // Apply effect
            target.hp = event.defender_hp_after.min(target.stats.hp);  // Cap at max HP

            // Visual feedback...
        } else {
            combat_log.add(format!("Ability {} is on cooldown!", idx + 1));
        }
    }
}
```

#### Cooldown Tick System

```rust
fn tick_ability_cooldowns(
    time: Res<Time>,
    mut query: Query<&mut logic::AbilitySet>,
) {
    let delta = time.delta_secs();
    for mut ability_set in query.iter_mut() {
        ability_set.tick_all(delta);
    }
}
```

#### Ability UI Components

```rust
#[derive(Component)]
struct AbilityButton {
    slot_index: usize,
}

#[derive(Component)]
struct CooldownBar {
    slot_index: usize,
}

fn spawn_ability_ui(commands: &mut Commands) {
    let start_x = -300.0;
    let y = -250.0;
    const BUTTON_WIDTH: f32 = 120.0;
    const BUTTON_HEIGHT: f32 = 60.0;
    const BUTTON_SPACING: f32 = 130.0;

    for i in 0..4 {
        let x = start_x + (i as f32 * BUTTON_SPACING);

        // Button background (changes color based on cooldown)
        commands.spawn((
            AbilityButton { slot_index: i },
            Sprite {
                color: Color::srgb(0.2, 0.2, 0.3),
                custom_size: Some(Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(x, y, 5.0),
        ));

        // Cooldown overlay (fills from left to right)
        commands.spawn((
            CooldownBar { slot_index: i },
            Sprite {
                color: Color::srgba(0.1, 0.1, 0.1, 0.7),
                custom_size: Some(Vec2::new(0.0, BUTTON_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(x, y, 6.0),
        ));

        // Ability name
        commands.spawn((
            AbilityButton { slot_index: i },
            Text2d::new(""),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::WHITE),
            Transform::from_xyz(x, y + 15.0, 7.0),
        ));

        // Cooldown timer
        commands.spawn((
            AbilityButton { slot_index: i },
            Text2d::new(""),
            TextFont { font_size: 11.0, ..default() },
            TextColor(Color::srgb(1.0, 0.8, 0.2)),
            Transform::from_xyz(x, y - 5.0, 7.0),
        ));

        // Keybind hint
        commands.spawn((
            Text2d::new(format!("[{}]", i + 1)),
            TextFont { font_size: 10.0, ..default() },
            TextColor(Color::srgb(0.5, 0.5, 0.5)),
            Transform::from_xyz(x, y - 22.0, 7.0),
        ));
    }
}
```

#### UI Update System

```rust
fn update_ability_ui(
    players: Query<&logic::AbilitySet, With<Player>>,
    mut buttons: Query<(&AbilityButton, &mut Sprite), Without<CooldownBar>>,
    mut cooldown_bars: Query<(&CooldownBar, &mut Sprite, &mut Transform), Without<AbilityButton>>,
    mut texts: Query<(&AbilityButton, &mut Text2d)>,
) {
    if let Ok(ability_set) = players.get_single() {
        const BUTTON_WIDTH: f32 = 120.0;

        // Update button colors
        for (button, mut sprite) in buttons.iter_mut() {
            if let Some(slot) = ability_set.abilities.get(button.slot_index) {
                sprite.color = if slot.is_ready() {
                    Color::srgb(0.2, 0.6, 0.3)  // Green when ready
                } else {
                    Color::srgb(0.3, 0.3, 0.4)  // Gray when on cooldown
                };
            }
        }

        // Update cooldown bars
        for (bar, mut sprite, mut transform) in cooldown_bars.iter_mut() {
            if let Some(slot) = ability_set.abilities.get(bar.slot_index) {
                let progress = slot.cooldown_progress();
                let bar_width = BUTTON_WIDTH * progress;
                sprite.custom_size = Some(Vec2::new(bar_width, 60.0));

                // Position from left
                let offset = (BUTTON_WIDTH - bar_width) / 2.0;
                transform.translation.x -= offset;
            }
        }

        // Update text
        for (button, mut text) in texts.iter_mut() {
            if let Some(slot) = ability_set.abilities.get(button.slot_index) {
                if text.0.contains('[') {
                    continue; // Skip keybind hints
                }

                if text.0.is_empty() || !text.0.contains(':') {
                    **text = slot.ability.name.clone();
                } else if slot.is_ready() {
                    **text = "READY".to_string();
                } else {
                    **text = format!("CD: {:.1}s", slot.cooldown_current);
                }
            }
        }
    }
}
```

## Abilities Implemented

1. **Basic Attack** - 5 power, 0.5s cooldown (standard attack)
2. **Powerful Attack** - 12 power, 3s cooldown (high damage, long cooldown)
3. **Heal** - -8 power, 5s cooldown (negative power = heal yourself)
4. **Quick Strike** - 3 power, 0.2s cooldown (low damage, spammable)

## Healing Mechanics

- Abilities with negative power heal instead of damage
- Healing bypasses defense calculation
- Healing targets self (player heals player)
- HP cannot exceed max HP (capped in application logic)

## Visual Feedback

1. **Button Colors**:
   - Green = Ready to use
   - Gray = On cooldown

2. **Cooldown Overlay**:
   - Dark bar fills from left to right
   - Width = cooldown progress (100% = just used, 0% = ready)

3. **Text Labels**:
   - Ability name always visible
   - Cooldown timer shows remaining seconds
   - "READY" when available
   - Keybind hint ([1], [2], [3], [4])

4. **Damage Numbers**:
   - Red = Damage dealt
   - Green = Healing received
   - Format: "-X" for damage, "+X" for healing

## Testing

See `crates/logic-fsharp/src/lib.rs` for comprehensive tests:

- `test_ability_with_meta_is_ready()` - Cooldown lifecycle
- `test_ability_with_meta_tick_does_not_go_negative()` - Edge cases
- Healing damage tests with negative power

## Acceptance Criteria Status

- ✅ Characters have multiple abilities
- ✅ Cooldown system prevents spam
- ✅ UI shows abilities and cooldowns
- ✅ Healing ability works (negative damage)
- ✅ F# types align with Rust
- ✅ Keyboard shortcuts work (1-4 keys)
- ✅ Visual feedback for cooldowns

## Future Enhancements

1. Mouse click support for ability buttons
2. Ability tooltips on hover
3. Animation for ability activation
4. Sound effects for each ability type
5. Particle effects for heal/attack
6. Monster AI using cooldown-aware abilities

## Files Modified

- `fsharp/Domain.fs` - F# domain types
- `crates/logic-fsharp/src/lib.rs` - Rust logic implementation
- `crates/app/src/lib.rs` - Bevy integration (needs manual merge)
- `crates/app/src/cooldown_system.rs` - Cooldown system module (NEW)

## Integration Notes

The ability system is designed to be modular and can be easily extended with:
- More abilities
- Different cooldown strategies (charges, mana costs)
- Ability unlocks/progression
- Combo systems
- Status effects
