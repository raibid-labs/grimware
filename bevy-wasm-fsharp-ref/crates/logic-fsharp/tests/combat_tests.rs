/// Tests for combat logic to verify F# ↔ Rust type alignment
/// These tests ensure the Rust implementation matches the F# specification

use bevy_wasm_fsharp_ref_logic::{Ability, Character, Stats, compute_attack};

#[test]
fn test_basic_attack_damage_calculation() {
    // Setup: Player with 10 attack, Monster with 1 defense
    let player = Character {
        name: "Hero".into(),
        hp: 30,
        stats: Stats {
            hp: 30,
            attack: 10,
            defense: 2,
        },
    };

    let monster = Character {
        name: "Goblin".into(),
        hp: 20,
        stats: Stats {
            hp: 20,
            attack: 6,
            defense: 1,
        },
    };

    let basic_attack = Ability {
        name: "Basic Attack".into(),
        power: 5,
    };

    // Execute: Player attacks monster
    let event = compute_attack(&player, &monster, &basic_attack);

    // Verify: Damage = max(1, 10 + 5 - 1) = 14
    assert_eq!(event.damage, 14, "Damage should be 14");
    assert_eq!(event.defender_hp_after, 6, "Monster HP should be 6 after attack");
    assert_eq!(event.attacker_name, "Hero");
    assert_eq!(event.defender_name, "Goblin");
}

#[test]
fn test_minimum_damage_one() {
    // Setup: Weak attacker vs strong defender
    let weak_attacker = Character {
        name: "Novice".into(),
        hp: 10,
        stats: Stats {
            hp: 10,
            attack: 1,
            defense: 0,
        },
    };

    let strong_defender = Character {
        name: "Tank".into(),
        hp: 50,
        stats: Stats {
            hp: 50,
            attack: 5,
            defense: 20,
        },
    };

    let weak_ability = Ability {
        name: "Poke".into(),
        power: 1,
    };

    // Execute: Weak attack against strong defense
    let event = compute_attack(&weak_attacker, &strong_defender, &weak_ability);

    // Verify: Minimum damage is 1, even if calculation is negative
    // raw = 1 + 1 = 2
    // dmg = max(1, 2 - 20) = max(1, -18) = 1
    assert_eq!(event.damage, 1, "Minimum damage should always be 1");
    assert_eq!(event.defender_hp_after, 49, "Defender HP should decrease by 1");
}

#[test]
fn test_character_constructors() {
    let player = Character::new_player("Hero");
    assert_eq!(player.name, "Hero");
    assert_eq!(player.hp, 30);
    assert_eq!(player.stats.hp, 30);
    assert_eq!(player.stats.attack, 10);
    assert_eq!(player.stats.defense, 2);

    let monster = Character::new_monster("Goblin");
    assert_eq!(monster.name, "Goblin");
    assert_eq!(monster.hp, 20);
    assert_eq!(monster.stats.hp, 20);
    assert_eq!(monster.stats.attack, 6);
    assert_eq!(monster.stats.defense, 1);
}

#[test]
fn test_ability_basic_attack() {
    let ability = Ability::basic_attack();
    assert_eq!(ability.name, "Basic Attack");
    assert_eq!(ability.power, 5);
}

#[test]
fn test_combat_event_fields() {
    let attacker = Character::new_player("Alice");
    let defender = Character::new_monster("Dragon");
    let ability = Ability::basic_attack();

    let event = compute_attack(&attacker, &defender, &ability);

    // Verify all fields are present and correctly named
    assert!(!event.attacker_name.is_empty());
    assert!(!event.defender_name.is_empty());
    assert!(event.damage > 0);
    // defender_hp_after can be negative (character death)
}

#[test]
fn test_overkill_damage() {
    let attacker = Character::new_player("Hero");
    let weak_defender = Character {
        name: "Slime".into(),
        hp: 5,
        stats: Stats {
            hp: 5,
            attack: 1,
            defense: 0,
        },
    };
    let ability = Ability::basic_attack();

    let event = compute_attack(&attacker, &weak_defender, &ability);

    // Damage exceeds defender's HP
    assert!(event.damage >= weak_defender.hp);
    // HP can go negative
    assert!(event.defender_hp_after <= 0);
}

#[test]
fn test_serialization_roundtrip() {
    use serde_json;

    let event = compute_attack(
        &Character::new_player("Alice"),
        &Character::new_monster("Goblin"),
        &Ability::basic_attack(),
    );

    // Serialize to JSON
    let json = serde_json::to_string(&event).expect("Serialization failed");

    // Deserialize back
    let deserialized: bevy_wasm_fsharp_ref_logic::CombatEvent =
        serde_json::from_str(&json).expect("Deserialization failed");

    // Verify fields match
    assert_eq!(event.attacker_name, deserialized.attacker_name);
    assert_eq!(event.defender_name, deserialized.defender_name);
    assert_eq!(event.damage, deserialized.damage);
    assert_eq!(event.defender_hp_after, deserialized.defender_hp_after);
}

#[test]
fn test_fsharp_alignment_example() {
    // This test mirrors the example in F# documentation
    // F#: let result = computeAttack player monster basicAttack
    // Expected: result.Damage = 14, result.DefenderHpAfter = 6

    let player = Character {
        name: "Hero".into(),
        hp: 30,
        stats: Stats {
            hp: 30,
            attack: 10,
            defense: 2,
        },
    };

    let monster = Character {
        name: "Goblin".into(),
        hp: 20,
        stats: Stats {
            hp: 20,
            attack: 6,
            defense: 1,
        },
    };

    let basic_attack = Ability {
        name: "Basic Attack".into(),
        power: 5,
    };

    let result = compute_attack(&player, &monster, &basic_attack);

    // Exact values from F# documentation example
    assert_eq!(result.damage, 14);
    assert_eq!(result.defender_hp_after, 6);
}
