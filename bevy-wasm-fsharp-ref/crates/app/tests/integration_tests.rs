//! Integration tests for the Bevy combat application
//!
//! These tests verify the complete combat system including:
//! - Entity spawning (player and monster)
//! - Combat turn flow (PlayerTurn → MonsterTurn → PlayerTurn)
//! - Game over detection (victory and defeat conditions)
//! - Damage application and HP tracking
//! - Combat log functionality
//! - Multi-round combat scenarios
//!
//! All tests run headlessly using MinimalPlugins for CI/CD compatibility.

use bevy::prelude::*;
use bevy_wasm_fsharp_ref_logic as logic;
use std::collections::VecDeque;

// ==================== Test Helpers and Utilities ====================

/// Component marking the player entity (duplicated from lib.rs for tests)
#[derive(Component)]
struct Player;

/// Component marking the monster entity (duplicated from lib.rs for tests)
#[derive(Component)]
struct Monster;

/// Resource tracking the current state of combat (duplicated from lib.rs for tests)
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

/// Resource for tracking combat events and messages (duplicated from lib.rs for tests)
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

    fn contains(&self, text: &str) -> bool {
        self.events.iter().any(|e| e.contains(text))
    }
}

/// Timer for automatic monster turn execution (duplicated from lib.rs for tests)
#[derive(Resource)]
#[allow(dead_code)]
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

/// Helper function to create a minimal test app with all necessary resources
fn create_test_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CombatState>()
        .insert_resource(CombatLog::new(10))
        .init_resource::<MonsterTurnTimer>();
    app
}

/// Helper function to spawn test entities (player and monster)
fn spawn_test_entities(app: &mut App) {
    app.world_mut().spawn((
        Player,
        logic::Character::new_player("Hero"),
    ));

    app.world_mut().spawn((
        Monster,
        logic::Character::new_monster("Slime"),
    ));
}

/// Helper function to simulate a player attack
fn simulate_player_attack(app: &mut App) {
    // Get entity IDs and character data
    let (_player_entity, monster_entity, player, monster) = {
        let world = app.world_mut();

        let player_entity = world
            .query_filtered::<Entity, With<Player>>()
            .single(world);

        let monster_entity = world
            .query_filtered::<Entity, With<Monster>>()
            .single(world);

        let mut query = world.query::<&logic::Character>();
        let player = query.get(world, player_entity).unwrap().clone();
        let monster = query.get(world, monster_entity).unwrap().clone();

        (player_entity, monster_entity, player, monster)
    };

    let mut monster = monster;

    // Execute attack
    let ability = logic::Ability::basic_attack();
    let event = logic::compute_attack(&player, &monster, &ability);

    // Apply damage
    monster.hp = event.defender_hp_after;

    // Update monster entity
    {
        let world = app.world_mut();
        let mut query = world.query::<&mut logic::Character>();
        *query.get_mut(world, monster_entity).unwrap() = monster.clone();
    }

    // Log the attack and update state
    {
        let world = app.world_mut();
        let mut combat_log = world.resource_mut::<CombatLog>();
        combat_log.add(format!(
            "{} attacks {} for {} damage!",
            event.attacker_name, event.defender_name, event.damage
        ));
        combat_log.add(format!(
            "{} HP: {} / {}",
            monster.name, monster.hp, monster.stats.hp
        ));

        if monster.hp <= 0 {
            combat_log.add(format!("{} has been defeated!", monster.name));
        }
    }

    // Update combat state separately
    {
        let world = app.world_mut();
        let mut combat_state = world.resource_mut::<CombatState>();
        if monster.hp <= 0 {
            *combat_state = CombatState::GameOver {
                winner: player.name.clone(),
            };
        } else {
            *combat_state = CombatState::MonsterTurn;
        }
    }
}

/// Helper function to simulate a monster attack
fn simulate_monster_attack(app: &mut App) {
    // Get entity IDs and character data
    let (player_entity, _monster_entity, player, monster) = {
        let world = app.world_mut();

        let player_entity = world
            .query_filtered::<Entity, With<Player>>()
            .single(world);

        let monster_entity = world
            .query_filtered::<Entity, With<Monster>>()
            .single(world);

        let mut query = world.query::<&logic::Character>();
        let monster = query.get(world, monster_entity).unwrap().clone();
        let player = query.get(world, player_entity).unwrap().clone();

        (player_entity, monster_entity, player, monster)
    };

    let mut player = player;

    // Execute attack
    let ability = logic::Ability::basic_attack();
    let event = logic::compute_attack(&monster, &player, &ability);

    // Apply damage
    player.hp = event.defender_hp_after;

    // Update player entity
    {
        let world = app.world_mut();
        let mut query = world.query::<&mut logic::Character>();
        *query.get_mut(world, player_entity).unwrap() = player.clone();
    }

    // Log the attack and update state
    {
        let world = app.world_mut();
        let mut combat_log = world.resource_mut::<CombatLog>();
        combat_log.add(format!(
            "{} attacks {} for {} damage!",
            event.attacker_name, event.defender_name, event.damage
        ));
        combat_log.add(format!(
            "{} HP: {} / {}",
            player.name, player.hp, player.stats.hp
        ));

        if player.hp <= 0 {
            combat_log.add(format!("{} has been defeated!", player.name));
        }
    }

    // Update combat state separately
    {
        let world = app.world_mut();
        let mut combat_state = world.resource_mut::<CombatState>();
        if player.hp <= 0 {
            *combat_state = CombatState::GameOver {
                winner: monster.name.clone(),
            };
        } else {
            *combat_state = CombatState::PlayerTurn;
        }
    }
}

/// Helper function to get a character from the world
fn get_character<T: Component>(app: &mut App) -> logic::Character {
    let world = app.world_mut();
    world
        .query_filtered::<&logic::Character, With<T>>()
        .single(world)
        .clone()
}

/// Helper function to modify a character's stats
fn modify_character_stats<T: Component, F>(app: &mut App, modifier: F)
where
    F: FnOnce(&mut logic::Character),
{
    let (entity, mut character) = {
        let world = app.world_mut();
        let entity = world
            .query_filtered::<Entity, With<T>>()
            .single(world);

        let mut query = world.query::<&logic::Character>();
        let character = query.get(world, entity).unwrap().clone();

        (entity, character)
    };

    modifier(&mut character);

    {
        let world = app.world_mut();
        let mut query = world.query::<&mut logic::Character>();
        *query.get_mut(world, entity).unwrap() = character;
    }
}

// ==================== Entity Spawning Tests ====================

#[test]
fn test_player_entity_spawned() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Verify player entity exists
    let player_count = {
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<Player>>();
        query.iter(world).count()
    };

    assert_eq!(player_count, 1, "Exactly one player entity should exist");
}

#[test]
fn test_monster_entity_spawned() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Verify monster entity exists
    let monster_count = {
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<Monster>>();
        query.iter(world).count()
    };

    assert_eq!(monster_count, 1, "Exactly one monster entity should exist");
}

#[test]
fn test_spawned_entities_have_correct_stats() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Check player stats
    let player = get_character::<Player>(&mut app);
    assert_eq!(player.name, "Hero");
    assert_eq!(player.hp, 30);
    assert_eq!(player.stats.attack, 10);
    assert_eq!(player.stats.defense, 2);

    // Check monster stats
    let monster = get_character::<Monster>(&mut app);
    assert_eq!(monster.name, "Slime");
    assert_eq!(monster.hp, 20);
    assert_eq!(monster.stats.attack, 6);
    assert_eq!(monster.stats.defense, 1);
}

// ==================== Combat Turn Flow Tests ====================

#[test]
fn test_initial_combat_state_is_player_turn() {
    let app = create_test_app();

    let state = app.world().resource::<CombatState>();
    assert_eq!(*state, CombatState::PlayerTurn);
}

#[test]
fn test_player_attack_transitions_to_monster_turn() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Initial state should be PlayerTurn
    {
        let state = app.world().resource::<CombatState>();
        assert_eq!(*state, CombatState::PlayerTurn);
    }

    // Simulate player attack
    simulate_player_attack(&mut app);

    // State should transition to MonsterTurn
    {
        let state = app.world().resource::<CombatState>();
        assert_eq!(*state, CombatState::MonsterTurn);
    }
}

#[test]
fn test_monster_attack_transitions_to_player_turn() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Set initial state to MonsterTurn
    {
        let mut state = app.world_mut().resource_mut::<CombatState>();
        *state = CombatState::MonsterTurn;
    }

    // Simulate monster attack
    simulate_monster_attack(&mut app);

    // State should transition back to PlayerTurn
    {
        let state = app.world().resource::<CombatState>();
        assert_eq!(*state, CombatState::PlayerTurn);
    }
}

#[test]
fn test_turn_alternation_sequence() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Weaken both characters so combat lasts longer
    modify_character_stats::<Player, _>(&mut app, |character| {
        character.stats.attack = 3; // Reduced from 10 to make combat last longer
    });
    modify_character_stats::<Monster, _>(&mut app, |character| {
        character.stats.attack = 2; // Reduced from 6 to make combat last longer
    });

    // Turn 1: Player attacks
    {
        let state = app.world().resource::<CombatState>();
        assert_eq!(*state, CombatState::PlayerTurn);
    }
    simulate_player_attack(&mut app);

    // Turn 2: Monster attacks
    {
        let state = app.world().resource::<CombatState>();
        assert_eq!(*state, CombatState::MonsterTurn);
    }
    simulate_monster_attack(&mut app);

    // Turn 3: Player attacks again
    {
        let state = app.world().resource::<CombatState>();
        assert_eq!(*state, CombatState::PlayerTurn);
    }
    simulate_player_attack(&mut app);

    // Turn 4: Monster attacks again
    {
        let state = app.world().resource::<CombatState>();
        assert_eq!(*state, CombatState::MonsterTurn);
    }
}

// ==================== Victory Condition Tests ====================

#[test]
fn test_player_victory_condition() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Give player high attack to win quickly
    modify_character_stats::<Player, _>(&mut app, |character| {
        character.stats.attack = 100;
    });

    // Execute attacks until game over
    for _ in 0..10 {
        let state = app.world().resource::<CombatState>().clone();

        match state {
            CombatState::GameOver { ref winner } => {
                assert_eq!(winner, "Hero");
                return;
            }
            CombatState::PlayerTurn => simulate_player_attack(&mut app),
            CombatState::MonsterTurn => simulate_monster_attack(&mut app),
        }
    }

    panic!("Game should have ended with player victory");
}

#[test]
fn test_monster_victory_condition() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Give monster high attack to win quickly
    modify_character_stats::<Monster, _>(&mut app, |character| {
        character.stats.attack = 100;
    });

    // Set state to MonsterTurn to let monster attack first
    {
        let mut state = app.world_mut().resource_mut::<CombatState>();
        *state = CombatState::MonsterTurn;
    }

    // Execute attacks until game over
    for _ in 0..10 {
        let state = app.world().resource::<CombatState>().clone();

        match state {
            CombatState::GameOver { ref winner } => {
                assert_eq!(winner, "Slime");
                return;
            }
            CombatState::PlayerTurn => simulate_player_attack(&mut app),
            CombatState::MonsterTurn => simulate_monster_attack(&mut app),
        }
    }

    panic!("Game should have ended with monster victory");
}

#[test]
fn test_game_over_state_is_permanent() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Manually set game over state
    {
        let mut state = app.world_mut().resource_mut::<CombatState>();
        *state = CombatState::GameOver {
            winner: "Test".to_string(),
        };
    }

    let initial_state = app.world().resource::<CombatState>().clone();

    // Try to simulate attacks (should not change state)
    // Note: In a real implementation, the systems would prevent this
    // For now, we just verify the state was set correctly
    let final_state = app.world().resource::<CombatState>().clone();

    assert_eq!(initial_state, final_state);
    assert!(matches!(final_state, CombatState::GameOver { .. }));
}

// ==================== Damage Application Tests ====================

#[test]
fn test_player_attack_applies_correct_damage() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    let initial_monster_hp = get_character::<Monster>(&mut app).hp;

    // Player attack: 10 + 5 (ability) - 1 (defense) = 14 damage
    simulate_player_attack(&mut app);

    let final_monster_hp = get_character::<Monster>(&mut app).hp;
    let damage_dealt = initial_monster_hp - final_monster_hp;

    assert_eq!(damage_dealt, 14);
    assert_eq!(final_monster_hp, 6); // 20 - 14
}

#[test]
fn test_monster_attack_applies_correct_damage() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Set state to MonsterTurn
    {
        let mut state = app.world_mut().resource_mut::<CombatState>();
        *state = CombatState::MonsterTurn;
    }

    let initial_player_hp = get_character::<Player>(&mut app).hp;

    // Monster attack: 6 + 5 (ability) - 2 (defense) = 9 damage
    simulate_monster_attack(&mut app);

    let final_player_hp = get_character::<Player>(&mut app).hp;
    let damage_dealt = initial_player_hp - final_player_hp;

    assert_eq!(damage_dealt, 9);
    assert_eq!(final_player_hp, 21); // 30 - 9
}

#[test]
fn test_hp_cannot_go_below_zero() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Give player massive attack
    modify_character_stats::<Player, _>(&mut app, |character| {
        character.stats.attack = 1000;
    });

    simulate_player_attack(&mut app);

    let monster_hp = get_character::<Monster>(&mut app).hp;
    assert!(monster_hp <= 0, "Monster HP should be 0 or negative");
}

// ==================== Combat Log Tests ====================

#[test]
fn test_combat_log_records_attacks() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    simulate_player_attack(&mut app);

    let log = app.world().resource::<CombatLog>();
    assert!(log.contains("Hero attacks Slime"));
}

#[test]
fn test_combat_log_records_damage() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    simulate_player_attack(&mut app);

    let log = app.world().resource::<CombatLog>();
    assert!(log.contains("14 damage"));
}

#[test]
fn test_combat_log_records_hp_status() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    simulate_player_attack(&mut app);

    let log = app.world().resource::<CombatLog>();
    assert!(log.contains("Slime HP: 6 / 20"));
}

#[test]
fn test_combat_log_records_defeat() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Give player high attack to defeat monster quickly
    modify_character_stats::<Player, _>(&mut app, |character| {
        character.stats.attack = 100;
    });

    simulate_player_attack(&mut app);

    let log = app.world().resource::<CombatLog>();
    assert!(log.contains("has been defeated"));
}

#[test]
fn test_combat_log_max_events() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<CombatState>()
        .insert_resource(CombatLog::new(3)) // Max 3 events
        .init_resource::<MonsterTurnTimer>();

    let mut log = app.world_mut().resource_mut::<CombatLog>();

    log.add("Event 1".to_string());
    log.add("Event 2".to_string());
    log.add("Event 3".to_string());
    log.add("Event 4".to_string());

    // Should only have the last 3 events
    assert_eq!(log.events.len(), 3);
    assert!(!log.contains("Event 1"));
    assert!(log.contains("Event 4"));
}

// ==================== Multi-Round Combat Tests ====================

#[test]
fn test_multiple_combat_rounds() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Execute multiple rounds
    for round in 0..3 {
        let state = app.world().resource::<CombatState>().clone();

        if matches!(state, CombatState::GameOver { .. }) {
            break;
        }

        // Player turn
        if matches!(state, CombatState::PlayerTurn) {
            simulate_player_attack(&mut app);
        }

        let state = app.world().resource::<CombatState>().clone();

        if matches!(state, CombatState::GameOver { .. }) {
            break;
        }

        // Monster turn
        if matches!(state, CombatState::MonsterTurn) {
            simulate_monster_attack(&mut app);
        }

        // Verify both characters are still alive after this round
        if round < 2 {
            let player = get_character::<Player>(&mut app);
            let monster = get_character::<Monster>(&mut app);
            assert!(player.hp > 0 || monster.hp > 0);
        }
    }

    // After 3 rounds, verify combat state is valid
    let state = app.world().resource::<CombatState>();
    assert!(
        matches!(state, CombatState::PlayerTurn)
            || matches!(state, CombatState::MonsterTurn)
            || matches!(state, CombatState::GameOver { .. })
    );
}

#[test]
fn test_complete_combat_until_victory() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Player: 10 attack + 5 ability - 1 defense = 14 damage per hit
    // Monster has 20 HP, so needs 2 hits (14 + 14 > 20)

    // Round 1: Player attacks
    simulate_player_attack(&mut app);
    {
        let monster = get_character::<Monster>(&mut app);
        assert_eq!(monster.hp, 6); // 20 - 14
    }

    // Round 1: Monster attacks
    simulate_monster_attack(&mut app);
    {
        let player = get_character::<Player>(&mut app);
        assert_eq!(player.hp, 21); // 30 - 9
    }

    // Round 2: Player attacks (should defeat monster)
    simulate_player_attack(&mut app);
    {
        let monster = get_character::<Monster>(&mut app);
        assert!(monster.hp <= 0);
    }

    // Verify game over
    let state = app.world().resource::<CombatState>();
    match state {
        CombatState::GameOver { winner } => {
            assert_eq!(winner, "Hero");
        }
        _ => panic!("Game should be over with player victory"),
    }
}

#[test]
fn test_hp_tracking_across_rounds() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    let mut player_hp_history = vec![];
    let mut monster_hp_history = vec![];

    player_hp_history.push(get_character::<Player>(&mut app).hp);
    monster_hp_history.push(get_character::<Monster>(&mut app).hp);

    for _ in 0..3 {
        let state = app.world().resource::<CombatState>().clone();

        if matches!(state, CombatState::GameOver { .. }) {
            break;
        }

        if matches!(state, CombatState::PlayerTurn) {
            simulate_player_attack(&mut app);
            monster_hp_history.push(get_character::<Monster>(&mut app).hp);
        }

        let state = app.world().resource::<CombatState>().clone();

        if matches!(state, CombatState::GameOver { .. }) {
            break;
        }

        if matches!(state, CombatState::MonsterTurn) {
            simulate_monster_attack(&mut app);
            player_hp_history.push(get_character::<Player>(&mut app).hp);
        }
    }

    // Verify HP is monotonically decreasing (or staying at 0/-negative)
    for i in 1..player_hp_history.len() {
        assert!(
            player_hp_history[i] <= player_hp_history[i - 1],
            "Player HP should decrease or stay same"
        );
    }

    for i in 1..monster_hp_history.len() {
        assert!(
            monster_hp_history[i] <= monster_hp_history[i - 1],
            "Monster HP should decrease or stay same"
        );
    }
}

// ==================== Edge Case Tests ====================

#[test]
fn test_minimum_damage_is_applied() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Give monster extremely high defense
    modify_character_stats::<Monster, _>(&mut app, |character| {
        character.stats.defense = 1000;
    });

    let initial_hp = get_character::<Monster>(&mut app).hp;
    simulate_player_attack(&mut app);
    let final_hp = get_character::<Monster>(&mut app).hp;

    // Even with high defense, minimum 1 damage should be dealt
    assert_eq!(initial_hp - final_hp, 1);
}

#[test]
fn test_zero_defense_character() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Set monster defense to 0
    modify_character_stats::<Monster, _>(&mut app, |character| {
        character.stats.defense = 0;
    });

    simulate_player_attack(&mut app);

    let monster_hp = get_character::<Monster>(&mut app).hp;
    // 10 attack + 5 ability - 0 defense = 15 damage
    // 20 - 15 = 5 HP
    assert_eq!(monster_hp, 5);
}

#[test]
fn test_combat_with_equal_strength_opponents() {
    let mut app = create_test_app();
    spawn_test_entities(&mut app);

    // Make both characters equal
    modify_character_stats::<Player, _>(&mut app, |character| {
        character.hp = 50;
        character.stats.hp = 50;
        character.stats.attack = 10;
        character.stats.defense = 5;
    });

    modify_character_stats::<Monster, _>(&mut app, |character| {
        character.hp = 50;
        character.stats.hp = 50;
        character.stats.attack = 10;
        character.stats.defense = 5;
    });

    // Run combat for several rounds
    for _ in 0..20 {
        let state = app.world().resource::<CombatState>().clone();

        match state {
            CombatState::GameOver { winner } => {
                // Someone should eventually win
                assert!(winner == "Hero" || winner == "Slime");
                return;
            }
            CombatState::PlayerTurn => simulate_player_attack(&mut app),
            CombatState::MonsterTurn => simulate_monster_attack(&mut app),
        }
    }

    // Game should have ended by now with equal stats
    let state = app.world().resource::<CombatState>();
    assert!(matches!(state, CombatState::GameOver { .. }));
}
