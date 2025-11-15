//! # Headless Simulation Example
//!
//! This example demonstrates running combat simulations without a window or rendering.
//! Perfect for testing, benchmarking, and server-side game logic.
//!
//! ## What You'll Learn
//!
//! - How to run Bevy apps headless (no window)
//! - Using MinimalPlugins for logic-only applications
//! - Batch simulation and statistics gathering
//! - Performance benchmarking techniques
//!
//! ## Run This Example
//!
//! ```bash
//! cargo run --example headless_simulation
//! ```
//!
//! ## Use Cases
//!
//! - **Testing**: Run thousands of combat scenarios
//! - **Benchmarking**: Measure performance without rendering overhead
//! - **AI Training**: Generate training data for machine learning
//! - **Server**: Run game logic on a server without graphics
//! - **CI/CD**: Automated testing in headless environments

use bevy::app::{App, AppExit, Startup, Update};
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::{Commands, Query, ResMut, Resource};
use bevy::log::{Level, LogPlugin};
use bevy::prelude::{Component, MinimalPlugins};
use bevy_wasm_fsharp_ref_logic::{Ability, Character, compute_attack};

/// Marker component for player entities
#[derive(Component)]
struct Player;

/// Marker component for monster entities
#[derive(Component)]
struct Monster;

/// Resource tracking simulation state
#[derive(Resource)]
struct SimulationState {
    turn: i32,
    max_turns: i32,
    completed_simulations: i32,
    total_simulations: i32,
    player_wins: i32,
    monster_wins: i32,
    total_turns: i32,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            turn: 0,
            max_turns: 100,
            completed_simulations: 0,
            total_simulations: 100, // Run 100 simulations
            player_wins: 0,
            monster_wins: 0,
            total_turns: 0,
        }
    }
}

/// System to set up a single combat scenario
fn setup_combat(mut commands: Commands) {
    // Spawn player
    commands.spawn((Player, Character::new_player("Hero")));

    // Spawn monster
    commands.spawn((Monster, Character::new_monster("Slime")));
}

/// System that runs the combat simulation (headless)
fn run_combat(
    mut commands: Commands,
    mut state: ResMut<SimulationState>,
    mut players: Query<&mut Character, bevy::ecs::query::With<Player>>,
    mut monsters: Query<
        &mut Character,
        (bevy::ecs::query::With<Monster>, bevy::ecs::query::Without<Player>),
    >,
    mut exit: EventWriter<AppExit>,
) {
    // Check if we're done with all simulations
    if state.completed_simulations >= state.total_simulations {
        // Print final statistics
        print_statistics(&state);
        exit.send(AppExit::Success);
        return;
    }

    // Get player and monster
    let Ok(mut player) = players.get_single_mut() else {
        return;
    };
    let Ok(mut monster) = monsters.get_single_mut() else {
        return;
    };

    // Run combat turn
    state.turn += 1;

    if state.turn > state.max_turns {
        // Turn limit reached - end this simulation
        println!(
            "Simulation {} reached turn limit",
            state.completed_simulations + 1
        );
        reset_combat(&mut commands, &mut state, &mut player, &mut monster);
        return;
    }

    let ability = Ability::basic_attack();

    // Player attacks
    let event = compute_attack(&player, &monster, &ability);
    monster.hp = event.defender_hp_after;

    if monster.hp <= 0 {
        // Player wins
        state.player_wins += 1;
        state.total_turns += state.turn;
        println!(
            "Simulation {}/{}: Player wins in {} turns",
            state.completed_simulations + 1,
            state.total_simulations,
            state.turn
        );
        reset_combat(&mut commands, &mut state, &mut player, &mut monster);
        return;
    }

    // Monster attacks
    let event = compute_attack(&monster, &player, &ability);
    player.hp = event.defender_hp_after;

    if player.hp <= 0 {
        // Monster wins
        state.monster_wins += 1;
        state.total_turns += state.turn;
        println!(
            "Simulation {}/{}: Monster wins in {} turns",
            state.completed_simulations + 1,
            state.total_simulations,
            state.turn
        );
        reset_combat(&mut commands, &mut state, &mut player, &mut monster);
    }
}

/// Helper function to reset combat state for next simulation
fn reset_combat(
    _commands: &mut Commands,
    state: &mut SimulationState,
    player: &mut Character,
    monster: &mut Character,
) {
    state.completed_simulations += 1;
    state.turn = 0;

    // Reset characters to full HP
    *player = Character::new_player("Hero");
    *monster = Character::new_monster("Slime");
}

/// Print final statistics
fn print_statistics(state: &SimulationState) {
    println!("\n╔════════════════════════════════════════╗");
    println!("║   Simulation Results                   ║");
    println!("╚════════════════════════════════════════╝\n");

    println!("Total Simulations: {}", state.completed_simulations);
    println!("Player Wins: {} ({:.1}%)",
        state.player_wins,
        (state.player_wins as f64 / state.completed_simulations as f64) * 100.0
    );
    println!("Monster Wins: {} ({:.1}%)",
        state.monster_wins,
        (state.monster_wins as f64 / state.completed_simulations as f64) * 100.0
    );

    if state.completed_simulations > 0 {
        let avg_turns = state.total_turns as f64 / state.completed_simulations as f64;
        println!("Average Combat Length: {:.2} turns", avg_turns);
    }

    println!();
}

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   Headless Simulation Example          ║");
    println!("║   Running Combat Without Rendering     ║");
    println!("╚════════════════════════════════════════╝\n");

    println!("Starting 100 combat simulations...\n");

    App::new()
        // Use MinimalPlugins instead of DefaultPlugins - no window, rendering, or input
        .add_plugins(MinimalPlugins)
        // Optional: Add logging for debugging
        .add_plugins(LogPlugin {
            level: Level::WARN, // Reduce log spam
            filter: "wgpu=error,bevy_render=error,bevy_ecs=warn".to_string(),
            ..Default::default()
        })
        // Add our simulation state
        .init_resource::<SimulationState>()
        // Setup system runs once
        .add_systems(Startup, setup_combat)
        // Update system runs every frame
        .add_systems(Update, run_combat)
        .run();

    println!("\n=== Headless Simulation Benefits ===\n");
    println!("✓ No window or graphics overhead");
    println!("✓ Faster execution (no rendering delay)");
    println!("✓ Runs in CI/CD environments");
    println!("✓ Perfect for testing and benchmarking");
    println!("✓ Can run thousands of simulations quickly\n");

    println!("📚 Use Cases:");
    println!("  - Automated testing: cargo test");
    println!("  - Performance benchmarking: cargo bench");
    println!("  - AI training data generation");
    println!("  - Server-side game logic");
    println!("  - Balance testing (iterate on stats and re-run)\n");

    println!("💡 Tips:");
    println!("  - Increase total_simulations for better statistics");
    println!("  - Adjust character stats to test balance");
    println!("  - Add timing code to measure performance");
    println!("  - Export results to CSV for analysis\n");

    println!("🔧 Performance Notes:");
    println!("  MinimalPlugins includes:");
    println!("    • TaskPoolPlugin (multi-threading)");
    println!("    • TypeRegistrationPlugin (reflection)");
    println!("    • FrameCountPlugin (frame tracking)");
    println!("    • TimePlugin (delta time)");
    println!("    • ScheduleRunnerPlugin (app loop)\n");

    println!("  Excluded (saves resources):");
    println!("    • Window management");
    println!("    • Rendering");
    println!("    • Input handling");
    println!("    • Audio");
    println!("    • Asset loading\n");

    println!("📚 Next Steps:");
    println!("  - Modify SimulationState.total_simulations for more runs");
    println!("  - Test different character stats and abilities");
    println!("  - Add performance metrics with std::time::Instant");
    println!("  - Export data for visualization");
}
