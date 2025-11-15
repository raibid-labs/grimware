# Chapter 5: Bevy Integration

## What You'll Learn

In this chapter, you'll integrate your pure logic layer with the Bevy game engine. You'll learn about Entity Component Systems (ECS), create game systems, manage state, and connect everything to make an interactive game.

**Time Required**: 45 minutes

## Understanding Bevy's ECS

Bevy uses an Entity Component System (ECS) architecture:

- **Entities**: Unique IDs for game objects
- **Components**: Data attached to entities
- **Systems**: Functions that operate on components
- **Resources**: Global data shared across systems

```rust
// Entity: Just an ID
let player_entity = commands.spawn(...);

// Component: Data
#[derive(Component)]
struct Health(i32);

// System: Logic
fn damage_system(mut query: Query<&mut Health>) {
    for mut health in &mut query {
        health.0 -= 1;
    }
}

// Resource: Shared state
#[derive(Resource)]
struct GameState { turn: i32 }
```

## Step 1: Project Setup

### Dependencies

```toml
# Cargo.toml
[dependencies]
bevy = { version = "0.14", features = ["wayland", "mp3"] }
logic-fsharp = { path = "crates/logic-fsharp" }

[features]
default = []
dev = ["bevy/dynamic_linking"]  # Faster compilation

# Platform-specific
[target.'cfg(target_arch = "wasm32")'.dependencies]
wasm-bindgen = "0.2"
web-sys = "0.3"
```

### Main Structure

```rust
// src/main.rs

use bevy::prelude::*;
use logic_fsharp::{Character, Stats};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, combat_system)
        .run();
}
```

## Step 2: Components from Domain Types

### Bridging Logic and ECS

```rust
// src/components.rs

use bevy::prelude::*;
use logic_fsharp::{Character as LogicCharacter, Stats as LogicStats};

/// Component wrapping our logic Character
#[derive(Component)]
pub struct Character {
    pub data: LogicCharacter,
}

/// Visual representation component
#[derive(Component)]
pub struct CharacterSprite;

/// Marks the player entity
#[derive(Component)]
pub struct Player;

/// Marks enemy entities
#[derive(Component)]
pub struct Enemy;

/// Combat state for an entity
#[derive(Component)]
pub struct CombatStats {
    pub attack_cooldown: f32,
    pub is_defending: bool,
}

/// Display name above character
#[derive(Component)]
pub struct NameTag {
    pub text: String,
    pub offset: Vec2,
}
```

### Creating Entities from Logic

```rust
// src/spawning.rs

use bevy::prelude::*;
use crate::components::*;
use logic_fsharp;

pub fn spawn_hero(
    commands: &mut Commands,
    position: Vec3,
) -> Entity {
    let hero_data = logic_fsharp::create_hero();

    commands
        .spawn((
            Character { data: hero_data.clone() },
            Player,
            CharacterSprite,
            Transform::from_translation(position),
            GlobalTransform::default(),
            NameTag {
                text: hero_data.name.clone(),
                offset: Vec2::new(0.0, 50.0),
            },
            CombatStats {
                attack_cooldown: 0.0,
                is_defending: false,
            },
        ))
        .id()
}

pub fn spawn_enemy(
    commands: &mut Commands,
    enemy_data: LogicCharacter,
    position: Vec3,
) -> Entity {
    commands
        .spawn((
            Character { data: enemy_data.clone() },
            Enemy,
            CharacterSprite,
            Transform::from_translation(position),
            GlobalTransform::default(),
            NameTag {
                text: enemy_data.name.clone(),
                offset: Vec2::new(0.0, 50.0),
            },
            CombatStats {
                attack_cooldown: 0.0,
                is_defending: false,
            },
        ))
        .id()
}
```

## Step 3: Game State Management

### State Machine

```rust
// src/states.rs

use bevy::prelude::*;

/// Main game states
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

/// Combat sub-states
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum CombatState {
    #[default]
    NotInCombat,
    PlayerTurn,
    EnemyTurn,
    ProcessingAction,
    Victory,
    Defeat,
}

/// Turn-based combat resource
#[derive(Resource)]
pub struct TurnManager {
    pub current_turn: usize,
    pub turn_order: Vec<Entity>,
    pub action_queue: Vec<CombatAction>,
}

#[derive(Clone, Debug)]
pub enum CombatAction {
    Attack { attacker: Entity, target: Entity },
    Defend { entity: Entity },
    UseAbility { user: Entity, ability_id: String, targets: Vec<Entity> },
    Flee { entity: Entity },
}
```

### State Transitions

```rust
pub fn manage_combat_states(
    current_state: Res<State<CombatState>>,
    mut next_state: ResMut<NextState<CombatState>>,
    mut turn_manager: ResMut<TurnManager>,
    player_query: Query<&Character, With<Player>>,
    enemy_query: Query<&Character, With<Enemy>>,
) {
    match current_state.get() {
        CombatState::PlayerTurn => {
            // Wait for player input
            // Transition to ProcessingAction when action selected
        }
        CombatState::ProcessingAction => {
            // Process queued actions
            if turn_manager.action_queue.is_empty() {
                // Move to next turn
                turn_manager.current_turn += 1;
                let next_entity = turn_manager.turn_order
                    [turn_manager.current_turn % turn_manager.turn_order.len()];

                if player_query.contains(next_entity) {
                    next_state.set(CombatState::PlayerTurn);
                } else {
                    next_state.set(CombatState::EnemyTurn);
                }
            }
        }
        CombatState::EnemyTurn => {
            // AI makes decision
            // Transition to ProcessingAction
        }
        _ => {}
    }
}
```

## Step 4: Combat Systems

### Input Handling

```rust
// src/input.rs

use bevy::prelude::*;
use crate::states::{CombatState, CombatAction, TurnManager};

pub fn handle_combat_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    combat_state: Res<State<CombatState>>,
    mut turn_manager: ResMut<TurnManager>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    // Only process input during player turn
    if *combat_state.get() != CombatState::PlayerTurn {
        return;
    }

    let player = player_query.single();
    let enemy = enemy_query.iter().next();

    if keyboard.just_pressed(KeyCode::KeyA) {
        // Attack
        if let Some(target) = enemy {
            turn_manager.action_queue.push(CombatAction::Attack {
                attacker: player,
                target,
            });
            next_state.set(CombatState::ProcessingAction);
        }
    } else if keyboard.just_pressed(KeyCode::KeyD) {
        // Defend
        turn_manager.action_queue.push(CombatAction::Defend {
            entity: player,
        });
        next_state.set(CombatState::ProcessingAction);
    } else if keyboard.just_pressed(KeyCode::KeyR) {
        // Run
        turn_manager.action_queue.push(CombatAction::Flee {
            entity: player,
        });
        next_state.set(CombatState::ProcessingAction);
    }
}
```

### Combat Processing

```rust
// src/combat.rs

use bevy::prelude::*;
use logic_fsharp::combat::{calculate_damage, apply_damage};
use crate::components::Character;
use crate::states::{CombatAction, TurnManager};

pub fn process_combat_actions(
    mut turn_manager: ResMut<TurnManager>,
    mut character_query: Query<&mut Character>,
    mut combat_log: ResMut<CombatLog>,
) {
    while let Some(action) = turn_manager.action_queue.pop() {
        match action {
            CombatAction::Attack { attacker, target } => {
                // Get attacker and target data
                let attacker_data = character_query
                    .get(attacker)
                    .map(|c| c.data.clone())
                    .ok();

                let target_data = character_query
                    .get(target)
                    .map(|c| c.data.clone())
                    .ok();

                if let (Some(atk_data), Some(def_data)) = (attacker_data, target_data) {
                    // Calculate damage using pure logic
                    let damage = calculate_damage(&atk_data, &def_data);

                    // Apply damage and get updated character
                    let updated_defender = apply_damage(&def_data, damage);

                    // Update ECS component
                    if let Ok(mut target_char) = character_query.get_mut(target) {
                        target_char.data = updated_defender;
                    }

                    // Log the action
                    combat_log.add(format!(
                        "{} attacks {} for {} damage!",
                        atk_data.name, def_data.name, damage
                    ));
                }
            }
            CombatAction::Defend { entity } => {
                if let Ok(character) = character_query.get(entity) {
                    combat_log.add(format!("{} defends!", character.data.name));
                    // Apply defense buff for next turn
                }
            }
            _ => {}
        }
    }
}
```

### Visual Feedback

```rust
// src/visual.rs

use bevy::prelude::*;
use crate::components::{Character, NameTag};

/// Update health bars based on character data
pub fn update_health_bars(
    mut query: Query<(&Character, &mut HealthBar)>,
) {
    for (character, mut health_bar) in &mut query {
        let percentage = character.data.current_hp as f32 / character.data.stats.hp as f32;
        health_bar.percentage = percentage;

        // Change color based on health
        health_bar.color = if percentage > 0.6 {
            Color::GREEN
        } else if percentage > 0.3 {
            Color::YELLOW
        } else {
            Color::RED
        };
    }
}

/// Floating damage numbers
pub fn spawn_damage_text(
    commands: &mut Commands,
    position: Vec3,
    damage: i32,
    is_critical: bool,
) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("{}", damage),
                TextStyle {
                    font_size: if is_critical { 40.0 } else { 30.0 },
                    color: if is_critical { Color::YELLOW } else { Color::WHITE },
                    ..default()
                },
            ),
            transform: Transform::from_translation(position),
            ..default()
        },
        DamageText {
            velocity: Vec3::new(0.0, 100.0, 0.0),
            lifetime: Timer::from_seconds(1.0, TimerMode::Once),
        },
    ));
}

#[derive(Component)]
struct DamageText {
    velocity: Vec3,
    lifetime: Timer,
}

pub fn animate_damage_text(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut DamageText)>,
) {
    for (entity, mut transform, mut damage_text) in &mut query {
        // Move upward and fade
        transform.translation += damage_text.velocity * time.delta_seconds();
        damage_text.lifetime.tick(time.delta());

        if damage_text.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

## Step 5: Resource Management

### Combat Log

```rust
#[derive(Resource, Default)]
pub struct CombatLog {
    entries: Vec<String>,
    max_entries: usize,
}

impl CombatLog {
    pub fn new(max_entries: usize) -> Self {
        CombatLog {
            entries: Vec::new(),
            max_entries,
        }
    }

    pub fn add(&mut self, message: String) {
        self.entries.push(message);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn get_recent(&self, count: usize) -> &[String] {
        let start = self.entries.len().saturating_sub(count);
        &self.entries[start..]
    }
}
```

### Asset Management

```rust
#[derive(Resource)]
pub struct GameAssets {
    pub hero_texture: Handle<Image>,
    pub enemy_textures: Vec<Handle<Image>>,
    pub ui_font: Handle<Font>,
    pub battle_music: Handle<AudioSource>,
    pub attack_sound: Handle<AudioSource>,
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameAssets {
        hero_texture: asset_server.load("textures/hero.png"),
        enemy_textures: vec![
            asset_server.load("textures/slime.png"),
            asset_server.load("textures/goblin.png"),
        ],
        ui_font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        battle_music: asset_server.load("audio/battle.mp3"),
        attack_sound: asset_server.load("audio/sword_hit.wav"),
    });
}
```

## Step 6: Complete Game Setup

### Main App Configuration

```rust
// src/main.rs

mod combat;
mod components;
mod input;
mod spawning;
mod states;
mod visual;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "F# Combat Game".to_string(),
                resolution: (1280., 720.).into(),
                canvas: Some("#game-canvas".to_string()), // For WASM
                ..default()
            }),
            ..default()
        }))

        // States
        .init_state::<states::GameState>()
        .init_state::<states::CombatState>()

        // Resources
        .insert_resource(states::TurnManager::default())
        .insert_resource(CombatLog::new(10))

        // Setup systems (run once)
        .add_systems(Startup, (
            setup_camera,
            setup_battle_scene,
            load_assets,
        ))

        // Game state systems
        .add_systems(
            Update,
            (
                input::handle_combat_input,
                states::manage_combat_states,
            )
                .run_if(in_state(states::GameState::Playing))
        )

        // Combat systems
        .add_systems(
            Update,
            (
                combat::process_combat_actions,
                visual::update_health_bars,
                visual::animate_damage_text,
                ui::update_combat_log,
            )
                .chain()
                .run_if(in_state(states::CombatState::ProcessingAction))
        )

        // AI system
        .add_systems(
            Update,
            ai::enemy_ai_decision
                .run_if(in_state(states::CombatState::EnemyTurn))
        )

        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_battle_scene(
    mut commands: Commands,
    mut next_game_state: ResMut<NextState<states::GameState>>,
) {
    // Spawn hero
    spawning::spawn_hero(&mut commands, Vec3::new(-200.0, 0.0, 0.0));

    // Spawn enemy
    let slime = logic_fsharp::create_slime();
    spawning::spawn_enemy(&mut commands, slime, Vec3::new(200.0, 0.0, 0.0));

    // Start game
    next_game_state.set(states::GameState::Playing);
}
```

## Step 7: AI System

```rust
// src/ai.rs

use bevy::prelude::*;
use crate::components::{Character, Enemy};
use crate::states::{CombatState, CombatAction, TurnManager};

pub fn enemy_ai_decision(
    enemy_query: Query<(Entity, &Character), With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
    mut turn_manager: ResMut<TurnManager>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    for (enemy_entity, enemy_char) in &enemy_query {
        // Simple AI: Attack if healthy, defend if low
        let health_percentage = enemy_char.data.current_hp as f32
            / enemy_char.data.stats.hp as f32;

        let action = if health_percentage < 0.3 {
            // Low health - defend
            CombatAction::Defend { entity: enemy_entity }
        } else {
            // Attack player
            if let Ok(player) = player_query.get_single() {
                CombatAction::Attack {
                    attacker: enemy_entity,
                    target: player,
                }
            } else {
                continue;
            }
        };

        turn_manager.action_queue.push(action);
    }

    next_state.set(CombatState::ProcessingAction);
}
```

## Common Patterns

### Query Filters

```rust
// Multiple component requirements
fn system(query: Query<(&Transform, &Health), With<Player>>) {}

// Excluding components
fn system(query: Query<&Transform, Without<Player>>) {}

// Changed detection
fn system(query: Query<&Health, Changed<Health>>) {}

// Optional components
fn system(query: Query<(&Transform, Option<&Health>)>) {}
```

### Event System

```rust
#[derive(Event)]
struct DamageEvent {
    entity: Entity,
    amount: i32,
}

fn send_damage(mut events: EventWriter<DamageEvent>) {
    events.send(DamageEvent {
        entity: Entity::from_raw(0),
        amount: 10,
    });
}

fn receive_damage(mut events: EventReader<DamageEvent>) {
    for event in events.read() {
        println!("Entity {:?} took {} damage", event.entity, event.amount);
    }
}
```

### System Ordering

```rust
app.add_systems(
    Update,
    (
        first_system,
        second_system,
        third_system,
    )
        .chain()  // Run in order
);

app.add_systems(
    Update,
    system_a.before(system_b)
);
```

## Exercises

### Exercise 1: Add Particle Effects

Create a particle system for attacks:

```rust
#[derive(Component)]
struct Particle {
    velocity: Vec3,
    lifetime: Timer,
    color: Color,
}

fn spawn_attack_particles(position: Vec3) {
    // Spawn multiple particles
    // Give them random velocities
    // Fade over time
}
```

### Exercise 2: Implement Abilities UI

Create an ability bar:

```rust
fn create_ability_bar(
    commands: &mut Commands,
    abilities: Vec<Ability>,
) {
    // Create UI nodes for each ability
    // Show cooldowns
    // Handle clicks/hotkeys
}
```

### Exercise 3: Add Combat Animations

Implement sprite animations:

```rust
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    // Update sprite index based on timer
}
```

## Performance Tips

1. **Use `Changed<T>` filters** to avoid unnecessary work
2. **Batch similar operations** in single systems
3. **Use `Commands` for deferred operations**
4. **Minimize queries in hot loops**
5. **Consider using `ParallelCommands` for multi-threading**

## Summary

You've successfully integrated your logic layer with Bevy:

✅ Created components from domain types
✅ Built game systems for combat
✅ Managed state with state machines
✅ Handled player input
✅ Added visual feedback
✅ Implemented basic AI

## Next Steps

In the next chapter, we'll build a complete combat system with turns, abilities, and effects.

[Next: Combat System →](06-combat-system.md)

[← Previous: Rust Implementation](04-rust-implementation.md) | [Tutorial Index](README.md)