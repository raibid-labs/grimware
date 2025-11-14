use bevy::prelude::*;
use bevy_wasm_fsharp_ref_logic as logic;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Monster;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, tick_combat)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Player,
        logic::Character::new_player("Hero"),
        Transform::from_xyz(-100.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));

    commands.spawn((
        Monster,
        logic::Character::new_monster("Slime"),
        Transform::from_xyz(100.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));
}

fn tick_combat(
    keys: Res<Input<KeyCode>>,
    mut players: Query<&mut logic::Character, With<Player>>,
    mut monsters: Query<&mut logic::Character, (With<Monster>, Without<Player>)>,
) {
    if !keys.just_pressed(KeyCode::Space) {
        return;
    }

    let mut player = match players.get_single_mut() {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut monster = match monsters.get_single_mut() {
        Ok(m) => m,
        Err(_) => return,
    };

    let ability = logic::Ability::basic_attack();
    let event = logic::compute_attack(&player, &monster, &ability);

    println!("Combat event: {:?}", event);
    monster.hp = event.defender_hp_after;

    if monster.hp <= 0 {
        println!("Monster defeated!");
    }
}
