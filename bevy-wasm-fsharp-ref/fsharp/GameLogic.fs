module GameLogic

open Domain

let basicAttack =
    { Name = "Basic Attack"
      Power = 5 }

let computeAttack (attacker: Character) (defender: Character) (ability: Ability) : CombatEvent =
    let raw = attacker.Stats.Attack + ability.Power
    let dmg = max 1 (raw - defender.Stats.Defense)
    let hpAfter = defender.Hp - dmg
    { AttackerName = attacker.Name
      DefenderName = defender.Name
      Damage = dmg
      DefenderHpAfter = hpAfter }
