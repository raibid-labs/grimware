//! # Minimal Combat Example
//!
//! This example demonstrates the absolute minimum code needed to run a combat
//! simulation using the bevy-wasm-fsharp-ref logic crate.
//!
//! ## What You'll Learn
//!
//! - How to create characters (player and monster)
//! - How to create and use abilities
//! - How to execute combat with `compute_attack()`
//! - Basic turn-based combat loop
//! - Console-based combat output (no rendering)
//!
//! ## Run This Example
//!
//! ```bash
//! cargo run --example minimal_combat
//! ```
//!
//! ## Expected Output
//!
//! You'll see a turn-by-turn combat log showing:
//! - Each character's starting HP
//! - Damage dealt each turn
//! - Remaining HP after each attack
//! - Final winner announcement
//!
//! ## Key Concepts
//!
//! This example showcases the **functional core** of the game logic:
//! - Pure functions with no side effects
//! - Immutable character data (we clone and update HP)
//! - Deterministic combat resolution

use bevy_wasm_fsharp_ref_logic::{Ability, Character, compute_attack};

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   Minimal Combat Example              ║");
    println!("║   Bevy WASM F# Reference Project      ║");
    println!("╚════════════════════════════════════════╝\n");

    // Step 1: Create combatants
    // Characters come with predefined stats:
    // - Player: 30 HP, 10 Attack, 2 Defense
    // - Monster: 20 HP, 6 Attack, 1 Defense
    let mut player = Character::new_player("Hero");
    let mut monster = Character::new_monster("Slime");

    // Step 2: Create an ability
    // The basic attack has 5 power and is available to all characters
    let basic_attack = Ability::basic_attack();

    // Display starting state
    println!("=== Combat Start ===");
    println!(
        "Player: {} (HP: {}/{}, ATK: {}, DEF: {})",
        player.name, player.hp, player.stats.hp, player.stats.attack, player.stats.defense
    );
    println!(
        "Monster: {} (HP: {}/{}, ATK: {}, DEF: {})\n",
        monster.name, monster.hp, monster.stats.hp, monster.stats.attack, monster.stats.defense
    );

    println!("Combat Formula: damage = max(1, attacker.attack + ability.power - defender.defense)\n");

    // Step 3: Combat loop
    // Continue until one combatant reaches 0 HP or below
    let mut turn = 0;
    while player.hp > 0 && monster.hp > 0 {
        turn += 1;
        println!("--- Turn {} ---", turn);

        // Player attacks first
        // compute_attack() returns a CombatEvent with damage and new HP
        let event = compute_attack(&player, &monster, &basic_attack);
        monster.hp = event.defender_hp_after;

        println!(
            "▶ {} attacks {} for {} damage! (Monster HP: {} → {})",
            event.attacker_name,
            event.defender_name,
            event.damage,
            monster.hp + event.damage,
            monster.hp
        );

        // Check if monster is defeated
        if monster.hp <= 0 {
            println!("\n🎉 {} wins the battle!", player.name);
            println!("Final HP: {}/{}\n", player.hp, player.stats.hp);
            break;
        }

        // Monster counter-attacks
        let event = compute_attack(&monster, &player, &basic_attack);
        player.hp = event.defender_hp_after;

        println!(
            "▶ {} attacks {} for {} damage! (Player HP: {} → {})",
            event.attacker_name,
            event.defender_name,
            event.damage,
            player.hp + event.damage,
            player.hp
        );

        // Check if player is defeated
        if player.hp <= 0 {
            println!("\n💀 {} wins the battle!", monster.name);
            println!("Better luck next time!\n");
            break;
        }

        println!(); // Blank line between turns
    }

    println!("=== Combat End ===");
    println!("\n📚 Learn More:");
    println!("  - Try other examples: cargo run --example custom_abilities");
    println!("  - Read the docs: docs/combat-system.md");
    println!("  - Explore the logic crate: crates/logic-fsharp/src/lib.rs");
}
