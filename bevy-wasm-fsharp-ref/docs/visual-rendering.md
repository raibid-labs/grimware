# Visual Rendering Implementation

## Overview

This document describes the visual rendering implementation for player and monster entities in the bevy-wasm-fsharp-ref project.

## Implementation Details

### Visual Components

**Player Entity:**
- **Shape**: Square (60x60 pixels, representing a circle)
- **Color**: Blue (`Color::srgb(0.2, 0.4, 0.8)`)
- **Position**: (-100, 0, 0) - Left side of screen
- **Component**: `Sprite` with `custom_size`

**Monster Entity:**
- **Shape**: Rectangle (50x60 pixels)
- **Color**: Red (`Color::srgb(0.8, 0.2, 0.2)`)
- **Position**: (100, 0, 0) - Right side of screen
- **Component**: `Sprite` with `custom_size`

### Technical Approach

The implementation uses Bevy 0.15's `Sprite` component with custom sizing. This approach:

1. **Simple and Performant**: Uses Bevy's optimized sprite rendering
2. **No Asset Loading**: Colors are defined programmatically
3. **WASM Compatible**: Works in both native and web builds
4. **ECS Pattern**: Sprites are components attached to existing entities

### Code Structure

```rust
// Player visual representation
commands.spawn((
    Player,
    logic::Character::new_player("Hero"),
    Sprite {
        color: Color::srgb(0.2, 0.4, 0.8), // Blue
        custom_size: Some(Vec2::new(60.0, 60.0)),
        ..default()
    },
    Transform::from_xyz(-100.0, 0.0, 0.0),
));

// Monster visual representation
commands.spawn((
    Monster,
    logic::Character::new_monster("Slime"),
    Sprite {
        color: Color::srgb(0.8, 0.2, 0.2), // Red
        custom_size: Some(Vec2::new(50.0, 60.0)),
        ..default()
    },
    Transform::from_xyz(100.0, 0.0, 0.0),
));
```

## Bevy 0.15 Compatibility

The implementation uses the modern Bevy 0.15 API:
- `Sprite` component (not deprecated `MaterialMesh2dBundle`)
- `Camera2d` for orthographic 2D rendering
- `Transform` for positioning entities

## Future Enhancements

Potential improvements for visual rendering:

1. **Animations**: Add idle animations using sprite sheet or transform tweens
2. **Combat Feedback**: Visual effects for attacks (flash, shake, etc.)
3. **Health Bars**: Display HP above entities
4. **Texture Support**: Replace colored squares with sprite assets
5. **Particle Effects**: Add visual flair for combat events

## Testing

### Native Build
```bash
cargo run -p bevy-wasm-fsharp-ref-app
```

### WASM Build
```bash
# Build WASM
cargo build -p bevy-wasm-fsharp-ref-app --target wasm32-unknown-unknown --release

# Use wasm-bindgen
wasm-bindgen --out-dir web/pkg --target web \
  target/wasm32-unknown-unknown/release/app.wasm
```

## Integration with Other Systems

The visual rendering integrates with:
- **Combat System**: Entities remain visible during combat
- **Logic Crate**: `Character` component holds game state
- **Camera System**: 2D orthographic camera centers on combat area

## Dependencies

No additional dependencies required - uses only Bevy's built-in sprite rendering.

---

**Implementation completed for Issue #8: Visual & Rendering**
