//! # AI Behavior Example
//!
//! This example demonstrates different AI decision-making patterns for monster behavior.
//! It shows how to implement strategic AI that chooses abilities based on game state.
//!
//! ## What You'll Learn
//!
//! - How to implement AI decision trees
//! - Different AI personalities (aggressive, defensive, balanced)
//! - State-based ability selection
//! - How to test AI strategies against each other
//!
//! ## Run This Example
//!
//! ```bash
//! cargo run --example ai_behavior
//! ```
//!
//! ## Key Concepts
//!
//! AI behavior is implemented as **pure functions** that:
//! - Take game state as input (character HP, stats, etc.)
//! - Return an ability choice as output
//! - Have no side effects (can be tested easily)
//!
//! This functional approach aligns perfectly with F# → Rust transpilation.

use bevy_wasm_fsharp_ref_logic::{Ability, Character, compute_attack};
use std::io::{self, Write};

/// AI Strategy trait - defines how an AI makes decisions
trait AiStrategy {
    fn choose_ability(&self, ai_character: &Character, opponent: &Character) -> Ability;
    fn name(&self) -> &str;
}

/// Aggressive AI - always uses the strongest attack
struct AggressiveAi;

impl AiStrategy for AggressiveAi {
    fn choose_ability(&self, _ai_character: &Character, _opponent: &Character) -> Ability {
        // Always use the most powerful attack
        Ability {
            name: "Aggressive Strike".to_string(),
            power: 15,
        }
    }

    fn name(&self) -> &str {
        "Aggressive"
    }
}

/// Defensive AI - uses weaker attacks but focuses on survival
struct DefensiveAi;

impl AiStrategy for DefensiveAi {
    fn choose_ability(&self, ai_character: &Character, _opponent: &Character) -> Ability {
        // Use healing when HP is low (conceptual - would restore HP in real implementation)
        if ai_character.hp < ai_character.stats.hp / 2 {
            Ability {
                name: "Heal (Conceptual)".to_string(),
                power: 10, // In a real implementation, this would restore HP
            }
        } else {
            // Use basic attack when HP is healthy
            Ability::basic_attack()
        }
    }

    fn name(&self) -> &str {
        "Defensive"
    }
}

/// Balanced AI - adapts strategy based on opponent's HP
struct BalancedAi;

impl AiStrategy for BalancedAi {
    fn choose_ability(&self, _ai_character: &Character, opponent: &Character) -> Ability {
        let hp_percentage = (opponent.hp * 100) / opponent.stats.hp;

        if hp_percentage > 70 {
            // Opponent is healthy - use strong attack to pressure
            Ability {
                name: "Power Attack".to_string(),
                power: 12,
            }
        } else if hp_percentage > 30 {
            // Opponent is wounded - use balanced attack
            Ability::basic_attack()
        } else {
            // Opponent is critical - finish with ultimate
            Ability {
                name: "Finishing Blow".to_string(),
                power: 20,
            }
        }
    }

    fn name(&self) -> &str {
        "Balanced"
    }
}

/// Smart AI - considers both HP values and defense
struct SmartAi;

impl AiStrategy for SmartAi {
    fn choose_ability(&self, ai_character: &Character, opponent: &Character) -> Ability {
        let ai_hp_percent = (ai_character.hp * 100) / ai_character.stats.hp;
        let opponent_hp_percent = (opponent.hp * 100) / opponent.stats.hp;

        // If we're low HP and opponent is low HP, go aggressive
        if ai_hp_percent < 30 && opponent_hp_percent < 40 {
            return Ability {
                name: "Desperate Strike".to_string(),
                power: 25,
            };
        }

        // If opponent has high defense, use armor-breaking attack
        if opponent.stats.defense > 5 {
            return Ability {
                name: "Armor Break".to_string(),
                power: 18,
            };
        }

        // If we're healthy and opponent is healthy, build up
        if ai_hp_percent > 70 && opponent_hp_percent > 70 {
            return Ability {
                name: "Charge Attack".to_string(),
                power: 8,
            };
        }

        // Default: basic attack
        Ability::basic_attack()
    }

    fn name(&self) -> &str {
        "Smart"
    }
}

/// Run a combat simulation between two AI-controlled characters
fn simulate_combat(
    mut ai1_char: Character,
    ai1_strategy: &dyn AiStrategy,
    mut ai2_char: Character,
    ai2_strategy: &dyn AiStrategy,
    verbose: bool,
) -> String {
    let mut turn = 0;
    const MAX_TURNS: i32 = 20; // Prevent infinite loops

    if verbose {
        println!("\n=== Combat: {} ({}) vs {} ({}) ===",
            ai1_char.name, ai1_strategy.name(),
            ai2_char.name, ai2_strategy.name());
        println!("Starting HP: {} vs {}\n", ai1_char.hp, ai2_char.hp);
    }

    while ai1_char.hp > 0 && ai2_char.hp > 0 && turn < MAX_TURNS {
        turn += 1;

        if verbose {
            println!("Turn {}:", turn);
        }

        // AI 1's turn
        let ability1 = ai1_strategy.choose_ability(&ai1_char, &ai2_char);
        let event1 = compute_attack(&ai1_char, &ai2_char, &ability1);
        ai2_char.hp = event1.defender_hp_after;

        if verbose {
            println!("  {} uses {} → {} damage (Opponent HP: {})",
                ai1_char.name, ability1.name, event1.damage, ai2_char.hp);
        }

        if ai2_char.hp <= 0 {
            if verbose {
                println!("\n✓ {} wins in {} turns!\n", ai1_char.name, turn);
            }
            return ai1_char.name.clone();
        }

        // AI 2's turn
        let ability2 = ai2_strategy.choose_ability(&ai2_char, &ai1_char);
        let event2 = compute_attack(&ai2_char, &ai1_char, &ability2);
        ai1_char.hp = event2.defender_hp_after;

        if verbose {
            println!("  {} uses {} → {} damage (Opponent HP: {})\n",
                ai2_char.name, ability2.name, event2.damage, ai1_char.hp);
        }

        if ai1_char.hp <= 0 {
            if verbose {
                println!("✓ {} wins in {} turns!\n", ai2_char.name, turn);
            }
            return ai2_char.name.clone();
        }
    }

    if verbose {
        println!("Combat reached turn limit ({} turns)\n", MAX_TURNS);
    }

    // If we hit max turns, winner is whoever has more HP
    if ai1_char.hp > ai2_char.hp {
        ai1_char.name.clone()
    } else {
        ai2_char.name.clone()
    }
}

fn main() {
    println!("╔════════════════════════════════════════╗");
    println!("║   AI Behavior Example                  ║");
    println!("║   Strategic Decision Making            ║");
    println!("╚════════════════════════════════════════╝\n");

    // ====================
    // Section 1: AI Strategy Overview
    // ====================

    println!("=== 1. AI Strategies ===\n");
    println!("Available AI strategies:");
    println!("  • Aggressive: Always uses strongest attacks");
    println!("  • Defensive: Focuses on survival, heals when low HP");
    println!("  • Balanced: Adapts based on opponent's HP percentage");
    println!("  • Smart: Considers multiple factors (HP, defense, state)");
    io::stdout().flush().unwrap();

    // ====================
    // Section 2: Single Combat Demo
    // ====================

    println!("\n=== 2. Detailed Combat Demo ===");
    io::stdout().flush().unwrap();

    let fighter1 = Character::new_player("Warrior");
    let fighter2 = Character::new_monster("Goblin");

    let aggressive = AggressiveAi;
    let balanced = BalancedAi;

    simulate_combat(fighter1, &aggressive, fighter2, &balanced, true);

    // ====================
    // Section 3: AI Tournament
    // ====================

    println!("\n=== 3. AI Strategy Tournament ===\n");
    println!("Testing each AI strategy against all others (best of 5):\n");
    io::stdout().flush().unwrap();

    let strategies: Vec<(&str, Box<dyn AiStrategy>)> = vec![
        ("Aggressive", Box::new(AggressiveAi)),
        ("Defensive", Box::new(DefensiveAi)),
        ("Balanced", Box::new(BalancedAi)),
        ("Smart", Box::new(SmartAi)),
    ];

    // Track wins for each strategy
    let mut wins = vec![0; strategies.len()];

    for i in 0..strategies.len() {
        for j in (i + 1)..strategies.len() {
            let strategy1 = &strategies[i];
            let strategy2 = &strategies[j];

            print!("{} vs {}: ", strategy1.0, strategy2.0);

            let mut strategy1_wins = 0;
            let mut strategy2_wins = 0;

            // Best of 5
            for _ in 0..5 {
                let char1 = Character::new_player("Fighter1");
                let char2 = Character::new_player("Fighter2");

                let winner = simulate_combat(
                    char1.clone(),
                    strategy1.1.as_ref(),
                    char2.clone(),
                    strategy2.1.as_ref(),
                    false,
                );

                if winner == "Fighter1" {
                    strategy1_wins += 1;
                } else {
                    strategy2_wins += 1;
                }
            }

            if strategy1_wins > strategy2_wins {
                println!("✓ {} wins {}-{}", strategy1.0, strategy1_wins, strategy2_wins);
                wins[i] += 1;
            } else {
                println!("✓ {} wins {}-{}", strategy2.0, strategy2_wins, strategy1_wins);
                wins[j] += 1;
            }
            io::stdout().flush().unwrap();
        }
    }

    println!("\n=== Tournament Results ===\n");
    let mut results: Vec<_> = strategies.iter().zip(wins.iter()).collect();
    results.sort_by(|a, b| b.1.cmp(a.1));

    for (rank, (strategy, win_count)) in results.iter().enumerate() {
        println!("{}. {} - {} wins", rank + 1, strategy.0, win_count);
    }

    // ====================
    // Section 4: Implementing AI in F#
    // ====================

    println!("\n=== 4. F# Implementation Pattern ===\n");
    println!("To implement AI in F# (then transpile to Rust):\n");
    println!("```fsharp");
    println!("type AiStrategy = Character -> Character -> Ability");
    println!();
    println!("let aggressiveAi (aiChar: Character) (opponent: Character) : Ability =");
    println!("    {{ Name = \"Heavy Attack\"; Power = 15 }}");
    println!();
    println!("let balancedAi (aiChar: Character) (opponent: Character) : Ability =");
    println!("    let hpPercent = (opponent.Hp * 100) / opponent.Stats.Hp");
    println!("    if hpPercent > 70 then");
    println!("        {{ Name = \"Power Attack\"; Power = 12 }}");
    println!("    else");
    println!("        basicAttack");
    println!("```\n");

    println!("This F# code transpiles to Rust functions that can be called from Bevy systems!\n");

    println!("📚 Next Steps:");
    println!("  - Implement AI functions in fsharp/GameLogic.fs");
    println!("  - Add status effects and buffs");
    println!("  - Create more complex decision trees");
    println!("  - Try: cargo run --example fsharp_integration");
}
