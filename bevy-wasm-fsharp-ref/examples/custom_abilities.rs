//! # Custom Abilities Example
//!
//! This example demonstrates how to create and use custom abilities in the combat system.
//! It shows the extensibility of the ability system and how different power levels affect combat.
//!
//! ## What You'll Learn
//!
//! - How to create custom abilities with different power levels
//! - How abilities affect damage calculations
//! - Multiple ability types (damage, heal, buff concepts)
//! - How to extend the type system for game-specific abilities
//!
//! ## Run This Example
//!
//! ```bash
//! cargo run --example custom_abilities
//! ```
//!
//! ## Key Concepts
//!
//! The Ability struct is simple but powerful:
//! - `name`: Display name for UI/logging
//! - `power`: Added to attacker's base attack stat
//!
//! Future enhancements could add:
//! - MP cost, cooldowns, status effects
//! - Multi-target abilities
//! - Conditional effects based on HP/state

use bevy_wasm_fsharp_ref_logic::{Ability, Character, Stats, compute_attack};

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   Custom Abilities Example            ║");
    println!("║   Extending the Ability System         ║");
    println!("╚════════════════════════════════════════╝\n");

    // ====================
    // Section 1: Define Custom Abilities
    // ====================

    println!("=== 1. Defining Custom Abilities ===\n");

    // Weak attack - low power
    let weak_strike = Ability {
        name: "Weak Strike".to_string(),
        power: 2,
    };
    println!("✓ Created: {} (Power: {})", weak_strike.name, weak_strike.power);

    // Standard attack (use the built-in helper)
    let basic_attack = Ability::basic_attack();
    println!("✓ Created: {} (Power: {})", basic_attack.name, basic_attack.power);

    // Powerful attack - high power, could have MP cost in the future
    let heavy_slash = Ability {
        name: "Heavy Slash".to_string(),
        power: 12,
    };
    println!("✓ Created: {} (Power: {})", heavy_slash.name, heavy_slash.power);

    // Ultimate attack - very high power
    let ultimate_attack = Ability {
        name: "Ultimate Strike".to_string(),
        power: 25,
    };
    println!("✓ Created: {} (Power: {})\n", ultimate_attack.name, ultimate_attack.power);

    // ====================
    // Section 2: Demonstrate Damage Scaling
    // ====================

    println!("=== 2. Testing Damage Scaling ===\n");

    let player = Character::new_player("Hero");
    let monster = Character::new_monster("Slime");

    println!(
        "Attacker: {} (ATK: {}, DEF: {})",
        player.name, player.stats.attack, player.stats.defense
    );
    println!(
        "Defender: {} (HP: {}, ATK: {}, DEF: {})\n",
        monster.name, monster.hp, monster.stats.attack, monster.stats.defense
    );

    // Test each ability and show damage
    let abilities = vec![weak_strike, basic_attack, heavy_slash, ultimate_attack];

    for ability in &abilities {
        let event = compute_attack(&player, &monster, ability);
        println!(
            "▶ {} deals {} damage (ATK {} + Power {} - DEF {} = {})",
            ability.name,
            event.damage,
            player.stats.attack,
            ability.power,
            monster.stats.defense,
            event.damage
        );
    }

    // ====================
    // Section 3: Healing Ability Concept
    // ====================

    println!("\n=== 3. Healing Ability Concept ===\n");
    println!("Note: The current logic crate doesn't have healing functions,");
    println!("but we can demonstrate how it could work in F# → Rust.\n");

    // This is what a healing ability might look like
    let heal_spell = Ability {
        name: "Heal".to_string(),
        power: 15, // Would restore 15 HP instead of dealing damage
    };

    println!("Conceptual Healing Ability:");
    println!("  Name: {}", heal_spell.name);
    println!("  Restore: {} HP", heal_spell.power);
    println!("\nIn F#, you would define:");
    println!("  type AbilityType = Damage | Heal | Buff");
    println!("  type Ability = {{ Name: string; Power: int; Type: AbilityType }}");
    println!("\nThen in Rust (via fsrs):");
    println!("  enum AbilityType {{ Damage, Heal, Buff }}");
    println!("  struct Ability {{ name: String, power: i32, ability_type: AbilityType }}");

    // ====================
    // Section 4: Advanced Ability Concepts
    // ====================

    println!("\n=== 4. Advanced Ability Concepts ===\n");

    // Example: Boss with high defense
    let boss = Character {
        name: "Dragon Boss".to_string(),
        hp: 100,
        stats: Stats {
            hp: 100,
            attack: 15,
            defense: 10, // High defense!
        },
    };

    println!("Fighting a high-defense enemy:");
    println!(
        "▶ Boss: {} (HP: {}, DEF: {})\n",
        boss.name, boss.hp, boss.stats.defense
    );

    // Show how different abilities perform against high defense
    let test_abilities = vec![
        Ability {
            name: "Weak Poke".to_string(),
            power: 1,
        },
        Ability::basic_attack(),
        Ability {
            name: "Armor Break".to_string(),
            power: 20,
        },
    ];

    for ability in &test_abilities {
        let event = compute_attack(&player, &boss, ability);
        println!(
            "▶ {} vs high DEF: {} damage (ATK {} + Power {} - DEF {} = max(1, {}))",
            ability.name,
            event.damage,
            player.stats.attack,
            ability.power,
            boss.stats.defense,
            event.damage
        );
    }

    println!("\nNote: Damage is always at least 1, even if defense > attack!");

    // ====================
    // Section 5: Multi-Ability Combat
    // ====================

    println!("\n=== 5. Multi-Ability Combat Simulation ===\n");

    let mut player = Character::new_player("Warrior");
    let mut enemy = Character::new_monster("Goblin");

    let player_abilities = vec![
        Ability::basic_attack(),
        Ability {
            name: "Power Strike".to_string(),
            power: 10,
        },
    ];

    println!("Player has access to multiple abilities:");
    for (i, ability) in player_abilities.iter().enumerate() {
        println!("  {}. {} (Power: {})", i + 1, ability.name, ability.power);
    }

    println!("\nSimulating strategic ability usage:\n");

    let mut turn = 0;
    while player.hp > 0 && enemy.hp > 0 && turn < 5 {
        turn += 1;
        println!("Turn {}:", turn);

        // Simple AI: Use power strike if enemy HP > 10, otherwise basic attack
        let chosen_ability = if enemy.hp > 10 {
            &player_abilities[1] // Power Strike
        } else {
            &player_abilities[0] // Basic Attack
        };

        let event = compute_attack(&player, &enemy, chosen_ability);
        enemy.hp = event.defender_hp_after;

        println!(
            "  Player uses {} → {} damage! (Enemy HP: {})",
            chosen_ability.name, event.damage, enemy.hp
        );

        if enemy.hp <= 0 {
            println!("\n✓ Victory! Enemy defeated in {} turns.", turn);
            break;
        }

        // Enemy counter-attack
        let counter = compute_attack(&enemy, &player, &Ability::basic_attack());
        player.hp = counter.defender_hp_after;

        println!(
            "  Enemy counters → {} damage (Player HP: {})\n",
            counter.damage, player.hp
        );
    }

    println!("\n📚 Next Steps:");
    println!("  - Implement these ability types in F# (fsharp/GameLogic.fs)");
    println!("  - Transpile to Rust via fsrs");
    println!("  - Integrate with Bevy for visual feedback");
    println!("  - Try: cargo run --example ai_behavior");
}
