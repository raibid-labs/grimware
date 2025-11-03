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

    // Spawn multiple spheres with different colors and speeds
    let sphere_configs = [
        (Vec3::new(-3.0, 0.5, 0.0), Color::srgb(0.8, 0.2, 0.2), 0.5, "Red Sphere"),
        (Vec3::new(0.0, 0.5, 0.0), Color::srgb(0.2, 0.8, 0.2), 1.0, "Green Sphere"),
        (Vec3::new(3.0, 0.5, 0.0), Color::srgb(0.2, 0.2, 0.8), 1.5, "Blue Sphere"),
    ];

    for (pos, color, speed, name) in sphere_configs.iter() {
        let mut entity = commands.spawn((
            Mesh3d(meshes.add(Sphere::new(0.5))),
            MeshMaterial3d(materials.add(*color)),
            Transform::from_translation(*pos),
            RotatingCube { speed: *speed },
            Name::new(*name),
        ));

        // Add bouncing component to green sphere
        if *name == "Green Sphere" {
            entity.insert(BouncingCube {
                height: 5.0,  // Jump high enough to clear the blue sphere!
                speed: 2.0,   // Slower for dramatic effect
                base_height: 0.5,
            });
        }

        info!("Spawned: {}", name);
    }

    // Add scenery - Trees (tall cylinders with sphere tops)
    for i in 0..5 {
        let angle = (i as f32 / 5.0) * std::f32::consts::PI * 2.0;
        let radius = 8.0;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        // Tree trunk
        commands.spawn((
            Mesh3d(meshes.add(Cylinder::new(0.2, 2.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.4, 0.25, 0.1))),
            Transform::from_xyz(x, 1.0, z),
            Name::new(format!("Tree Trunk {}", i + 1)),
        ));

        // Tree foliage
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(1.2))),
            MeshMaterial3d(materials.add(Color::srgb(0.1, 0.5, 0.1))),
            Transform::from_xyz(x, 2.5, z),
            Name::new(format!("Tree Foliage {}", i + 1)),
        ));
    }

    // Add rocks scattered around
    for i in 0..8 {
        let angle = (i as f32 / 8.0) * std::f32::consts::PI * 2.0 + 0.5;
        let radius = 6.0 + (i as f32 * 0.3);
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let size = 0.3 + (i as f32 * 0.05);

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(size))),
            MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
            Transform::from_xyz(x, size * 0.5, z).with_scale(Vec3::new(1.0, 0.6, 1.2)),
            Name::new(format!("Rock {}", i + 1)),
        ));
    }

    // Spawn ground plane (larger and grass-colored)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(25.0, 25.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.6, 0.2))),
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
    info!("💡 TIP: Use mcp__brp__bevy_mutate_component to change sphere colors");
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
        // Create an arc trajectory that jumps over the blue sphere
        let t = (time.elapsed_secs() * bouncing.speed) % (std::f32::consts::PI * 2.0);

        // Horizontal motion: oscillate between -4 and +4 (passing over blue sphere at x=3)
        let x_range = 4.0;
        transform.translation.x = (t.cos()) * x_range;

        // Vertical motion: parabolic arc (peaks when horizontal velocity is zero)
        // Use sin for smooth parabola that peaks at the extremes
        let arc_height = t.sin().abs();
        transform.translation.y = bouncing.base_height + (arc_height * bouncing.height);
    }
}
