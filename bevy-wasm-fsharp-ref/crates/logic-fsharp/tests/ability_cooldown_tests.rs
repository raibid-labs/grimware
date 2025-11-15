// Ability cooldown system tests

use bevy_wasm_fsharp_ref_logic::*;

// ==================== AbilitySlot Tests ====================

#[test]
fn test_ability_slot_creation() {
    let slot = AbilitySlot::new(Ability::basic_attack(), 2.0);

    assert_eq!(slot.ability.name, "Basic Attack");
    assert_eq!(slot.cooldown_max, 2.0);
    assert_eq!(slot.cooldown_current, 0.0);
    assert!(slot.is_ready());
}

#[test]
fn test_ability_slot_use_triggers_cooldown() {
    let mut slot = AbilitySlot::new(Ability::powerful_attack(), 3.0);

    assert!(slot.is_ready());

    slot.use_ability();

    assert!(!slot.is_ready());
    assert_eq!(slot.cooldown_current, 3.0);
}

#[test]
fn test_ability_slot_tick_decreases_cooldown() {
    let mut slot = AbilitySlot::new(Ability::basic_attack(), 2.0);
    slot.use_ability();

    assert_eq!(slot.cooldown_current, 2.0);

    slot.tick(0.5);
    assert_eq!(slot.cooldown_current, 1.5);

    slot.tick(1.0);
    assert_eq!(slot.cooldown_current, 0.5);

    slot.tick(0.5);
    assert_eq!(slot.cooldown_current, 0.0);
    assert!(slot.is_ready());
}

#[test]
fn test_ability_slot_tick_does_not_go_negative() {
    let mut slot = AbilitySlot::new(Ability::quick_strike(), 0.2);

    assert_eq!(slot.cooldown_current, 0.0);

    slot.tick(1.0);
    assert_eq!(slot.cooldown_current, 0.0); // Should not go negative
}

#[test]
fn test_ability_slot_cooldown_progress() {
    let mut slot = AbilitySlot::new(Ability::powerful_attack(), 4.0);

    assert_eq!(slot.cooldown_progress(), 0.0); // Ready

    slot.use_ability();
    assert_eq!(slot.cooldown_progress(), 1.0); // Just used

    slot.tick(2.0);
    assert_eq!(slot.cooldown_progress(), 0.5); // Halfway

    slot.tick(2.0);
    assert_eq!(slot.cooldown_progress(), 0.0); // Ready again
}

#[test]
fn test_ability_slot_zero_cooldown() {
    let mut slot = AbilitySlot::new(Ability::basic_attack(), 0.0);

    assert!(slot.is_ready());

    slot.use_ability();
    assert!(slot.is_ready()); // Still ready with 0 cooldown

    assert_eq!(slot.cooldown_progress(), 0.0);
}

// ==================== AbilitySet Tests ====================

#[test]
fn test_ability_set_player_default() {
    let ability_set = AbilitySet::player_default();

    assert_eq!(ability_set.abilities.len(), 4);
    assert_eq!(ability_set.abilities[0].ability.name, "Basic Attack");
    assert_eq!(ability_set.abilities[1].ability.name, "Powerful Attack");
    assert_eq!(ability_set.abilities[2].ability.name, "Heal");
    assert_eq!(ability_set.abilities[3].ability.name, "Quick Strike");
}

#[test]
fn test_ability_set_monster_default() {
    let ability_set = AbilitySet::monster_default();

    assert_eq!(ability_set.abilities.len(), 2);
    assert_eq!(ability_set.abilities[0].ability.name, "Basic Attack");
    assert_eq!(ability_set.abilities[1].ability.name, "Quick Strike");
}

#[test]
fn test_ability_set_tick_all() {
    let mut ability_set = AbilitySet::player_default();

    // Use all abilities
    for slot in &mut ability_set.abilities {
        slot.use_ability();
        assert!(!slot.is_ready());
    }

    // Tick all cooldowns
    ability_set.tick_all(0.3);

    // Check quick strike (0.2s cooldown) is ready
    assert!(ability_set.abilities[3].is_ready());
    // Check others are still on cooldown
    assert!(!ability_set.abilities[0].is_ready()); // 0.5s cooldown, 0.2s remaining
    assert!(!ability_set.abilities[1].is_ready()); // 3.0s cooldown, 2.7s remaining
    assert!(!ability_set.abilities[2].is_ready()); // 5.0s cooldown, 4.7s remaining
}

#[test]
fn test_ability_set_get_ready_ability() {
    let mut ability_set = AbilitySet::player_default();

    // All abilities should be ready initially
    assert!(ability_set.get_ready_ability(0).is_some());
    assert!(ability_set.get_ready_ability(1).is_some());
    assert!(ability_set.get_ready_ability(2).is_some());
    assert!(ability_set.get_ready_ability(3).is_some());

    // Use ability 0
    if let Some(slot) = ability_set.get_ready_ability(0) {
        slot.use_ability();
    }

    // Ability 0 should no longer be ready
    assert!(ability_set.get_ready_ability(0).is_none());
    assert!(ability_set.get_ready_ability(1).is_some());
}

#[test]
fn test_ability_set_get_ready_ability_out_of_bounds() {
    let mut ability_set = AbilitySet::player_default();

    assert!(ability_set.get_ready_ability(10).is_none()); // Out of bounds
}

// ==================== Healing Ability Tests ====================

#[test]
fn test_heal_ability_restores_hp() {
    let attacker = Character::new_player("Hero");
    let mut defender = Character::new_player("Wounded Hero");
    defender.hp = 10; // Wounded

    let heal_ability = Ability::heal();
    let event = compute_attack(&attacker, &defender, &heal_ability);

    assert_eq!(event.damage, -8); // Negative damage = healing
    assert_eq!(event.defender_hp_after, 18); // 10 + 8 = 18
    assert_eq!(event.ability_used, "Heal");
}

#[test]
fn test_heal_bypasses_defense() {
    let attacker = Character::new_player("Hero");
    let mut defender = Character {
        name: "Tank".to_string(),
        hp: 20,
        stats: Stats {
            hp: 50,
            attack: 5,
            defense: 100, // Very high defense
        },
    };

    let heal_ability = Ability::heal();
    let event = compute_attack(&attacker, &defender, &heal_ability);

    // Healing should ignore defense entirely
    assert_eq!(event.damage, -8);
    assert_eq!(event.defender_hp_after, 28); // 20 + 8
}

#[test]
fn test_heal_can_exceed_original_hp() {
    // Note: In the app, HP should be capped at stats.hp
    // But the logic function itself doesn't enforce this
    let attacker = Character::new_player("Hero");
    let defender = Character::new_player("Full HP Hero");

    let heal_ability = Ability::heal();
    let event = compute_attack(&attacker, &defender, &heal_ability);

    assert_eq!(event.defender_hp_after, 38); // 30 + 8 = 38 (over max)
}

// ==================== Diverse Ability Tests ====================

#[test]
fn test_basic_attack_ability() {
    let ability = Ability::basic_attack();

    assert_eq!(ability.name, "Basic Attack");
    assert_eq!(ability.power, 5);
}

#[test]
fn test_powerful_attack_ability() {
    let ability = Ability::powerful_attack();

    assert_eq!(ability.name, "Powerful Attack");
    assert_eq!(ability.power, 12);
}

#[test]
fn test_heal_ability() {
    let ability = Ability::heal();

    assert_eq!(ability.name, "Heal");
    assert_eq!(ability.power, -8);
}

#[test]
fn test_quick_strike_ability() {
    let ability = Ability::quick_strike();

    assert_eq!(ability.name, "Quick Strike");
    assert_eq!(ability.power, 3);
}

#[test]
fn test_powerful_attack_vs_basic_attack() {
    let attacker = Character::new_player("Hero");
    let defender = Character::new_monster("Slime");

    let basic = Ability::basic_attack();
    let powerful = Ability::powerful_attack();

    let basic_event = compute_attack(&attacker, &defender, &basic);
    let powerful_event = compute_attack(&attacker, &defender, &powerful);

    // Powerful attack should deal more damage
    assert!(powerful_event.damage > basic_event.damage);
}

#[test]
fn test_quick_strike_vs_basic_attack() {
    let attacker = Character::new_player("Hero");
    let defender = Character::new_monster("Slime");

    let basic = Ability::basic_attack();
    let quick = Ability::quick_strike();

    let basic_event = compute_attack(&attacker, &defender, &basic);
    let quick_event = compute_attack(&attacker, &defender, &quick);

    // Quick strike should deal less damage than basic
    assert!(quick_event.damage < basic_event.damage);
}

// ==================== Integration Tests ====================

#[test]
fn test_full_combat_sequence_with_cooldowns() {
    let mut ability_set = AbilitySet::player_default();

    // Use powerful attack
    if let Some(slot) = ability_set.get_ready_ability(1) {
        assert_eq!(slot.ability.name, "Powerful Attack");
        slot.use_ability();
    }

    // Powerful attack should be on cooldown
    assert!(ability_set.get_ready_ability(1).is_none());

    // Basic attack should still be available
    if let Some(slot) = ability_set.get_ready_ability(0) {
        assert_eq!(slot.ability.name, "Basic Attack");
        slot.use_ability();
    }

    // Tick forward by 0.6 seconds
    ability_set.tick_all(0.6);

    // Basic attack should be ready again (0.5s cooldown)
    assert!(ability_set.get_ready_ability(0).is_some());

    // Powerful attack still on cooldown (3.0s, 2.4s remaining)
    assert!(ability_set.get_ready_ability(1).is_none());

    // Tick forward by 2.5 more seconds
    ability_set.tick_all(2.5);

    // Powerful attack should be ready now (total 3.1s elapsed)
    assert!(ability_set.get_ready_ability(1).is_some());
}

#[test]
fn test_combat_event_includes_ability_name() {
    let attacker = Character::new_player("Hero");
    let defender = Character::new_monster("Slime");
    let ability = Ability::powerful_attack();

    let event = compute_attack(&attacker, &defender, &ability);

    assert_eq!(event.ability_used, "Powerful Attack");
    assert_eq!(event.attacker_name, "Hero");
    assert_eq!(event.defender_name, "Slime");
}

#[test]
fn test_spam_protection() {
    let mut ability_set = AbilitySet::player_default();

    // Use quick strike (0.2s cooldown)
    if let Some(slot) = ability_set.get_ready_ability(3) {
        slot.use_ability();
    }

    // Should not be available immediately
    assert!(ability_set.get_ready_ability(3).is_none());

    // Tick by 0.1s (not enough)
    ability_set.tick_all(0.1);
    assert!(ability_set.get_ready_ability(3).is_none());

    // Tick by another 0.1s (total 0.2s, should be ready)
    ability_set.tick_all(0.1);
    assert!(ability_set.get_ready_ability(3).is_some());
}
