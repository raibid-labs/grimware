//! Basic TUI rendering example
//!
//! Demonstrates terminal rendering of a simple 3D scene.
//!
//! Run with: cargo run --example tui_basic --features tui

use bevy::prelude::*;
use bevy_mcp_ratatui_ref::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "TUI Basic Example".to_string(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(BevyMcpTuiPlugin::default())
        .add_plugins(DemoSystemsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("Setting up TUI basic example scene...");

    // Spawn rotating cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Rotating { speed: 1.0 },
        Name::new("Rotating Cube"),
    ));

    // Spawn ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Ground Plane"),
    ));

    // Spawn light
    commands.spawn((
        PointLight {
            intensity: 2000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        Name::new("Main Light"),
    ));

    // Spawn camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-3.0, 3.0, 6.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        Name::new("Main Camera"),
    ));

    info!("✅ TUI basic scene setup complete");
    info!("📺 Check your terminal for 3D rendering!");
}
