use bevy::prelude::*;
use bevy_wasm_fsharp_ref_logic as logic;
use std::collections::VecDeque;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Component marking the player entity
#[derive(Component)]
struct Player;

/// Component marking the monster entity
#[derive(Component)]
struct Monster;

/// Resource tracking the current state of combat
#[derive(Resource, Debug, Clone, PartialEq)]
enum CombatState {
    PlayerTurn,
    MonsterTurn,
    GameOver { winner: String },
}

impl Default for CombatState {
    fn default() -> Self {
        CombatState::PlayerTurn
    }
}

/// Resource for tracking combat events and messages
#[derive(Resource, Default)]
struct CombatLog {
    events: VecDeque<String>,
    max_events: usize,
}

impl CombatLog {
    fn new(max_events: usize) -> Self {
        Self {
            events: VecDeque::new(),
            max_events,
        }
    }

    fn add(&mut self, message: String) {
        println!("{}", message); // Also log to console
        self.events.push_back(message);
        if self.events.len() > self.max_events {
            self.events.pop_front();
        }
    }

    #[allow(dead_code)]
    fn get_recent(&self, count: usize) -> Vec<String> {
        self.events
            .iter()
            .rev()
            .take(count)
            .rev()
            .cloned()
            .collect()
    }
}

/// Timer for automatic monster turn execution
#[derive(Resource)]
struct MonsterTurnTimer {
    timer: Timer,
}

impl Default for MonsterTurnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy WASM F# Reference - Combat Demo".to_string(),
                resolution: (800., 600.).into(),
                #[cfg(target_arch = "wasm32")]
                canvas: Some("#bevy".to_string()),
                #[cfg(target_arch = "wasm32")]
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<CombatState>()
        .insert_resource(CombatLog::new(10))
        .init_resource::<MonsterTurnTimer>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_player_turn,
                handle_monster_turn,
                check_game_over,
                display_turn_indicator,
                display_combat_log,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, mut combat_log: ResMut<CombatLog>) {
    // Spawn 2D camera
    commands.spawn(Camera2d::default());

    // Player: Blue circle at (-100, 0)
    // Using simple colored sprites for visual representation
    commands.spawn((
        Player,
        logic::Character::new_player("Hero"),
        Sprite {
            color: Color::srgb(0.2, 0.4, 0.8),        // Blue
            custom_size: Some(Vec2::new(60.0, 60.0)), // 60x60 square to represent circle
            ..default()
        },
        Transform::from_xyz(-100.0, 0.0, 0.0),
    ));

    // Monster: Red rectangle at (100, 0)
    // Using simple colored sprites for visual representation
    commands.spawn((
        Monster,
        logic::Character::new_monster("Slime"),
        Sprite {
            color: Color::srgb(0.8, 0.2, 0.2),        // Red
            custom_size: Some(Vec2::new(50.0, 60.0)), // 50x60 rectangle
            ..default()
        },
        Transform::from_xyz(100.0, 0.0, 0.0),
    ));

    // Welcome message
    combat_log.add("=== Combat Start ===".to_string());
    combat_log.add("Press SPACE to attack on your turn!".to_string());
}

/// System that handles player input and actions during PlayerTurn
fn handle_player_turn(
    keys: Res<ButtonInput<KeyCode>>,
    mut combat_state: ResMut<CombatState>,
    mut combat_log: ResMut<CombatLog>,
    mut players: Query<&mut logic::Character, With<Player>>,
    mut monsters: Query<&mut logic::Character, (With<Monster>, Without<Player>)>,
) {
    // Only process during player's turn
    if *combat_state != CombatState::PlayerTurn {
        return;
    }

    // Wait for space key press
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    // Get player and monster
    let player = match players.iter_mut().next() {
        Some(p) => p,
        None => return,
    };

    let mut monster = match monsters.iter_mut().next() {
        Some(m) => m,
        None => return,
    };

    // Execute attack
    let ability = logic::Ability::basic_attack();
    let event = logic::compute_attack(&player, &monster, &ability);

    // Log the attack
    combat_log.add(format!(
        "{} attacks {} for {} damage!",
        event.attacker_name, event.defender_name, event.damage
    ));

    // Apply damage
    monster.hp = event.defender_hp_after;

    // Log HP status
    combat_log.add(format!(
        "{} HP: {} / {}",
        monster.name, monster.hp, monster.stats.hp
    ));

    // Check if monster is defeated
    if monster.hp <= 0 {
        combat_log.add(format!("{} has been defeated!", monster.name));
        *combat_state = CombatState::GameOver {
            winner: player.name.clone(),
        };
    } else {
        // Switch to monster's turn
        *combat_state = CombatState::MonsterTurn;
        combat_log.add("--- Monster's Turn ---".to_string());
    }
}

/// System that handles automatic monster actions during MonsterTurn
fn handle_monster_turn(
    time: Res<Time>,
    mut timer: ResMut<MonsterTurnTimer>,
    mut combat_state: ResMut<CombatState>,
    mut combat_log: ResMut<CombatLog>,
    mut players: Query<&mut logic::Character, With<Player>>,
    monsters: Query<&logic::Character, (With<Monster>, Without<Player>)>,
) {
    // Only process during monster's turn
    if *combat_state != CombatState::MonsterTurn {
        return;
    }

    // Tick the timer
    timer.timer.tick(time.delta());

    // Wait for timer to finish
    if !timer.timer.finished() {
        return;
    }

    // Reset timer for next turn
    timer.timer.reset();

    // Get player and monster
    let monster = match monsters.iter().next() {
        Some(m) => m,
        None => return,
    };

    let mut player = match players.iter_mut().next() {
        Some(p) => p,
        None => return,
    };

    // Monster attacks (simple AI: always basic attack)
    let ability = logic::Ability::basic_attack();
    let event = logic::compute_attack(&monster, &player, &ability);

    // Log the attack
    combat_log.add(format!(
        "{} attacks {} for {} damage!",
        event.attacker_name, event.defender_name, event.damage
    ));

    // Apply damage
    player.hp = event.defender_hp_after;

    // Log HP status
    combat_log.add(format!(
        "{} HP: {} / {}",
        player.name, player.hp, player.stats.hp
    ));

    // Check if player is defeated
    if player.hp <= 0 {
        combat_log.add(format!("{} has been defeated!", player.name));
        *combat_state = CombatState::GameOver {
            winner: monster.name.clone(),
        };
    } else {
        // Switch back to player's turn
        *combat_state = CombatState::PlayerTurn;
        combat_log.add("--- Player's Turn ---".to_string());
    }
}

/// System that checks for game over conditions
fn check_game_over(combat_state: Res<CombatState>, mut combat_log: ResMut<CombatLog>) {
    // Only log once when entering game over state
    if !combat_state.is_changed() {
        return;
    }

    if let CombatState::GameOver { winner } = &*combat_state {
        combat_log.add("=== GAME OVER ===".to_string());
        combat_log.add(format!("{} wins!", winner));
        combat_log.add("Close the window to exit.".to_string());
    }
}

/// System that displays the current turn indicator
fn display_turn_indicator(combat_state: Res<CombatState>) {
    // Only log when state changes
    if !combat_state.is_changed() {
        return;
    }

    match &*combat_state {
        CombatState::PlayerTurn => {
            println!("\n>>> YOUR TURN <<<");
            println!("Press SPACE to attack!");
        }
        CombatState::MonsterTurn => {
            println!("\n>>> MONSTER'S TURN <<<");
            println!("Monster is preparing to attack...");
        }
        CombatState::GameOver { winner } => {
            println!("\n>>> GAME OVER <<<");
            println!("Winner: {}", winner);
        }
    }
}

/// System that displays recent combat log entries
fn display_combat_log(combat_log: Res<CombatLog>) {
    // Only display when log changes
    if !combat_log.is_changed() {
        return;
    }

    // Get the most recent event and display it
    if let Some(last_event) = combat_log.events.back() {
        // The event is already printed in CombatLog::add()
        // This system exists for future UI integration
        let _ = last_event; // Silence unused warning
    }
}
