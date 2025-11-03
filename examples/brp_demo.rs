use bevy::prelude::*;

#[cfg(feature = "brp")]
use bevy_brp_extras::BrpExtrasPlugin;

/// Example demonstrating BRP integration for MCP tools
/// Run with: cargo run --example brp_demo --features brp
///
/// This example shows:
/// - Setting up BRP for MCP integration
/// - Using Name components for easy entity identification
/// - Creating a scene that can be manipulated via MCP tools
fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "BRP Demo - MCP Interactive".to_string(),
            resolution: (1024., 768.).into(),
            ..default()
        }),
        ..default()
    }));

    // Add BRP support for MCP integration
    #[cfg(feature = "brp")]
    {
        // BrpExtrasPlugin includes RemotePlugin and RemoteHttpPlugin internally
        app.add_plugins(BrpExtrasPlugin)
            .register_type::<BouncingCube>();
        info!("🎮 BRP enabled on port 15702");
        info!("🚀 BRP Extras enabled - full mutation support!");
        info!("🤖 MCP tools can now interact with this game");
        info!("💡 Try querying entities, modifying transforms, or spawning objects!");
    }

    app.add_systems(Startup, setup)
    .add_systems(Update, (rotate_cubes, orbit_camera, update_instructions, bounce_green_cube))
    .run();
}

#[derive(Component)]
struct RotatingCube {
    speed: f32,
}

#[derive(Component)]
struct Instructions;

#[derive(Component)]
#[cfg_attr(feature = "brp", derive(bevy::reflect::Reflect))]
#[cfg_attr(feature = "brp", reflect(Component))]
struct BouncingCube {
    height: f32,
    speed: f32,
    base_height: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Setting up BRP demo scene...");

    // Spawn multiple cubes with different colors and speeds
    let cube_configs = [
        (Vec3::new(-3.0, 0.5, 0.0), Color::srgb(0.8, 0.2, 0.2), 0.5, "Red Cube"),
        (Vec3::new(0.0, 0.5, 0.0), Color::srgb(0.2, 0.8, 0.2), 1.0, "Green Cube"),
        (Vec3::new(3.0, 0.5, 0.0), Color::srgb(0.2, 0.2, 0.8), 1.5, "Blue Cube"),
    ];

    for (pos, color, speed, name) in cube_configs.iter() {
        let mut entity = commands.spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(*color)),
            Transform::from_translation(*pos),
            RotatingCube { speed: *speed },
            Name::new(*name),
        ));

        // Add bouncing component to green cube
        if *name == "Green Cube" {
            entity.insert(BouncingCube {
                height: 5.0,  // Jump high enough to clear the blue cube!
                speed: 2.0,   // Slower for dramatic effect
                base_height: 0.5,
            });
        }

        info!("Spawned: {}", name);
    }

    // Spawn ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(15.0, 15.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.3, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Ground Plane"),
    ));

    // Spawn multiple lights
    commands.spawn((
        PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            color: Color::srgb(1.0, 0.9, 0.8),
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        Name::new("Main Light"),
    ));

    commands.spawn((
        PointLight {
            intensity: 1000.0,
            color: Color::srgb(0.8, 0.8, 1.0),
            ..default()
        },
        Transform::from_xyz(-4.0, 4.0, -4.0),
        Name::new("Fill Light"),
    ));

    // Spawn orbiting camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 8.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("Orbiting Camera"),
    ));

    // Instructions text
    commands.spawn((
        Text::new("BRP Demo Active - Connect via MCP tools!"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        Instructions,
        Name::new("Instructions Text"),
    ));

    info!("✅ Scene setup complete!");
    info!("💡 TIP: Use mcp__brp__bevy_query to see all entities");
    info!("💡 TIP: Use mcp__brp__bevy_mutate_component to change cube colors");
    info!("💡 TIP: Use mcp__brp__bevy_spawn to add more objects");
}

fn rotate_cubes(time: Res<Time>, mut query: Query<(&mut Transform, &RotatingCube)>) {
    for (mut transform, cube) in &mut query {
        transform.rotate_y(time.delta_secs() * cube.speed);
    }
}

fn orbit_camera(time: Res<Time>, mut query: Query<&mut Transform, With<Camera3d>>) {
    for mut transform in &mut query {
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(time.delta_secs() * 0.2));
        transform.look_at(Vec3::ZERO, Vec3::Y);
    }
}

fn update_instructions(time: Res<Time>, mut query: Query<&mut Text, With<Instructions>>) {
    for mut text in &mut query {
        let elapsed = time.elapsed_secs() as i32;
        **text = format!(
            "BRP Demo Active ({:02}:{:02}) - Connect via MCP tools!",
            elapsed / 60,
            elapsed % 60
        );
    }
}

fn bounce_green_cube(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &BouncingCube)>
) {
    for (mut transform, bouncing) in &mut query {
        // Create a bouncing motion using sine wave
        let bounce = (time.elapsed_secs() * bouncing.speed).sin().abs();
        transform.translation.y = bouncing.base_height + (bounce * bouncing.height);
    }
}
