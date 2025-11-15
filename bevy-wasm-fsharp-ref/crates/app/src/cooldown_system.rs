//! Ability cooldown system implementation
//!
//! This module provides cooldown tracking, ability selection, and visual feedback
//! for the combat system

use bevy::prelude::*;
use bevy_wasm_fsharp_ref_logic as logic;

/// Component for ability button UI
#[derive(Component)]
pub struct AbilityButton {
    pub slot_index: usize,
}

/// Component for ability cooldown bar
#[derive(Component)]
pub struct CooldownBar {
    pub slot_index: usize,
}

/// System to tick ability cooldowns
pub fn tick_ability_cooldowns(
    time: Res<Time>,
    mut query: Query<&mut logic::AbilitySet>,
) {
    let delta = time.delta_secs();
    for mut ability_set in query.iter_mut() {
        ability_set.tick_all(delta);
    }
}

/// Helper to spawn ability UI panel
pub fn spawn_ability_ui(commands: &mut Commands) {
    let start_x = -300.0;
    let y = -250.0;
    const BUTTON_WIDTH: f32 = 120.0;
    const BUTTON_HEIGHT: f32 = 60.0;
    const BUTTON_SPACING: f32 = 130.0;

    // Create 4 ability buttons
    for i in 0..4 {
        let x = start_x + (i as f32 * BUTTON_SPACING);

        // Button background
        commands.spawn((
            AbilityButton { slot_index: i },
            Sprite {
                color: Color::srgb(0.2, 0.2, 0.3),
                custom_size: Some(Vec2::new(BUTTON_WIDTH, BUTTON_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(x, y, 5.0),
        ));

        // Cooldown overlay bar
        commands.spawn((
            CooldownBar { slot_index: i },
            Sprite {
                color: Color::srgba(0.1, 0.1, 0.1, 0.7),
                custom_size: Some(Vec2::new(0.0, BUTTON_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(x, y, 6.0),
        ));

        // Ability name text
        commands.spawn((
            AbilityButton { slot_index: i },
            Text2d::new(""),
            TextFont {
                font_size: 14.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(x, y + 15.0, 7.0),
        ));

        // Cooldown timer text
        commands.spawn((
            AbilityButton { slot_index: i },
            Text2d::new(""),
            TextFont {
                font_size: 11.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 0.8, 0.2)),
            Transform::from_xyz(x, y - 5.0, 7.0),
        ));

        // Keybind hint
        commands.spawn((
            Text2d::new(format!("[{}]", i + 1)),
            TextFont {
                font_size: 10.0,
                ..default()
            },
            TextColor(Color::srgb(0.5, 0.5, 0.5)),
            Transform::from_xyz(x, y - 22.0, 7.0),
        ));
    }
}

/// System to update ability UI with current status
pub fn update_ability_ui(
    players: Query<&logic::AbilitySet, With<crate::Player>>,
    mut buttons: Query<(&AbilityButton, &mut Sprite), Without<CooldownBar>>,
    mut cooldown_bars: Query<(&CooldownBar, &mut Sprite, &mut Transform), Without<AbilityButton>>,
    mut texts: Query<(&AbilityButton, &mut Text2d)>,
) {
    if let Ok(ability_set) = players.get_single() {
        const BUTTON_WIDTH: f32 = 120.0;

        // Update button colors and cooldown bars
        for (button, mut sprite) in buttons.iter_mut() {
            if let Some(slot) = ability_set.abilities.get(button.slot_index) {
                if slot.is_ready() {
                    sprite.color = Color::srgb(0.2, 0.6, 0.3); // Green when ready
                } else {
                    sprite.color = Color::srgb(0.3, 0.3, 0.4); // Gray when on cooldown
                }
            }
        }

        // Update cooldown overlay bars
        for (cooldown_bar, mut sprite, mut transform) in cooldown_bars.iter_mut() {
            if let Some(slot) = ability_set.abilities.get(cooldown_bar.slot_index) {
                let progress = slot.cooldown_progress();
                let bar_width = BUTTON_WIDTH * progress;
                sprite.custom_size = Some(Vec2::new(bar_width, 60.0));

                // Position bar from left
                let offset = (BUTTON_WIDTH - bar_width) / 2.0;
                transform.translation.x -= offset;
            }
        }

        // Update text labels
        for (button, mut text) in texts.iter_mut() {
            if let Some(slot) = ability_set.abilities.get(button.slot_index) {
                // Check if this is the ability name text (positioned higher)
                if text.0.contains('[') {
                    continue; // Skip keybind text
                }

                if text.0.is_empty() || !text.0.contains(':') {
                    // This is the ability name text
                    **text = slot.ability.name.clone();
                } else {
                    // This is the cooldown text
                    if slot.is_ready() {
                        **text = "READY".to_string();
                    } else {
                        **text = format!("CD: {:.1}s", slot.cooldown_current);
                    }
                }
            }
        }
    }
}
