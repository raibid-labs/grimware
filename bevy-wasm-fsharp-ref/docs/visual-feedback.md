# Visual Combat Feedback System

This document describes the visual feedback system implemented for the combat game, providing real-time visual indicators for all combat events.

## Overview

The visual feedback system enhances the combat experience by providing:
- **Health bars**: Real-time HP display above each character
- **Damage numbers**: Floating numbers that show damage dealt
- **Attack animations**: Visual flash on the attacker
- **Hit effects**: Shake animation on the defender
- **Game over messages**: Victory/defeat text at game end

## Components

### Health Bar System

**Components:**
- `HealthBar`: Container marking health bar ownership
- `HealthBarBackground`: Red background showing max HP
- `HealthBarForeground`: Green foreground showing current HP
- `HealthText`: Text display showing "X / Y" HP

**Layout:**
```
    30 / 30  <-- Health text (white)
  [████████]  <-- Health bar (green on red)
     ▲
     │
  Character
```

**Features:**
- Background (red): Fixed width of 80px
- Foreground (green): Shrinks proportionally to HP
- Text: Updates to show current/max HP
- Position: 50px above character center

### Damage Numbers

**Component:** `DamageNumber`

**Behavior:**
- Spawns at target position + (0, 30, 10)
- Rises upward at 50px/second
- Fades out over 1 second using alpha blending
- Automatically despawns when animation completes

**Colors:**
- Player attacking monster: Red (1.0, 0.3, 0.3)
- Monster attacking player: Orange (1.0, 0.6, 0.0)

**Font:**
- Size: 32px
- Style: Bold, centered

### Attack Flash Effect

**Component:** `AttackFlash`

**Behavior:**
- Added to attacker entity when attack executes
- Flashes sprite to white over 0.15 seconds
- Uses color mixing: `original.mix(WHITE, 1.0 - progress)`
- Restores original color and removes component when complete

**Implementation:**
```rust
commands.entity(attacker_entity).insert(AttackFlash {
    timer: Timer::from_seconds(0.15, TimerMode::Once),
    original_color: attacker_sprite.color,
});
```

### Hit Shake Effect

**Component:** `HitShake`

**Behavior:**
- Added to defender entity when hit
- Shakes sprite horizontally using sine wave
- Duration: 0.3 seconds
- Intensity: 5.0 pixels (decays over time)
- Frequency: 20 oscillations per second

**Formula:**
```rust
let decay = 1.0 - progress;
let shake_amount = intensity * decay;
let offset = (progress * 20.0 * 2π).sin() * shake_amount;
position.x = original_x + offset;
```

### Game Over Message

**Component:** `GameOverMessage`

**Display:**
- Text: "VICTORY!" or "DEFEAT!"
- Font size: 72px
- Position: Center screen, (0, 100, 100)
- Color:
  - Victory: Green (0.2, 1.0, 0.2)
  - Defeat: Red (1.0, 0.2, 0.2)

**Spawn condition:**
- Only spawned once when CombatState changes to GameOver
- Checks for existing message to prevent duplicates

## Systems

### `update_health_bars`

Updates health bar foreground width and text based on character HP.

**Query pattern:**
```rust
characters: Query<(Entity, &Character)>
health_bars: Query<(&HealthBarForeground, &mut Sprite, &mut Transform)>
health_texts: Query<(&HealthText, &mut Text2d)>
```

**Logic:**
1. For each character, find matching health bar by `owner` entity
2. Calculate HP ratio: `current_hp / max_hp`
3. Update sprite width: `80.0 * hp_ratio`
4. Adjust position to keep left-aligned
5. Update text: `"{current} / {max}"`

### `animate_damage_numbers`

Animates floating damage numbers with rise and fade effects.

**Per frame:**
1. Tick lifetime timer
2. Move upward: `y += rise_speed * delta_time`
3. Calculate alpha: `1.0 - timer.fraction()`
4. Apply alpha to color
5. Despawn when timer finishes

### `animate_attack_flash`

Animates attacker flash effect.

**Per frame:**
1. Tick timer
2. Calculate progress: `timer.fraction()`
3. Mix colors: `original.mix(WHITE, 1.0 - progress)`
4. When finished: restore original color, remove component

### `animate_hit_shake`

Animates defender shake effect using sine wave.

**Per frame:**
1. Tick timer
2. Calculate shake offset using sine wave
3. Apply decaying intensity
4. When finished: restore original position, remove component

## Integration

### Player Turn Handler

When player attacks:
```rust
// Spawn damage number at monster position
spawn_damage_number(
    &mut commands,
    damage,
    monster_transform.translation + Vec3::new(0.0, 30.0, 10.0),
    Color::srgb(1.0, 0.3, 0.3), // Red
);

// Flash player (attacker)
commands.entity(player_entity).insert(AttackFlash { ... });

// Shake monster (defender)
commands.entity(monster_entity).insert(HitShake { ... });
```

### Monster Turn Handler

Similar to player turn but with reversed roles and orange damage numbers.

### Game Over Handler

```rust
if let CombatState::GameOver { winner } = &*combat_state {
    let message_text = if winner == "Hero" { "VICTORY!" } else { "DEFEAT!" };
    let message_color = if winner == "Hero" { GREEN } else { RED };

    commands.spawn((
        GameOverMessage,
        Text2d::new(message_text),
        // ... styling
    ));
}
```

## Performance Considerations

**Entity counts:**
- Health bars: 2 backgrounds + 2 foregrounds + 2 text = 6 entities (constant)
- Damage numbers: 1-2 entities (short-lived, 1 second lifespan)
- Flash/shake effects: Temporary components (0.15-0.3 seconds)
- Game over message: 1 entity (spawned once at game end)

**Update frequency:**
- Health bars: Every frame, but simple calculations
- Damage numbers: Only when active (1-2 per turn)
- Animations: Only entities with components

**Memory footprint:**
- Minimal - all components are small structs
- Damage numbers despawn automatically
- Animation components removed when finished

## Customization

### Adjusting Damage Number Speed

In `spawn_damage_number`:
```rust
rise_speed: 50.0, // Change this value (pixels/second)
```

### Changing Flash Duration

In attack handlers:
```rust
Timer::from_seconds(0.15, TimerMode::Once), // Adjust duration
```

### Modifying Shake Intensity

In attack handlers:
```rust
intensity: 5.0, // Adjust shake distance in pixels
```

In `animate_hit_shake`:
```rust
let frequency = 20.0; // Adjust oscillation speed
```

### Customizing Colors

Health bars:
```rust
background: Color::srgb(0.6, 0.1, 0.1), // Dark red
foreground: Color::srgb(0.1, 0.8, 0.1), // Green
```

Damage numbers:
```rust
player_damage: Color::srgb(1.0, 0.3, 0.3), // Red
monster_damage: Color::srgb(1.0, 0.6, 0.0), // Orange
```

Game over messages:
```rust
victory: Color::srgb(0.2, 1.0, 0.2), // Green
defeat: Color::srgb(1.0, 0.2, 0.2), // Red
```

## Testing

### Native Build

```bash
cargo run -p app
```

Press SPACE to attack and observe:
- Health bars update
- Damage numbers float and fade
- Attacker flashes white
- Defender shakes
- Game over message displays

### WASM Build

```bash
just build-wasm
just serve-wasm
```

Open browser and test same behaviors.

## Future Enhancements

Potential improvements:
- **Status effect indicators**: Buffs/debuffs above health bars
- **Critical hit effects**: Larger damage numbers, screen shake
- **Heal effects**: Green numbers floating up, particle effects
- **Death animations**: Fade out defeated characters
- **Sound effects**: Audio feedback for combat events
- **Combo counters**: Track consecutive hits
- **Ability icons**: Show which ability was used

## Architecture Notes

**Separation of concerns:**
- Combat logic (logic crate): Pure functions, no visuals
- Visual feedback (app crate): UI components and animations
- Clear boundary: CombatEvent → Visual effects

**ECS patterns:**
- Components for state (HealthBar, DamageNumber)
- Systems for behavior (update, animate)
- Commands for spawning/despawning
- Queries for filtering entities

**Animation approach:**
- Timer-based: Consistent across frame rates
- Component-based: Clean add/remove lifecycle
- Delta-time aware: Smooth on all hardware

## Related Files

- `/crates/app/src/lib.rs`: All visual feedback code
- `/crates/logic-fsharp/src/lib.rs`: Combat logic (no visuals)
- `/docs/bevy-mcp.md`: Bevy integration patterns
- `/docs/getting-started.md`: Setup and running

---

**Visual feedback system complete. All combat events now have clear, animated visual indicators.**
