module Domain

type Stats =
    { Hp: int
      Attack: int
      Defense: int }

type Character =
    { Name: string
      Hp: int
      Stats: Stats }

type Ability =
    { Name: string
      Power: int }

type CombatEvent =
    { AttackerName: string
      DefenderName: string
      Damage: int
      DefenderHpAfter: int }
