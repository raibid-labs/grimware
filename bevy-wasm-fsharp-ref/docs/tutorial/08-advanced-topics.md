# Chapter 8: Advanced Topics

## What You'll Learn

In this final chapter, you'll explore advanced game development concepts including AI decision trees, ability systems, performance optimization, and future possibilities with F# transpilation.

**Time Required**: 20 minutes (reading) + unlimited (experimentation)

## AI Decision Systems

### Behavior Trees

Behavior trees provide flexible, modular AI:

```rust
// src/ai/behavior_tree.rs

use bevy::prelude::*;

/// Node types in the behavior tree
#[derive(Clone, Debug)]
pub enum BehaviorNode {
    // Composite nodes
    Sequence(Vec<BehaviorNode>),    // All must succeed
    Selector(Vec<BehaviorNode>),    // First success wins
    Parallel(Vec<BehaviorNode>),    // Run all simultaneously

    // Decorator nodes
    Inverter(Box<BehaviorNode>),    // Invert result
    Repeater(Box<BehaviorNode>, u32), // Repeat N times

    // Leaf nodes (actions)
    Action(ActionType),
    Condition(ConditionType),
}

#[derive(Clone, Debug)]
pub enum ActionType {
    Attack(Entity),
    Heal(Entity),
    MoveTo(Vec3),
    UseAbility(String),
    Flee,
}

#[derive(Clone, Debug)]
pub enum ConditionType {
    HealthBelow(f32),
    EnemyInRange(f32),
    AllyNeedsHealing,
    HasMana(i32),
}

#[derive(Clone, Copy, PartialEq)]
pub enum NodeStatus {
    Success,
    Failure,
    Running,
}

/// Execute a behavior tree node
pub fn execute_node(
    node: &BehaviorNode,
    entity: Entity,
    world: &World,
) -> NodeStatus {
    match node {
        BehaviorNode::Sequence(children) => {
            for child in children {
                match execute_node(child, entity, world) {
                    NodeStatus::Failure => return NodeStatus::Failure,
                    NodeStatus::Running => return NodeStatus::Running,
                    NodeStatus::Success => continue,
                }
            }
            NodeStatus::Success
        }

        BehaviorNode::Selector(children) => {
            for child in children {
                match execute_node(child, entity, world) {
                    NodeStatus::Success => return NodeStatus::Success,
                    NodeStatus::Running => return NodeStatus::Running,
                    NodeStatus::Failure => continue,
                }
            }
            NodeStatus::Failure
        }

        BehaviorNode::Condition(condition) => {
            evaluate_condition(condition, entity, world)
        }

        BehaviorNode::Action(action) => {
            execute_action(action, entity, world)
        }

        _ => NodeStatus::Success,
    }
}
```

### Utility AI

Make decisions based on scoring multiple factors:

```rust
// src/ai/utility.rs

use bevy::prelude::*;

/// Possible AI actions with utility scores
#[derive(Debug)]
pub struct ScoredAction {
    pub action: AIAction,
    pub score: f32,
}

#[derive(Debug, Clone)]
pub enum AIAction {
    AttackWeakest,
    AttackStrongest,
    HealLowestAlly,
    DefendSelf,
    UseOffensiveAbility,
    UseDefensiveAbility,
    Retreat,
}

/// Evaluate all possible actions and choose the best
pub fn evaluate_actions(
    entity: Entity,
    world: &World,
) -> AIAction {
    let mut actions = vec![
        score_attack_weakest(entity, world),
        score_attack_strongest(entity, world),
        score_heal_ally(entity, world),
        score_defend(entity, world),
        score_ability_use(entity, world),
        score_retreat(entity, world),
    ];

    // Add randomness for variety (0-10% variation)
    for action in &mut actions {
        action.score *= 1.0 + (rand::random::<f32>() * 0.1);
    }

    // Choose highest scoring action
    actions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    actions[0].action.clone()
}

fn score_attack_weakest(entity: Entity, world: &World) -> ScoredAction {
    let mut score = 50.0; // Base score

    // Increase score if we're winning
    if let Some(our_health) = get_health_percentage(entity, world) {
        score += (our_health * 20.0);
    }

    // Increase score if enemy is almost dead
    if let Some(enemy_health) = get_weakest_enemy_health(world) {
        score += (1.0 - enemy_health) * 30.0;
    }

    ScoredAction {
        action: AIAction::AttackWeakest,
        score,
    }
}

fn score_retreat(entity: Entity, world: &World) -> ScoredAction {
    let mut score = 0.0;

    // High score if health critical
    if let Some(health) = get_health_percentage(entity, world) {
        if health < 0.2 {
            score = 100.0;
        } else if health < 0.4 {
            score = 40.0;
        }
    }

    // Consider if outnumbered
    let enemy_count = count_enemies(world);
    let ally_count = count_allies(world);
    if enemy_count > ally_count * 2 {
        score += 30.0;
    }

    ScoredAction {
        action: AIAction::Retreat,
        score,
    }
}
```

### Goal-Oriented Action Planning (GOAP)

AI that plans sequences to achieve goals:

```rust
// src/ai/goap.rs

use std::collections::{HashSet, VecDeque};

/// World state representation
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WorldState {
    pub properties: HashSet<String>,
}

/// Action that changes world state
#[derive(Clone, Debug)]
pub struct GOAPAction {
    pub name: String,
    pub cost: f32,
    pub preconditions: HashSet<String>,
    pub effects: HashSet<String>,
}

/// Find plan to achieve goal
pub fn plan(
    current_state: WorldState,
    goal_state: WorldState,
    available_actions: Vec<GOAPAction>,
) -> Option<Vec<GOAPAction>> {
    #[derive(Clone)]
    struct Node {
        state: WorldState,
        actions: Vec<GOAPAction>,
        cost: f32,
    }

    let mut queue = VecDeque::new();
    queue.push_back(Node {
        state: current_state,
        actions: vec![],
        cost: 0.0,
    });

    while let Some(node) = queue.pop_front() {
        // Check if goal reached
        if goal_met(&node.state, &goal_state) {
            return Some(node.actions);
        }

        // Try each action
        for action in &available_actions {
            if preconditions_met(&node.state, &action.preconditions) {
                let mut new_state = node.state.clone();
                apply_effects(&mut new_state, &action.effects);

                let mut new_actions = node.actions.clone();
                new_actions.push(action.clone());

                queue.push_back(Node {
                    state: new_state,
                    actions: new_actions,
                    cost: node.cost + action.cost,
                });
            }
        }
    }

    None // No plan found
}

fn goal_met(current: &WorldState, goal: &WorldState) -> bool {
    goal.properties.is_subset(&current.properties)
}

fn preconditions_met(state: &WorldState, preconditions: &HashSet<String>) -> bool {
    preconditions.is_subset(&state.properties)
}

fn apply_effects(state: &mut WorldState, effects: &HashSet<String>) {
    for effect in effects {
        if effect.starts_with('!') {
            // Remove property
            state.properties.remove(&effect[1..]);
        } else {
            // Add property
            state.properties.insert(effect.clone());
        }
    }
}
```

## Ability Systems

### Flexible Ability Framework

```rust
// src/abilities/mod.rs

use bevy::prelude::*;
use std::collections::HashMap;

/// Ability definition
#[derive(Clone, Debug)]
pub struct AbilityDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: Handle<Image>,
    pub cooldown: f32,
    pub mana_cost: i32,
    pub cast_time: f32,
    pub range: f32,
    pub effects: Vec<AbilityEffect>,
    pub requirements: Vec<Requirement>,
}

#[derive(Clone, Debug)]
pub enum AbilityEffect {
    Damage { amount: i32, damage_type: DamageType },
    Heal { amount: i32 },
    ApplyBuff { buff: BuffType, duration: f32 },
    ApplyDebuff { debuff: DebuffType, duration: f32 },
    Summon { entity_type: String, count: u32 },
    Teleport { max_distance: f32 },
    AreaOfEffect { radius: f32, effect: Box<AbilityEffect> },
    Chain { max_targets: u32, effect: Box<AbilityEffect> },
}

#[derive(Clone, Debug)]
pub enum Requirement {
    Level(u32),
    Stat { stat: String, min_value: i32 },
    HasAbility(String),
    HasItem(String),
    InCombat(bool),
}

/// Runtime ability state
#[derive(Component)]
pub struct AbilitySlot {
    pub ability_id: String,
    pub cooldown_remaining: f32,
    pub charges: u32,
    pub max_charges: u32,
}

/// Ability registry
#[derive(Resource)]
pub struct AbilityRegistry {
    abilities: HashMap<String, AbilityDefinition>,
}

impl AbilityRegistry {
    pub fn register(&mut self, ability: AbilityDefinition) {
        self.abilities.insert(ability.id.clone(), ability);
    }

    pub fn get(&self, id: &str) -> Option<&AbilityDefinition> {
        self.abilities.get(id)
    }
}
```

### Combo System

Chain abilities for powerful effects:

```rust
// src/abilities/combos.rs

use bevy::prelude::*;

#[derive(Component)]
pub struct ComboTracker {
    pub sequence: Vec<String>,
    pub timer: Timer,
    pub max_sequence_length: usize,
}

#[derive(Resource)]
pub struct ComboRegistry {
    combos: Vec<Combo>,
}

#[derive(Clone)]
pub struct Combo {
    pub name: String,
    pub sequence: Vec<String>,  // Ability IDs in order
    pub reward_effect: ComboEffect,
    pub window: f32,  // Time window to complete
}

#[derive(Clone)]
pub enum ComboEffect {
    BonusDamage(f32),
    InstantCooldownReset,
    SpecialAbility(String),
    ResourceRefund(i32),
}

pub fn track_combos(
    mut combo_trackers: Query<&mut ComboTracker>,
    ability_used_events: EventReader<AbilityUsedEvent>,
    combo_registry: Res<ComboRegistry>,
    time: Res<Time>,
) {
    for mut tracker in &mut combo_trackers {
        tracker.timer.tick(time.delta());

        // Reset if timer expired
        if tracker.timer.finished() {
            tracker.sequence.clear();
            tracker.timer.reset();
        }

        // Check for completed combos
        for combo in &combo_registry.combos {
            if sequence_matches(&tracker.sequence, &combo.sequence) {
                // Trigger combo effect
                apply_combo_effect(&combo.reward_effect);
                tracker.sequence.clear();
            }
        }
    }
}

fn sequence_matches(tracker: &[String], combo: &[String]) -> bool {
    if tracker.len() < combo.len() {
        return false;
    }

    let start = tracker.len() - combo.len();
    &tracker[start..] == combo
}
```

## Performance Optimization

### System Parallelization

```rust
use bevy::prelude::*;
use bevy::ecs::schedule::*;

pub fn configure_parallel_systems(app: &mut App) {
    // These systems can run in parallel
    app.add_systems(
        Update,
        (
            update_animations,
            update_particle_effects,
            update_sound_effects,
        )
            .in_set(VisualSystems)
            .run_if(in_state(GameState::Playing)),
    );

    // These must run in order
    app.add_systems(
        Update,
        (
            process_input,
            apply_movement,
            check_collisions,
            resolve_collisions,
        )
            .chain()
            .in_set(PhysicsSystems),
    );

    // Configure system sets
    app.configure_sets(
        Update,
        (
            PhysicsSystems.before(VisualSystems),
            VisualSystems.before(UISystems),
        ),
    );
}
```

### Spatial Partitioning

```rust
// src/optimization/spatial.rs

use bevy::prelude::*;
use std::collections::HashMap;

/// Grid-based spatial partitioning
#[derive(Resource)]
pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        SpatialGrid {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn insert(&mut self, entity: Entity, position: Vec2) {
        let cell = self.get_cell(position);
        self.cells.entry(cell).or_default().push(entity);
    }

    pub fn query_radius(&self, center: Vec2, radius: f32) -> Vec<Entity> {
        let mut results = Vec::new();
        let cells_to_check = self.get_cells_in_radius(center, radius);

        for cell in cells_to_check {
            if let Some(entities) = self.cells.get(&cell) {
                results.extend(entities.iter());
            }
        }

        results
    }

    fn get_cell(&self, position: Vec2) -> (i32, i32) {
        (
            (position.x / self.cell_size).floor() as i32,
            (position.y / self.cell_size).floor() as i32,
        )
    }

    fn get_cells_in_radius(&self, center: Vec2, radius: f32) -> Vec<(i32, i32)> {
        let min_cell = self.get_cell(center - Vec2::splat(radius));
        let max_cell = self.get_cell(center + Vec2::splat(radius));

        let mut cells = Vec::new();
        for x in min_cell.0..=max_cell.0 {
            for y in min_cell.1..=max_cell.1 {
                cells.push((x, y));
            }
        }
        cells
    }
}
```

### Object Pooling

```rust
// src/optimization/pooling.rs

use bevy::prelude::*;

#[derive(Component)]
pub struct Pooled;

#[derive(Component)]
pub struct InUse;

#[derive(Resource)]
pub struct EntityPool {
    available: Vec<Entity>,
    archetype: EntityArchetype,
}

#[derive(Clone)]
pub struct EntityArchetype {
    pub components: Vec<Box<dyn PoolableComponent>>,
}

pub trait PoolableComponent: Send + Sync {
    fn clone_component(&self) -> Box<dyn PoolableComponent>;
    fn reset(&mut self);
}

impl EntityPool {
    pub fn preallocate(
        &mut self,
        commands: &mut Commands,
        count: usize,
    ) {
        for _ in 0..count {
            let entity = commands.spawn((
                Pooled,
                Visibility::Hidden,
            )).id();
            self.available.push(entity);
        }
    }

    pub fn get(&mut self, commands: &mut Commands) -> Entity {
        if let Some(entity) = self.available.pop() {
            commands.entity(entity)
                .insert(InUse)
                .remove::<Pooled>()
                .insert(Visibility::Visible);
            entity
        } else {
            // Create new if pool empty
            commands.spawn(InUse).id()
        }
    }

    pub fn return_entity(
        &mut self,
        commands: &mut Commands,
        entity: Entity,
    ) {
        commands.entity(entity)
            .remove::<InUse>()
            .insert(Pooled)
            .insert(Visibility::Hidden);
        self.available.push(entity);
    }
}
```

## Asset Management

### Dynamic Asset Loading

```rust
// src/assets/dynamic.rs

use bevy::prelude::*;
use bevy::asset::LoadState;

#[derive(Resource)]
pub struct DynamicAssets {
    handles: Vec<HandleUntyped>,
    loaded: bool,
}

pub fn load_level_assets(
    mut dynamic_assets: ResMut<DynamicAssets>,
    asset_server: Res<AssetServer>,
    level: u32,
) {
    dynamic_assets.handles.clear();

    // Load level-specific assets
    let level_path = format!("levels/level_{}/", level);

    dynamic_assets.handles.push(
        asset_server.load_untyped(&format!("{}background.png", level_path))
    );
    dynamic_assets.handles.push(
        asset_server.load_untyped(&format!("{}tilemap.ron", level_path))
    );
    dynamic_assets.handles.push(
        asset_server.load_untyped(&format!("{}music.ogg", level_path))
    );

    dynamic_assets.loaded = false;
}

pub fn check_assets_loaded(
    mut dynamic_assets: ResMut<DynamicAssets>,
    asset_server: Res<AssetServer>,
) {
    if !dynamic_assets.loaded {
        let all_loaded = dynamic_assets.handles.iter().all(|handle| {
            matches!(
                asset_server.get_load_state(handle.id()),
                Some(LoadState::Loaded)
            )
        });

        if all_loaded {
            dynamic_assets.loaded = true;
            info!("Level assets loaded!");
        }
    }
}
```

## Future: F# Transpilation

### The Vision

Instead of manually translating F# to Rust, automate it:

```fsharp
// F# source
[<Transpile>]
module Combat =
    let calculateDamage attacker defender =
        let damage = attacker.Attack - defender.Defense
        max 1 damage
```

Becomes:

```rust
// Generated Rust
pub mod combat {
    pub fn calculate_damage(attacker: &Character, defender: &Character) -> i32 {
        let damage = attacker.attack - defender.defense;
        damage.max(1)
    }
}
```

### Implementation Approach

```rust
// Potential transpiler architecture

pub struct FSharpTranspiler {
    parser: FSharpParser,
    type_mapper: TypeMapper,
    code_generator: RustGenerator,
}

impl FSharpTranspiler {
    pub fn transpile(&self, fsharp_code: &str) -> Result<String, Error> {
        // 1. Parse F# AST
        let ast = self.parser.parse(fsharp_code)?;

        // 2. Map F# types to Rust
        let rust_types = self.type_mapper.map_types(&ast)?;

        // 3. Generate Rust code
        let rust_code = self.code_generator.generate(&ast, &rust_types)?;

        Ok(rust_code)
    }
}
```

### Benefits of Transpilation

1. **Single Source of Truth**: F# remains the canonical domain model
2. **Automatic Updates**: Changes in F# automatically reflected in Rust
3. **Type Safety**: Preserve F#'s guarantees in generated Rust
4. **Optimization**: Transpiler can optimize for specific patterns

## Testing Strategies

### Property-Based Testing

```rust
#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn damage_always_positive(
            attack in 0i32..1000,
            defense in 0i32..1000
        ) {
            let damage = calculate_damage_raw(attack, defense);
            assert!(damage > 0);
        }

        #[test]
        fn healing_never_exceeds_max(
            current in 0i32..100,
            amount in 0i32..1000,
            max_hp in 1i32..100
        ) {
            let healed = apply_healing_raw(current, amount, max_hp);
            assert!(healed <= max_hp);
        }
    }
}
```

### Replay Testing

```rust
// Record and replay combat scenarios

#[derive(Serialize, Deserialize)]
pub struct CombatReplay {
    pub seed: u64,
    pub initial_state: GameState,
    pub inputs: Vec<TimestampedInput>,
}

pub fn record_combat(
    mut replay: ResMut<CombatReplay>,
    inputs: Res<InputBuffer>,
    time: Res<Time>,
) {
    for input in inputs.iter() {
        replay.inputs.push(TimestampedInput {
            timestamp: time.elapsed_seconds(),
            input: input.clone(),
        });
    }
}

pub fn replay_combat(
    replay: Res<CombatReplay>,
    mut rng: ResMut<StdRng>,
) {
    // Reset RNG to replay seed
    *rng = StdRng::seed_from_u64(replay.seed);

    // Replay all inputs at correct timestamps
    for input in &replay.inputs {
        // Apply input at correct time
    }
}
```

## Advanced Patterns

### Entity Relationships

```rust
// Parent-child relationships for complex entities

#[derive(Component)]
pub struct Mount {
    pub rider: Option<Entity>,
    pub mount_point: Vec3,
}

#[derive(Component)]
pub struct Rider {
    pub mount: Option<Entity>,
}

pub fn update_mount_positions(
    mut rider_query: Query<(&Rider, &mut Transform)>,
    mount_query: Query<(&Mount, &Transform), Without<Rider>>,
) {
    for (rider, mut rider_transform) in &mut rider_query {
        if let Some(mount_entity) = rider.mount {
            if let Ok((mount, mount_transform)) = mount_query.get(mount_entity) {
                rider_transform.translation =
                    mount_transform.translation + mount.mount_point;
            }
        }
    }
}
```

### Event-Driven Architecture

```rust
// Decouple systems through events

#[derive(Event)]
pub struct GameEvent {
    pub event_type: EventType,
    pub source: Entity,
    pub targets: Vec<Entity>,
    pub data: EventData,
}

#[derive(Debug)]
pub enum EventType {
    Combat(CombatEventType),
    Movement(MovementEventType),
    Inventory(InventoryEventType),
    Quest(QuestEventType),
}

pub fn event_dispatcher(
    mut events: EventReader<GameEvent>,
    mut combat_events: EventWriter<CombatEvent>,
    mut movement_events: EventWriter<MovementEvent>,
    // ... other event writers
) {
    for event in events.read() {
        match &event.event_type {
            EventType::Combat(combat_type) => {
                // Forward to combat system
            }
            EventType::Movement(movement_type) => {
                // Forward to movement system
            }
            // ... handle other event types
        }
    }
}
```

## Exercises

### Exercise 1: Implement Skill Trees

Design a skill tree system:

```rust
pub struct SkillTree {
    pub nodes: HashMap<String, SkillNode>,
    pub connections: Vec<(String, String)>,
}

pub struct SkillNode {
    pub id: String,
    pub name: String,
    pub max_rank: u32,
    pub current_rank: u32,
    pub prerequisites: Vec<String>,
}

// Implement:
// 1. Unlocking skills
// 2. Checking prerequisites
// 3. Calculating total points spent
// 4. Resetting the tree
```

### Exercise 2: Add Multiplayer Support

Design networking architecture:

```rust
// Consider:
// 1. Client-server vs peer-to-peer
// 2. State synchronization
// 3. Lag compensation
// 4. Cheat prevention
```

### Exercise 3: Create Modding Support

Make the game moddable:

```rust
// 1. Load external scripts (Lua, Rhai)
// 2. Custom ability definitions
// 3. User-generated content
// 4. Mod compatibility checking
```

## Resources for Further Learning

### Books
- "Game Programming Patterns" by Robert Nystrom
- "Real-Time Rendering" by Akenine-Möller et al.
- "AI for Games" by Ian Millington

### Online Resources
- [Bevy Cheat Book](https://bevy-cheatbook.github.io/)
- [F# for Fun and Profit](https://fsharpforfunandprofit.com/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)

### Communities
- [Bevy Discord](https://discord.gg/bevy)
- [F# Slack](https://fsharp.org/guides/slack/)
- [Rust GameDev WG](https://gamedev.rs/)

## Conclusion

You've completed the F# → Rust → Bevy tutorial series! You now have:

✅ **Domain Expertise**: Model complex game logic in F#
✅ **Implementation Skills**: Translate designs to performant Rust
✅ **Engine Knowledge**: Build games with Bevy ECS
✅ **Deployment Capability**: Ship to web via WASM
✅ **Advanced Techniques**: AI, optimization, and architecture

### Your Journey Continues

This tutorial provided the foundation. Now you can:

1. **Extend the Combat System**: Add more abilities, effects, and mechanics
2. **Build Different Games**: Apply these patterns to other genres
3. **Contribute to Tools**: Help build the F# transpiler
4. **Share Knowledge**: Write about your experiences

### Final Project Ideas

- **Roguelike Dungeon Crawler**: Procedural generation + combat system
- **Tower Defense**: Waves of enemies + strategic ability use
- **Auto-Battler**: AI vs AI with player strategy
- **MOBA Prototype**: Multiple heroes with unique abilities

Remember: The F# → Rust → Bevy workflow gives you the best of all worlds:
- **F#**: Unmatched domain modeling
- **Rust**: Performance and safety
- **Bevy**: Modern game engine features

Happy game development!

[← Previous: WASM Deployment](07-wasm-deployment.md) | [Tutorial Index](README.md)