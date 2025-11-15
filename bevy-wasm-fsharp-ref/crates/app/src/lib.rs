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

/// Resource tracking monster abilities with cooldowns
#[derive(Resource)]
struct MonsterAbilities {
    abilities: Vec<logic::AbilityWithMeta>,
}

impl MonsterAbilities {
    fn new() -> Self {
        Self {
            abilities: vec![
                logic::AbilityWithMeta::basic_attack_meta(),
                logic::AbilityWithMeta::powerful_attack_meta(),
                logic::AbilityWithMeta::heal_meta(),
            ],
        }
    }

    fn tick_cooldowns(&mut self) {
        for ability in &mut self.abilities {
            ability.tick_cooldown();
        }
    }

    fn activate_ability(&mut self, ability_name: &str) {
        if let Some(ability) = self
            .abilities
            .iter_mut()
            .find(|a| a.ability.name == ability_name)
        {
            ability.activate();
        }
    }
}

/// Component for health bar UI elements
#[derive(Component)]
struct HealthBar {
    owner: Entity,
}

/// Component for health bar background
#[derive(Component)]
struct HealthBarBackground;

/// Component for health bar foreground (the green part that shrinks)
#[derive(Component)]
struct HealthBarForeground {
    owner: Entity,
}

/// Component for health text display
#[derive(Component)]
struct HealthText {
    owner: Entity,
}

/// Component for floating damage numbers
#[derive(Component)]
struct DamageNumber {
    lifetime: Timer,
    rise_speed: f32,
}

/// Component for attack flash effect
#[derive(Component)]
struct AttackFlash {
    timer: Timer,
    original_color: Color,
}

/// Component for hit shake effect
#[derive(Component)]
struct HitShake {
    timer: Timer,
    original_position: Vec3,
    intensity: f32,
}

/// Component for game over message
#[derive(Component)]
struct GameOverMessage;

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
        .insert_resource(MonsterAbilities::new())
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
                update_health_bars,
                animate_damage_numbers,
                animate_attack_flash,
                animate_hit_shake,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands, mut combat_log: ResMut<CombatLog>) {
    // Spawn 2D camera
    commands.spawn(Camera2d::default());

    // Player: Blue circle at (-100, 0)
    let player_entity = commands
        .spawn((
            Player,
            logic::Character::new_player("Hero"),
            Sprite {
                color: Color::srgb(0.2, 0.4, 0.8),        // Blue
                custom_size: Some(Vec2::new(60.0, 60.0)), // 60x60 square to represent circle
                ..default()
            },
            Transform::from_xyz(-100.0, 0.0, 0.0),
        ))
        .id();

    // Monster: Red rectangle at (100, 0)
    let monster_entity = commands
        .spawn((
            Monster,
            logic::Character::new_monster("Slime"),
            Sprite {
                color: Color::srgb(0.8, 0.2, 0.2),        // Red
                custom_size: Some(Vec2::new(50.0, 60.0)), // 50x60 rectangle
                ..default()
            },
            Transform::from_xyz(100.0, 0.0, 0.0),
        ))
        .id();

    // Create health bars for player
    spawn_health_bar(&mut commands, player_entity, Vec3::new(-100.0, 50.0, 1.0));

    // Create health bars for monster
    spawn_health_bar(&mut commands, monster_entity, Vec3::new(100.0, 50.0, 1.0));

    // Welcome message
    combat_log.add("=== Combat Start ===".to_string());
    combat_log.add("Press SPACE to attack on your turn!".to_string());
}

/// Helper function to spawn a health bar for a character entity
fn spawn_health_bar(commands: &mut Commands, owner: Entity, position: Vec3) {
    const BAR_WIDTH: f32 = 80.0;
    const BAR_HEIGHT: f32 = 8.0;

    // Background (red)
    commands.spawn((
        HealthBarBackground,
        HealthBar { owner },
        Sprite {
            color: Color::srgb(0.6, 0.1, 0.1), // Dark red
            custom_size: Some(Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
            ..default()
        },
        Transform::from_translation(position),
    ));

    // Foreground (green) - will shrink as HP decreases
    commands.spawn((
        HealthBarForeground { owner },
        Sprite {
            color: Color::srgb(0.1, 0.8, 0.1), // Green
            custom_size: Some(Vec2::new(BAR_WIDTH, BAR_HEIGHT)),
            ..default()
        },
        Transform::from_translation(position + Vec3::new(0.0, 0.0, 0.1)),
    ));

    // Health text (HP: X / Y)
    commands.spawn((
        HealthText { owner },
        Text2d::new(""),
        TextFont {
            font_size: 16.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_translation(position + Vec3::new(0.0, 12.0, 0.2)),
    ));
}

/// System that handles player input and actions during PlayerTurn
fn handle_player_turn(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut combat_state: ResMut<CombatState>,
    mut combat_log: ResMut<CombatLog>,
    mut players: Query<(Entity, &mut logic::Character, &Transform, &Sprite), With<Player>>,
    mut monsters: Query<
        (Entity, &mut logic::Character, &Transform, &mut Sprite),
        (With<Monster>, Without<Player>),
    >,
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
    let (player_entity, player, _player_transform, player_sprite) =
        match players.iter_mut().next() {
            Some(p) => p,
            None => return,
        };

    let (monster_entity, mut monster, monster_transform, _monster_sprite) =
        match monsters.iter_mut().next() {
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

    // Spawn damage number at monster position
    spawn_damage_number(
        &mut commands,
        event.damage,
        monster_transform.translation + Vec3::new(0.0, 30.0, 10.0),
        Color::srgb(1.0, 0.3, 0.3), // Red damage numbers for player attacking
    );

    // Add attack flash to player
    commands.entity(player_entity).insert(AttackFlash {
        timer: Timer::from_seconds(0.15, TimerMode::Once),
        original_color: player_sprite.color,
    });

    // Add hit shake to monster
    commands.entity(monster_entity).insert(HitShake {
        timer: Timer::from_seconds(0.3, TimerMode::Once),
        original_position: monster_transform.translation,
        intensity: 5.0,
    });

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
    mut commands: Commands,
    mut timer: ResMut<MonsterTurnTimer>,
    mut combat_state: ResMut<CombatState>,
    mut combat_log: ResMut<CombatLog>,
    mut monster_abilities: ResMut<MonsterAbilities>,
    mut players: Query<
        (Entity, &mut logic::Character, &Transform, &mut Sprite),
        With<Player>,
    >,
    mut monsters: Query<
        (Entity, &mut logic::Character, &Transform, &Sprite),
        (With<Monster>, Without<Player>),
    >,
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

    // Tick cooldowns at the start of turn
    monster_abilities.tick_cooldowns();

    // Get player and monster
    let (monster_entity, mut monster, _monster_transform, monster_sprite) =
        match monsters.iter_mut().next() {
            Some(m) => m,
            None => return,
        };

    let (player_entity, mut player, player_transform, _player_sprite) =
        match players.iter_mut().next() {
            Some(p) => p,
            None => return,
        };

    // AI chooses the best action based on current health
    let ability = logic::choose_monster_action(&monster, &player, &monster_abilities.abilities);

    // Check if this is a heal ability (negative power)
    let is_heal = ability.power < 0;

    let event = if is_heal {
        // Healing: monster heals itself
        let event = logic::compute_attack(&monster, &monster, &ability);
        monster.hp = event.defender_hp_after.min(monster.stats.hp); // Cap at max HP
        event
    } else {
        // Attack: monster attacks player
        logic::compute_attack(&monster, &player, &ability)
    };

    // Activate cooldown for the ability used
    monster_abilities.activate_ability(&ability.name);

    // Log the action
    if is_heal {
        let heal_amount = -event.damage; // Negative damage = healing
        combat_log.add(format!(
            "{} uses {} and heals {} HP!",
            monster.name, ability.name, heal_amount
        ));

        // Spawn heal number at monster position
        spawn_damage_number(
            &mut commands,
            -heal_amount, // Display as negative to show healing
            _monster_transform.translation + Vec3::new(0.0, 30.0, 10.0),
            Color::srgb(0.2, 1.0, 0.2), // Green for healing
        );

        // Add flash effect to monster (green tint for heal)
        commands.entity(monster_entity).insert(AttackFlash {
            timer: Timer::from_seconds(0.15, TimerMode::Once),
            original_color: monster_sprite.color,
        });
    } else {
        combat_log.add(format!(
            "{} uses {} on {} for {} damage!",
            event.attacker_name, ability.name, event.defender_name, event.damage
        ));

        // Apply damage to player
        player.hp = event.defender_hp_after;

        // Spawn damage number at player position
        spawn_damage_number(
            &mut commands,
            event.damage,
            player_transform.translation + Vec3::new(0.0, 30.0, 10.0),
            Color::srgb(1.0, 0.6, 0.0), // Orange damage numbers for monster attacking
        );

        // Add attack flash to monster
        commands.entity(monster_entity).insert(AttackFlash {
            timer: Timer::from_seconds(0.15, TimerMode::Once),
            original_color: monster_sprite.color,
        });

        // Add hit shake to player
        commands.entity(player_entity).insert(HitShake {
            timer: Timer::from_seconds(0.3, TimerMode::Once),
            original_position: player_transform.translation,
            intensity: 5.0,
        });

        // Log HP status
        combat_log.add(format!(
            "{} HP: {} / {}",
            player.name, player.hp, player.stats.hp
        ));
    }

    // Log monster HP if healed
    if is_heal {
        combat_log.add(format!(
            "{} HP: {} / {}",
            monster.name, monster.hp, monster.stats.hp
        ));
    }

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
fn check_game_over(
    combat_state: Res<CombatState>,
    mut commands: Commands,
    mut combat_log: ResMut<CombatLog>,
    existing_message: Query<Entity, With<GameOverMessage>>,
) {
    // Only log once when entering game over state
    if !combat_state.is_changed() {
        return;
    }

    if let CombatState::GameOver { winner } = &*combat_state {
        combat_log.add("=== GAME OVER ===".to_string());
        combat_log.add(format!("{} wins!", winner));
        combat_log.add("Close the window to exit.".to_string());

        // Spawn game over message (only if not already spawned)
        if existing_message.is_empty() {
            let message_text = if winner == "Hero" {
                "VICTORY!"
            } else {
                "DEFEAT!"
            };
            let message_color = if winner == "Hero" {
                Color::srgb(0.2, 1.0, 0.2) // Green for victory
            } else {
                Color::srgb(1.0, 0.2, 0.2) // Red for defeat
            };

            commands.spawn((
                GameOverMessage,
                Text2d::new(message_text),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(message_color),
                Transform::from_translation(Vec3::new(0.0, 100.0, 100.0)),
            ));
        }
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

/// Helper function to spawn a floating damage number
fn spawn_damage_number(commands: &mut Commands, damage: i32, position: Vec3, color: Color) {
    commands.spawn((
        DamageNumber {
            lifetime: Timer::from_seconds(1.0, TimerMode::Once),
            rise_speed: 50.0,
        },
        Text2d::new(format!("{}", damage)),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(color),
        Transform::from_translation(position),
    ));
}

/// System that updates health bars based on character HP
fn update_health_bars(
    characters: Query<(Entity, &logic::Character)>,
    mut health_bars: Query<(&HealthBarForeground, &mut Sprite, &mut Transform)>,
    mut health_texts: Query<(&HealthText, &mut Text2d), Without<HealthBarForeground>>,
) {
    const BAR_WIDTH: f32 = 80.0;

    for (entity, character) in characters.iter() {
        // Update health bar foreground
        for (bar, mut sprite, mut transform) in health_bars.iter_mut() {
            if bar.owner == entity {
                let hp_ratio = (character.hp.max(0) as f32) / (character.stats.hp as f32);
                let new_width = BAR_WIDTH * hp_ratio;

                // Update sprite size
                sprite.custom_size = Some(Vec2::new(new_width, 8.0));

                // Adjust position to keep left-aligned
                let offset = (BAR_WIDTH - new_width) / 2.0;
                transform.translation.x -= offset;
            }
        }

        // Update health text
        for (text_comp, mut text) in health_texts.iter_mut() {
            if text_comp.owner == entity {
                **text = format!("{} / {}", character.hp.max(0), character.stats.hp);
            }
        }
    }
}

/// System that animates floating damage numbers
fn animate_damage_numbers(
    time: Res<Time>,
    mut commands: Commands,
    mut damage_numbers: Query<(Entity, &mut DamageNumber, &mut Transform, &mut TextColor)>,
) {
    for (entity, mut damage_num, mut transform, mut color) in damage_numbers.iter_mut() {
        // Tick timer
        damage_num.lifetime.tick(time.delta());

        // Move upward
        transform.translation.y += damage_num.rise_speed * time.delta_secs();

        // Fade out based on lifetime
        let progress = damage_num.lifetime.fraction();
        let alpha = 1.0 - progress;
        color.0 = color.0.with_alpha(alpha);

        // Remove when finished
        if damage_num.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// System that animates attack flash effects
fn animate_attack_flash(
    time: Res<Time>,
    mut commands: Commands,
    mut entities: Query<(Entity, &mut AttackFlash, &mut Sprite)>,
) {
    for (entity, mut flash, mut sprite) in entities.iter_mut() {
        // Tick timer
        flash.timer.tick(time.delta());

        // Flash white during attack
        let progress = flash.timer.fraction();
        let flash_color = Color::WHITE;
        sprite.color = flash.original_color.mix(&flash_color, 1.0 - progress);

        // Remove component when finished
        if flash.timer.finished() {
            sprite.color = flash.original_color;
            commands.entity(entity).remove::<AttackFlash>();
        }
    }
}

/// System that animates hit shake effects
fn animate_hit_shake(
    time: Res<Time>,
    mut commands: Commands,
    mut entities: Query<(Entity, &mut HitShake, &mut Transform)>,
) {
    use std::f32::consts::PI;

    for (entity, mut shake, mut transform) in entities.iter_mut() {
        // Tick timer
        shake.timer.tick(time.delta());

        if shake.timer.finished() {
            // Reset to original position
            transform.translation = shake.original_position;
            commands.entity(entity).remove::<HitShake>();
        } else {
            // Shake using sine wave
            let progress = shake.timer.fraction();
            let decay = 1.0 - progress;
            let shake_amount = shake.intensity * decay;

            // Oscillate on X axis
            let frequency = 20.0;
            let offset = (progress * frequency * 2.0 * PI).sin() * shake_amount;

            transform.translation.x = shake.original_position.x + offset;
            transform.translation.y = shake.original_position.y;
        }
    }
}
