# Chapter 6: Combat System Implementation

## What You'll Learn

In this chapter, you'll implement a complete turn-based combat system. You'll create state machines, handle combat events, add visual feedback, and build a polished battle experience.

**Time Required**: 45 minutes

## Combat System Architecture

Our combat system consists of:

1. **Turn Management**: Order and timing
2. **Action Queue**: Player and AI decisions
3. **State Machine**: Battle flow control
4. **Event System**: Combat log and feedback
5. **Visual Effects**: Animations and UI

## Step 1: Turn-Based Flow

### Turn Order System

```rust
// src/combat/turn_order.rs

use bevy::prelude::*;
use crate::components::{Character, CombatStats};

#[derive(Resource)]
pub struct TurnOrder {
    /// Ordered list of entities by speed
    pub order: Vec<Entity>,
    /// Current index in turn order
    pub current_index: usize,
    /// Turn counter
    pub turn_count: u32,
}

impl TurnOrder {
    pub fn new() -> Self {
        TurnOrder {
            order: Vec::new(),
            current_index: 0,
            turn_count: 0,
        }
    }

    /// Get current active entity
    pub fn current(&self) -> Option<Entity> {
        self.order.get(self.current_index).copied()
    }

    /// Advance to next turn
    pub fn next(&mut self) {
        self.current_index = (self.current_index + 1) % self.order.len();
        if self.current_index == 0 {
            self.turn_count += 1;
        }
    }

    /// Initialize turn order by speed
    pub fn initialize(&mut self, combatants: Vec<(Entity, i32)>) {
        // Sort by speed (highest first)
        let mut sorted = combatants;
        sorted.sort_by(|a, b| b.1.cmp(&a.1));

        self.order = sorted.into_iter().map(|(e, _)| e).collect();
        self.current_index = 0;
        self.turn_count = 0;
    }

    /// Remove defeated entity
    pub fn remove(&mut self, entity: Entity) {
        if let Some(pos) = self.order.iter().position(|&e| e == entity) {
            self.order.remove(pos);
            // Adjust current index if needed
            if pos < self.current_index && self.current_index > 0 {
                self.current_index -= 1;
            }
        }
    }
}

/// Initialize battle turn order
pub fn setup_turn_order(
    mut turn_order: ResMut<TurnOrder>,
    query: Query<(Entity, &Character)>,
) {
    let combatants: Vec<(Entity, i32)> = query
        .iter()
        .map(|(entity, character)| (entity, character.data.stats.speed))
        .collect();

    turn_order.initialize(combatants);
    info!("Battle started with {} combatants", combatants.len());
}
```

### Combat State Machine

```rust
// src/combat/states.rs

use bevy::prelude::*;

/// Main combat flow states
#[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
pub enum CombatPhase {
    /// Waiting to start combat
    Inactive,
    /// Starting combat, initializing
    Initializing,
    /// Waiting for current entity's action
    WaitingForAction,
    /// Processing the selected action
    ExecutingAction,
    /// Applying end-of-turn effects
    ProcessingEffects,
    /// Checking for battle end
    CheckingVictory,
    /// Battle complete
    BattleComplete,
}

/// Who controls the current turn
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TurnController {
    Player,
    AI,
}

#[derive(Resource)]
pub struct CombatContext {
    pub controller: Option<TurnController>,
    pub action_in_progress: bool,
    pub animations_playing: usize,
}

/// Transition between combat phases
pub fn combat_phase_transitions(
    current_phase: Res<State<CombatPhase>>,
    mut next_phase: ResMut<NextState<CombatPhase>>,
    turn_order: Res<TurnOrder>,
    context: Res<CombatContext>,
    player_query: Query<Entity, With<Player>>,
) {
    match current_phase.get() {
        CombatPhase::Initializing => {
            // Move to first turn
            next_phase.set(CombatPhase::WaitingForAction);
        }

        CombatPhase::WaitingForAction => {
            // Handled by input or AI systems
        }

        CombatPhase::ExecutingAction => {
            if !context.action_in_progress && context.animations_playing == 0 {
                next_phase.set(CombatPhase::ProcessingEffects);
            }
        }

        CombatPhase::ProcessingEffects => {
            // After effects, check for victory
            next_phase.set(CombatPhase::CheckingVictory);
        }

        CombatPhase::CheckingVictory => {
            // If battle not over, next turn
            if !turn_order.order.is_empty() {
                next_phase.set(CombatPhase::WaitingForAction);
            } else {
                next_phase.set(CombatPhase::BattleComplete);
            }
        }

        _ => {}
    }
}
```

## Step 2: Action System

### Action Types and Queue

```rust
// src/combat/actions.rs

use bevy::prelude::*;

#[derive(Debug, Clone)]
pub enum ActionType {
    /// Basic attack
    Attack {
        target: Entity,
    },
    /// Use an ability
    Ability {
        ability_id: String,
        targets: Vec<Entity>,
    },
    /// Defensive stance
    Defend,
    /// Use consumable item
    Item {
        item_id: String,
        target: Option<Entity>,
    },
    /// Attempt to flee
    Flee,
    /// Skip turn
    Wait,
}

#[derive(Debug, Clone)]
pub struct CombatAction {
    pub executor: Entity,
    pub action_type: ActionType,
    pub priority: i32, // Higher priority acts first
}

#[derive(Resource, Default)]
pub struct ActionQueue {
    actions: Vec<CombatAction>,
}

impl ActionQueue {
    pub fn add(&mut self, action: CombatAction) {
        self.actions.push(action);
        // Sort by priority
        self.actions.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    pub fn next(&mut self) -> Option<CombatAction> {
        if self.actions.is_empty() {
            None
        } else {
            Some(self.actions.remove(0))
        }
    }

    pub fn clear(&mut self) {
        self.actions.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
}
```

### Action Execution

```rust
// src/combat/execution.rs

use bevy::prelude::*;
use logic_fsharp::combat::{calculate_damage, apply_damage};
use crate::combat::actions::{ActionType, CombatAction};
use crate::components::Character;

#[derive(Event)]
pub struct ActionExecutedEvent {
    pub action: CombatAction,
    pub results: Vec<ActionResult>,
}

#[derive(Debug, Clone)]
pub enum ActionResult {
    Damage { target: Entity, amount: i32, critical: bool },
    Healing { target: Entity, amount: i32 },
    StatusApplied { target: Entity, status: String },
    Missed { target: Entity },
    Fled { entity: Entity },
}

pub fn execute_combat_action(
    action: CombatAction,
    executor_query: &Query<&Character>,
    target_query: &mut Query<&mut Character>,
    mut events: EventWriter<ActionExecutedEvent>,
) -> Vec<ActionResult> {
    let mut results = Vec::new();

    match action.action_type {
        ActionType::Attack { target } => {
            if let (Ok(attacker), Ok(mut defender)) =
                (executor_query.get(action.executor), target_query.get_mut(target)) {

                // Calculate damage using logic layer
                let damage = calculate_damage(&attacker.data, &defender.data);

                // Check for critical hit (simplified)
                let is_critical = attacker.data.stats.speed > defender.data.stats.speed + 5;
                let final_damage = if is_critical {
                    (damage as f32 * 1.5) as i32
                } else {
                    damage
                };

                // Apply damage
                defender.data = apply_damage(&defender.data, final_damage);

                results.push(ActionResult::Damage {
                    target,
                    amount: final_damage,
                    critical: is_critical,
                });
            }
        }

        ActionType::Defend => {
            // Apply defense buff
            results.push(ActionResult::StatusApplied {
                target: action.executor,
                status: "Defending".to_string(),
            });
        }

        ActionType::Flee => {
            // Calculate flee chance
            let flee_success = rand::random::<f32>() > 0.5; // 50% chance
            if flee_success {
                results.push(ActionResult::Fled {
                    entity: action.executor,
                });
            }
        }

        _ => {}
    }

    // Send event
    events.send(ActionExecutedEvent {
        action: action.clone(),
        results: results.clone(),
    });

    results
}

pub fn process_action_queue(
    mut queue: ResMut<ActionQueue>,
    executor_query: Query<&Character>,
    mut target_query: Query<&mut Character>,
    mut events: EventWriter<ActionExecutedEvent>,
    mut context: ResMut<CombatContext>,
) {
    if let Some(action) = queue.next() {
        context.action_in_progress = true;
        execute_combat_action(action, &executor_query, &mut target_query, events);
        context.action_in_progress = false;
    }
}
```

## Step 3: Player Input

### Input Handling System

```rust
// src/combat/input.rs

use bevy::prelude::*;
use crate::combat::{
    actions::{ActionType, CombatAction},
    ActionQueue, TurnOrder, CombatPhase,
};
use crate::components::{Player, Enemy};

#[derive(Resource)]
pub struct InputState {
    pub selected_target: Option<Entity>,
    pub selected_action: Option<ActionType>,
}

pub fn handle_player_combat_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    combat_phase: Res<State<CombatPhase>>,
    turn_order: Res<TurnOrder>,
    mut action_queue: ResMut<ActionQueue>,
    mut next_phase: ResMut<NextState<CombatPhase>>,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    mut input_state: ResMut<InputState>,
) {
    // Only accept input during player's turn
    if *combat_phase.get() != CombatPhase::WaitingForAction {
        return;
    }

    // Check if it's player's turn
    let current_entity = turn_order.current().unwrap();
    if !player_query.contains(current_entity) {
        return;
    }

    // Keyboard shortcuts
    if keyboard.just_pressed(KeyCode::KeyA) || keyboard.just_pressed(KeyCode::Digit1) {
        // Attack
        if let Some(enemy) = enemy_query.iter().next() {
            let action = CombatAction {
                executor: current_entity,
                action_type: ActionType::Attack { target: enemy },
                priority: 10,
            };
            action_queue.add(action);
            next_phase.set(CombatPhase::ExecutingAction);
        }
    } else if keyboard.just_pressed(KeyCode::KeyD) || keyboard.just_pressed(KeyCode::Digit2) {
        // Defend
        let action = CombatAction {
            executor: current_entity,
            action_type: ActionType::Defend,
            priority: 20, // Defense has high priority
        };
        action_queue.add(action);
        next_phase.set(CombatPhase::ExecutingAction);
    } else if keyboard.just_pressed(KeyCode::KeyW) || keyboard.just_pressed(KeyCode::Digit3) {
        // Wait/Skip turn
        let action = CombatAction {
            executor: current_entity,
            action_type: ActionType::Wait,
            priority: 0,
        };
        action_queue.add(action);
        next_phase.set(CombatPhase::ExecutingAction);
    } else if keyboard.just_pressed(KeyCode::KeyR) || keyboard.just_pressed(KeyCode::Escape) {
        // Run/Flee
        let action = CombatAction {
            executor: current_entity,
            action_type: ActionType::Flee,
            priority: 15,
        };
        action_queue.add(action);
        next_phase.set(CombatPhase::ExecutingAction);
    }
}

/// Handle mouse targeting
pub fn handle_mouse_targeting(
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse: Res<ButtonInput<MouseButton>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut input_state: ResMut<InputState>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // Check if click is near an enemy
        for (entity, transform) in &enemy_query {
            let distance = world_position.distance(transform.translation.truncate());
            if distance < 50.0 {
                // Click radius
                input_state.selected_target = Some(entity);
                info!("Selected target: {:?}", entity);
            }
        }
    }
}
```

## Step 4: AI Decision Making

### AI Strategy System

```rust
// src/combat/ai.rs

use bevy::prelude::*;
use crate::combat::{
    actions::{ActionType, CombatAction},
    ActionQueue, TurnOrder, CombatPhase,
};
use crate::components::{Character, Enemy, Player};

#[derive(Debug, Clone, Copy)]
pub enum AIStrategy {
    Aggressive,  // Always attack
    Defensive,   // Defend when low health
    Balanced,    // Mix of strategies
    Supportive,  // Heal allies first
}

#[derive(Component)]
pub struct AIController {
    pub strategy: AIStrategy,
    pub decision_delay: Timer,
}

impl Default for AIController {
    fn default() -> Self {
        AIController {
            strategy: AIStrategy::Balanced,
            decision_delay: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

pub fn enemy_ai_decisions(
    time: Res<Time>,
    combat_phase: Res<State<CombatPhase>>,
    turn_order: Res<TurnOrder>,
    mut action_queue: ResMut<ActionQueue>,
    mut next_phase: ResMut<NextState<CombatPhase>>,
    mut ai_query: Query<(Entity, &Character, &mut AIController), With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
) {
    if *combat_phase.get() != CombatPhase::WaitingForAction {
        return;
    }

    let current_entity = match turn_order.current() {
        Some(e) => e,
        None => return,
    };

    // Check if it's an AI turn
    if let Ok((entity, character, mut ai)) = ai_query.get_mut(current_entity) {
        // Add delay for better game feel
        ai.decision_delay.tick(time.delta());
        if !ai.decision_delay.finished() {
            return;
        }

        // Make decision based on strategy
        let action = decide_action(entity, character, ai.strategy, &player_query);

        action_queue.add(action);
        next_phase.set(CombatPhase::ExecutingAction);

        // Reset timer for next turn
        ai.decision_delay.reset();
    }
}

fn decide_action(
    entity: Entity,
    character: &Character,
    strategy: AIStrategy,
    player_query: &Query<Entity, With<Player>>,
) -> CombatAction {
    let health_percentage = character.data.current_hp as f32 / character.data.stats.hp as f32;
    let player = player_query.single();

    let action_type = match strategy {
        AIStrategy::Aggressive => {
            // Always attack
            ActionType::Attack { target: player }
        }
        AIStrategy::Defensive => {
            if health_percentage < 0.4 {
                ActionType::Defend
            } else {
                ActionType::Attack { target: player }
            }
        }
        AIStrategy::Balanced => {
            if health_percentage < 0.3 {
                ActionType::Defend
            } else if rand::random::<f32>() > 0.8 {
                ActionType::Wait // Sometimes skip turn
            } else {
                ActionType::Attack { target: player }
            }
        }
        AIStrategy::Supportive => {
            // Would heal allies if they existed
            ActionType::Attack { target: player }
        }
    };

    CombatAction {
        executor: entity,
        action_type,
        priority: 10,
    }
}
```

## Step 5: Visual Feedback

### Combat Animations

```rust
// src/combat/animations.rs

use bevy::prelude::*;

#[derive(Component)]
pub struct AttackAnimation {
    pub start_position: Vec3,
    pub target_position: Vec3,
    pub timer: Timer,
    pub phase: AttackPhase,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AttackPhase {
    MovingToTarget,
    Impact,
    ReturningToStart,
}

pub fn animate_attacks(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut AttackAnimation)>,
    mut context: ResMut<CombatContext>,
) {
    for (entity, mut transform, mut animation) in &mut query {
        animation.timer.tick(time.delta());

        let progress = animation.timer.fraction();

        match animation.phase {
            AttackPhase::MovingToTarget => {
                // Lerp to target
                transform.translation = animation.start_position.lerp(
                    animation.target_position,
                    progress,
                );

                if animation.timer.finished() {
                    animation.phase = AttackPhase::Impact;
                    animation.timer = Timer::from_seconds(0.1, TimerMode::Once);
                }
            }
            AttackPhase::Impact => {
                // Shake effect
                let shake = Vec3::new(
                    rand::random::<f32>() * 4.0 - 2.0,
                    rand::random::<f32>() * 4.0 - 2.0,
                    0.0,
                );
                transform.translation = animation.target_position + shake;

                if animation.timer.finished() {
                    animation.phase = AttackPhase::ReturningToStart;
                    animation.timer = Timer::from_seconds(0.3, TimerMode::Once);
                }
            }
            AttackPhase::ReturningToStart => {
                // Lerp back to start
                transform.translation = animation.target_position.lerp(
                    animation.start_position,
                    progress,
                );

                if animation.timer.finished() {
                    commands.entity(entity).remove::<AttackAnimation>();
                    context.animations_playing = context.animations_playing.saturating_sub(1);
                }
            }
        }
    }
}

pub fn spawn_attack_animation(
    commands: &mut Commands,
    attacker: Entity,
    attacker_pos: Vec3,
    target_pos: Vec3,
    context: &mut CombatContext,
) {
    commands.entity(attacker).insert(AttackAnimation {
        start_position: attacker_pos,
        target_position: target_pos - (target_pos - attacker_pos).normalize() * 50.0,
        timer: Timer::from_seconds(0.3, TimerMode::Once),
        phase: AttackPhase::MovingToTarget,
    });
    context.animations_playing += 1;
}
```

### Damage Numbers

```rust
// src/combat/damage_display.rs

use bevy::prelude::*;

#[derive(Component)]
pub struct DamageNumber {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub fade_start: f32,
}

pub fn spawn_damage_number(
    commands: &mut Commands,
    position: Vec3,
    damage: i32,
    is_critical: bool,
    is_healing: bool,
) {
    let color = if is_healing {
        Color::GREEN
    } else if is_critical {
        Color::YELLOW
    } else {
        Color::WHITE
    };

    let font_size = if is_critical { 48.0 } else { 32.0 };

    let text = if is_healing {
        format!("+{}", damage)
    } else {
        format!("-{}", damage)
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                text,
                TextStyle {
                    font_size,
                    color,
                    ..default()
                },
            ),
            transform: Transform::from_translation(position + Vec3::new(0.0, 30.0, 10.0)),
            ..default()
        },
        DamageNumber {
            velocity: Vec2::new(
                rand::random::<f32>() * 20.0 - 10.0,
                60.0,
            ),
            lifetime: Timer::from_seconds(1.5, TimerMode::Once),
            fade_start: 0.7,
        },
    ));
}

pub fn animate_damage_numbers(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Text, &mut DamageNumber)>,
) {
    for (entity, mut transform, mut text, mut damage) in &mut query {
        damage.lifetime.tick(time.delta());

        // Move upward with slight drift
        transform.translation.x += damage.velocity.x * time.delta_seconds();
        transform.translation.y += damage.velocity.y * time.delta_seconds();

        // Slow down over time
        damage.velocity.y *= 0.98;

        // Fade out
        let lifetime_fraction = damage.lifetime.fraction();
        if lifetime_fraction > damage.fade_start {
            let fade_progress = (lifetime_fraction - damage.fade_start) / (1.0 - damage.fade_start);
            let alpha = 1.0 - fade_progress;

            if let Some(section) = text.sections.first_mut() {
                section.style.color.set_a(alpha);
            }
        }

        // Remove when done
        if damage.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
```

## Step 6: Combat UI

### Battle HUD

```rust
// src/combat/ui.rs

use bevy::prelude::*;
use crate::components::Character;
use crate::combat::TurnOrder;

#[derive(Component)]
pub struct CombatUI;

#[derive(Component)]
pub struct HealthBar {
    pub entity: Entity,
}

#[derive(Component)]
pub struct TurnIndicator;

pub fn setup_combat_ui(mut commands: Commands) {
    // Root UI container
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
            CombatUI,
        ))
        .with_children(|parent| {
            // Top bar - Turn indicator
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "BATTLE START",
                            TextStyle {
                                font_size: 32.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ),
                        TurnIndicator,
                    ));
                });

            // Bottom bar - Action buttons
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(100.0),
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(0.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                    ..default()
                })
                .with_children(|parent| {
                    spawn_action_button(parent, "Attack [A]", Color::RED);
                    spawn_action_button(parent, "Defend [D]", Color::BLUE);
                    spawn_action_button(parent, "Wait [W]", Color::GRAY);
                    spawn_action_button(parent, "Flee [R]", Color::YELLOW);
                });
        });
}

fn spawn_action_button(parent: &mut ChildBuilder, text: &str, color: Color) {
    parent
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(120.0),
                height: Val::Px(60.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: color.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
        });
}

pub fn update_turn_indicator(
    turn_order: Res<TurnOrder>,
    character_query: Query<&Character>,
    mut text_query: Query<&mut Text, With<TurnIndicator>>,
) {
    if let Some(current_entity) = turn_order.current() {
        if let Ok(character) = character_query.get(current_entity) {
            for mut text in &mut text_query {
                text.sections[0].value = format!(
                    "{}'s Turn - Round {}",
                    character.data.name,
                    turn_order.turn_count + 1
                );
            }
        }
    }
}
```

## Step 7: Integration

### Complete Combat Plugin

```rust
// src/combat/mod.rs

mod actions;
mod ai;
mod animations;
mod damage_display;
mod execution;
mod input;
mod states;
mod turn_order;
mod ui;

use bevy::prelude::*;

pub use actions::{ActionQueue, ActionType, CombatAction};
pub use states::{CombatPhase, CombatContext};
pub use turn_order::TurnOrder;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .init_state::<CombatPhase>()

            // Resources
            .insert_resource(TurnOrder::new())
            .insert_resource(ActionQueue::default())
            .insert_resource(CombatContext {
                controller: None,
                action_in_progress: false,
                animations_playing: 0,
            })
            .insert_resource(input::InputState {
                selected_target: None,
                selected_action: None,
            })

            // Events
            .add_event::<execution::ActionExecutedEvent>()

            // Setup systems
            .add_systems(OnEnter(CombatPhase::Initializing), (
                turn_order::setup_turn_order,
                ui::setup_combat_ui,
            ))

            // Input systems
            .add_systems(Update, (
                input::handle_player_combat_input,
                input::handle_mouse_targeting,
            ).run_if(in_state(CombatPhase::WaitingForAction)))

            // AI systems
            .add_systems(Update,
                ai::enemy_ai_decisions
                    .run_if(in_state(CombatPhase::WaitingForAction))
            )

            // Execution systems
            .add_systems(Update, (
                execution::process_action_queue,
                animations::animate_attacks,
                damage_display::animate_damage_numbers,
            ).run_if(in_state(CombatPhase::ExecutingAction)))

            // UI systems
            .add_systems(Update, (
                ui::update_turn_indicator,
                ui::update_health_bars,
            ))

            // State transitions
            .add_systems(Update, states::combat_phase_transitions)

            // Cleanup
            .add_systems(OnExit(CombatPhase::BattleComplete),
                cleanup_combat
            );
    }
}

fn cleanup_combat(
    mut commands: Commands,
    ui_query: Query<Entity, With<ui::CombatUI>>,
) {
    // Remove combat UI
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }
}
```

## Exercises

### Exercise 1: Add Combo System

Create a combo counter that increases damage:

```rust
#[derive(Component)]
pub struct ComboCounter {
    pub count: u32,
    pub timer: Timer,
}

// Reset combo if no action for 3 seconds
// Each hit increases combo
// Damage multiplier based on combo count
```

### Exercise 2: Implement Critical Hits

Add proper critical hit calculation:

```rust
pub fn calculate_crit_chance(attacker: &Character, defender: &Character) -> f32 {
    // Base crit chance
    // Modified by speed difference
    // Modified by luck stat
    // Cap at reasonable maximum
}
```

### Exercise 3: Add Battle Victory Screen

Create a victory/defeat screen:

```rust
fn show_battle_results(
    commands: &mut Commands,
    result: BattleResult,
    stats: BattleStatistics,
) {
    // Show XP gained
    // Show items found
    // Show battle statistics
    // Continue button
}
```

## Performance Optimization

1. **Batch Visual Updates**: Update all health bars in one system
2. **Pool Damage Numbers**: Reuse text entities instead of spawning/despawning
3. **Limit Particles**: Cap maximum particle count
4. **Use State Scoping**: Only run systems in relevant states

## Summary

You've built a complete combat system with:

✅ Turn-based flow with speed-based ordering
✅ Player input with keyboard and mouse
✅ AI decision making with strategies
✅ Visual feedback and animations
✅ Combat UI with health bars and turn indicators
✅ Damage numbers and effects

## Next Steps

In the next chapter, we'll compile everything to WebAssembly for browser deployment.

[Next: WASM Deployment →](07-wasm-deployment.md)

[← Previous: Bevy Integration](05-bevy-integration.md) | [Tutorial Index](README.md)