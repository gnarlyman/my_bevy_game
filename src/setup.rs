use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin},
    prelude::*,
};
use crate::camera::{setup_camera, toggle_cursor_lock, camera_look, camera_movement};
use crate::debug_ui::{setup_debug_ui, update_debug_stats};
use crate::entities::spawn_entities;
use crate::lighting::setup_lighting;
use crate::orbital::update_orbits;
use crate::skybox::setup_skybox;
// Starfield removed in favor of skybox
// use crate::starfield::spawn_starfield;

/// Plugin that orchestrates all scene setup systems
pub struct SceneSetupPlugin;

impl Plugin for SceneSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add diagnostic plugins for performance monitoring
            .add_plugins((
                FrameTimeDiagnosticsPlugin::default(),
                EntityCountDiagnosticsPlugin::default(),
            ))
            // Set the space background color (black)
            .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            // Insert ambient light (space ambient light - increased for visibility)
            .insert_resource(AmbientLight {
                color: Color::srgb(0.15, 0.15, 0.2),
                brightness: 200.0,
                affects_lightmapped_meshes: false,
            })
            // Add setup systems (skybox must run after camera setup)
            .add_systems(Startup, (
                setup_camera,
                spawn_entities,
                setup_lighting,
                setup_debug_ui,
            ))
            .add_systems(Startup, setup_skybox.after(setup_camera))
            // Add runtime systems for camera control and orbital mechanics
            .add_systems(Update, (
                toggle_cursor_lock,
                camera_look,
                camera_movement,
                update_orbits,
                update_debug_stats,
            ));
    }
}

