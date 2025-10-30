use bevy::prelude::*;
use crate::camera::{setup_camera, toggle_cursor_lock, camera_look, camera_movement};
use crate::entities::spawn_entities;
use crate::lighting::setup_lighting;

/// Plugin that orchestrates all scene setup systems
pub struct SceneSetupPlugin;

impl Plugin for SceneSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            // Set the sky background color
            .insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92)))
            // Insert ambient light for overall sky illumination
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 300.0,
                affects_lightmapped_meshes: false,
            })
            // Add setup systems
            .add_systems(Startup, (
                setup_camera,
                spawn_entities,
                setup_lighting,
            ))
            // Add runtime systems for camera control
            .add_systems(Update, (
                toggle_cursor_lock,
                camera_look,
                camera_movement,
            ));
    }
}

