use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub hp: i32,
    pub stats: Stats,
}

impl Character {
    pub fn new_player(name: &str) -> Self {
        Self {
            name: name.into(),
            hp: 30,
            stats: Stats {
                hp: 30,
                attack: 10,
                defense: 2,
            },
        }
    }

    pub fn new_monster(name: &str) -> Self {
        Self {
            name: name.into(),
            hp: 20,
            stats: Stats {
                hp: 20,
                attack: 6,
                defense: 1,
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub name: String,
    pub power: i32,
}

impl Ability {
    pub fn basic_attack() -> Self {
        Self {
            name: "Basic Attack".into(),
            power: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatEvent {
    pub attacker_name: String,
    pub defender_name: String,
    pub damage: i32,
    pub defender_hp_after: i32,
}

pub fn compute_attack(attacker: &Character, defender: &Character, ability: &Ability) -> CombatEvent {
    let raw = attacker.stats.attack + ability.power;
    let dmg = (raw - defender.stats.defense).max(1);
    let hp_after = defender.hp - dmg;

    CombatEvent {
        attacker_name: attacker.name.clone(),
        defender_name: defender.name.clone(),
        damage: dmg,
        defender_hp_after: hp_after,
    }
}
