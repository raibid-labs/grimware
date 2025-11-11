//! TUI rendering plugin implementation

use bevy::prelude::*;
use bevy_ratatui_camera::RatatuiCamera;

use super::config::{TuiConfig, TuiRenderMode};

/// Main plugin for TUI rendering integration
pub struct BevyMcpTuiPlugin {
    config: TuiConfig,
}

impl Default for BevyMcpTuiPlugin {
    fn default() -> Self {
        Self {
            config: TuiConfig::default(),
        }
    }
}

impl BevyMcpTuiPlugin {
    /// Create a new TUI plugin with custom configuration
    pub fn new(config: TuiConfig) -> Self {
        Self { config }
    }
}

impl Plugin for BevyMcpTuiPlugin {
    fn build(&self, app: &mut App) {
        // Insert configuration as resource
        app.insert_resource(self.config.clone());

        if self.config.enabled {
            // Add startup system to configure TUI camera
            app.add_systems(Startup, setup_tui_camera);

            info!("TUI rendering enabled with mode: {:?}", self.config.render_mode);
        }
    }
}

/// System to setup TUI camera on existing cameras
fn setup_tui_camera(
    mut commands: Commands,
    cameras: Query<Entity, With<Camera3d>>,
    config: Res<TuiConfig>,
) {
    // Find the first camera and add RatatuiCamera component
    if let Some(camera_entity) = cameras.iter().next() {
        commands.entity(camera_entity).insert(RatatuiCamera::default());
        info!("✅ TUI camera configured on entity {:?}", camera_entity);
    } else {
        warn!("⚠️  No Camera3d found to attach TUI rendering");
    }
}
