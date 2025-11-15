# Troubleshooting Guide

This guide covers common issues you might encounter while working through the tutorial and building games with the F# → Rust → Bevy workflow.

## Common Compilation Errors

### Error: "cannot find type `Character` in this scope"

**Problem**: The Rust compiler can't find the Character type.

**Solution**: Ensure you're importing from the correct module:
```rust
use logic_fsharp::Character;  // If in the logic crate
// OR
use crate::components::Character;  // If in your game crate
```

### Error: "mismatched types: expected `i32`, found `f32`"

**Problem**: F# uses `int` which maps to `i32` in Rust, not `f32`.

**Solution**: Use explicit type conversions:
```rust
let damage = (base_damage as f32 * multiplier) as i32;
```

### Error: "the trait `Component` is not implemented"

**Problem**: Trying to use a type as a Bevy component without deriving Component.

**Solution**: Add the Component derive:
```rust
#[derive(Component)]
pub struct YourType {
    // fields
}
```

### Error: "cannot borrow `*query` as mutable more than once"

**Problem**: Rust's borrow checker preventing multiple mutable access.

**Solution**: Use `Query::iter_mut()` or split queries:
```rust
// Instead of getting multiple mutable references
let mut char1 = query.get_mut(entity1)?;
let mut char2 = query.get_mut(entity2)?; // ERROR!

// Use a single iteration
for mut character in &mut query {
    // Process all at once
}

// Or use ParamSet for exclusive access
fn system(mut queries: ParamSet<(Query<&mut Health>, Query<&mut Mana>)>) {
    queries.p0().iter_mut().for_each(|mut health| { /* ... */ });
    queries.p1().iter_mut().for_each(|mut mana| { /* ... */ });
}
```

## F# to Rust Translation Issues

### Issue: Option Types Not Matching

**F# Code**:
```fsharp
let getValue (x: int option) =
    match x with
    | Some v -> v
    | None -> 0
```

**Incorrect Rust**:
```rust
fn get_value(x: Option<i32>) -> i32 {
    match x {
        Some(v) => v,
        None => null, // ERROR!
    }
}
```

**Correct Rust**:
```rust
fn get_value(x: Option<i32>) -> i32 {
    match x {
        Some(v) => v,
        None => 0,  // Or use x.unwrap_or(0)
    }
}
```

### Issue: Record Updates

**F# Code**:
```fsharp
let updated = { character with Hp = 50 }
```

**Rust Equivalent**:
```rust
let updated = Character {
    hp: 50,
    ..character.clone()  // Don't forget clone() if not Copy
};
```

### Issue: Discriminated Union with Data

**F# Code**:
```fsharp
type Result = Victory of int | Defeat
```

**Incorrect Rust**:
```rust
enum Result {
    Victory(i32),
    Defeat(),  // ERROR: Empty tuple not needed
}
```

**Correct Rust**:
```rust
enum Result {
    Victory(i32),
    Defeat,  // No parentheses for unit variant
}
```

## Bevy-Specific Problems

### Issue: System Not Running

**Problem**: Your system isn't executing.

**Checklist**:
1. Is the system added to the app?
   ```rust
   app.add_systems(Update, your_system);
   ```

2. Check run conditions:
   ```rust
   app.add_systems(Update, your_system.run_if(in_state(GameState::Playing)));
   ```

3. Verify the query matches entities:
   ```rust
   fn your_system(query: Query<&Transform, With<Player>>) {
       if query.is_empty() {
           warn!("No entities match query!");
       }
   }
   ```

### Issue: Components Not Found

**Problem**: Query doesn't find expected components.

**Solution**: Check entity spawning:
```rust
// Make sure all components are added
commands.spawn((
    Transform::default(),
    GlobalTransform::default(),  // Often forgotten!
    YourComponent,
));
```

### Issue: Resources Not Available

**Problem**: `Res<T>` or `ResMut<T>` panics.

**Solution**: Initialize resources before use:
```rust
app.insert_resource(YourResource::default());
// OR
app.init_resource::<YourResource>();  // If implements Default
```

## WASM-Specific Issues

### Issue: WASM Build Fails

**Error**: "can't find crate for `std`"

**Solution**: Some dependencies don't support WASM. Use conditional compilation:
```toml
[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
native-only-crate = "1.0"
```

### Issue: Assets Not Loading in Browser

**Problem**: 404 errors for asset files.

**Solution**:
1. Assets must be in the `wasm/assets` directory
2. Use relative paths:
   ```rust
   asset_server.load("sprites/hero.png")  // Not "/sprites/hero.png"
   ```
3. Configure server for correct MIME types

### Issue: Performance Issues in Browser

**Problem**: Low FPS in WASM build.

**Solutions**:
1. Reduce texture sizes
2. Limit particle counts:
   ```rust
   #[cfg(target_arch = "wasm32")]
   const MAX_PARTICLES: usize = 100;
   #[cfg(not(target_arch = "wasm32"))]
   const MAX_PARTICLES: usize = 1000;
   ```
3. Use simpler shaders for web
4. Profile with browser DevTools

### Issue: Audio Not Playing

**Problem**: No sound in browser.

**Solution**: Browser autoplay policies require user interaction:
```javascript
document.addEventListener('click', () => {
    // Audio can now play
}, { once: true });
```

## Performance Issues

### Issue: Slow Compilation

**Solutions**:

1. Use dynamic linking for development:
   ```bash
   cargo run --features bevy/dynamic_linking
   ```

2. Use `mold` linker (Linux) or `lld` (Windows/Mac):
   ```toml
   # .cargo/config.toml
   [target.x86_64-unknown-linux-gnu]
   linker = "clang"
   rustflags = ["-C", "link-arg=-fuse-ld=mold"]
   ```

3. Enable incremental compilation:
   ```toml
   [profile.dev]
   incremental = true
   ```

### Issue: Runtime Performance

**Diagnostics**:
```rust
// Add diagnostic plugins
app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
   .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default());
```

**Common Fixes**:
1. Reduce query complexity
2. Use change detection:
   ```rust
   Query<&Transform, Changed<Transform>>
   ```
3. Batch operations
4. Use spatial partitioning for collision detection

## Type Alignment Issues

### Issue: F# and Rust Types Don't Match

**Problem**: Manually translated types have different layouts.

**Solution**: Keep types simple and verify with tests:
```rust
#[test]
fn test_type_sizes() {
    assert_eq!(std::mem::size_of::<Character>(), 48);  // Expected size
    assert_eq!(std::mem::align_of::<Character>(), 8);  // Expected alignment
}
```

### Issue: Serialization Failures

**Problem**: Can't serialize/deserialize between F# and Rust.

**Solution**: Use compatible serialization:
```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]  // Match F# naming
pub struct Character {
    pub name: String,
    pub hp: i32,
}
```

## Logic Errors

### Issue: Combat Damage Calculation Wrong

**Debug Steps**:
1. Add logging:
   ```rust
   debug!("Damage calc: {} - {} = {}", attack, defense, damage);
   ```

2. Write unit tests:
   ```rust
   #[test]
   fn test_damage_calculation() {
       let attacker = create_test_character(10, 0);
       let defender = create_test_character(0, 5);
       assert_eq!(calculate_damage(&attacker, &defender), 5);
   }
   ```

3. Compare with F# implementation:
   ```fsharp
   // Run in F# interactive to verify
   let damage = calculateDamage attacker defender
   printfn "Damage: %d" damage
   ```

### Issue: State Machine Stuck

**Problem**: Combat state doesn't transition.

**Debug**:
```rust
fn debug_state_transitions(
    state: Res<State<CombatState>>,
) {
    info!("Current state: {:?}", state.get());
}
```

**Common Causes**:
1. Missing `NextState::set()`
2. Condition never met
3. System not running in current state

## Environment Setup Issues

### Issue: "rustc: command not found"

**Solution**: Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Issue: F# Compilation Fails

**Solution**: Install .NET SDK:
```bash
# Verify installation
dotnet --version

# Should be 6.0 or later
```

### Issue: wasm-pack Not Found

**Solution**: Install wasm-pack:
```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

## Debugging Techniques

### Using Bevy Inspector

Add egui inspector for runtime debugging:
```rust
use bevy_inspector_egui::quick::WorldInspectorPlugin;

app.add_plugins(WorldInspectorPlugin::new());
```

### Logging

Configure log levels:
```rust
use bevy::log::LogPlugin;

app.add_plugins(DefaultPlugins.set(LogPlugin {
    level: bevy::log::Level::DEBUG,
    filter: "wgpu=error,bevy_render=info,bevy_ecs=debug".to_string(),
}));
```

### Conditional Compilation for Debug

```rust
#[cfg(debug_assertions)]
fn debug_system(query: Query<&Transform>) {
    for transform in &query {
        // Debug visualization
    }
}

#[cfg(debug_assertions)]
app.add_systems(Update, debug_system);
```

## Getting Help

### Where to Ask Questions

1. **Bevy Discord**: Real-time help for Bevy issues
2. **Rust Discord**: General Rust questions
3. **F# Slack**: F# domain modeling help
4. **Stack Overflow**: Tag with `bevy`, `rust`, or `f#`
5. **GitHub Issues**: For bugs in the reference implementation

### How to Ask Good Questions

1. **Minimal Example**: Reduce to smallest failing case
2. **Error Messages**: Include complete error output
3. **Environment**: Specify OS, Rust version, Bevy version
4. **What You've Tried**: List attempted solutions
5. **Expected vs Actual**: Clear description of the problem

### Example Good Question

```
Title: Bevy Query Fails to Find Character Component

Environment:
- Rust 1.75.0
- Bevy 0.14.0
- macOS 14.0

Problem:
Query<&Character> returns empty despite spawning entity with Character component.

Code:
```rust
// Spawning
commands.spawn(Character { ... });

// Querying (returns empty)
fn my_system(query: Query<&Character>) {
    println!("Count: {}", query.iter().count()); // Always 0
}
```

Expected: Query should find the spawned Character
Actual: Query is always empty

Tried:
1. Verified system is running
2. Added Name component for debugging
3. Used bevy-inspector-egui (entity exists but no Character component shown)
```

## Common Gotchas

1. **Forgetting `mut` in queries**: `Query<&Transform>` vs `Query<&mut Transform>`
2. **System ordering**: Some systems must run in specific order
3. **Resource initialization**: Resources must exist before systems use them
4. **WASM asset paths**: Different from native paths
5. **Component bundles**: Some components require others (e.g., Transform needs GlobalTransform)

## Prevention Tips

1. **Write tests early**: Catch issues before they compound
2. **Use type aliases**: Reduce translation errors
3. **Version control**: Commit working states frequently
4. **Incremental development**: Add features one at a time
5. **Documentation**: Document assumptions and decisions

---

Remember: Most issues have been encountered before. Don't hesitate to search existing discussions or ask the community!

[← Back to Tutorial Index](README.md)