use bevy::prelude::*;
use crate::camera::{setup_camera, toggle_cursor_lock, camera_look, camera_movement};
use crate::entities::spawn_entities;
use crate::lighting::setup_lighting;
use crate::orbital::update_orbits;
use crate::starfield::spawn_starfield;

/// Plugin that orchestrates all scene setup systems
pub struct SceneSetupPlugin;

impl Plugin for SceneSetupPlugin {
    fn build(&self, app: &mut App) {
        app
            // Set the space background color (black)
            .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
            // Insert ambient light for space (very dim)
            .insert_resource(AmbientLight {
                color: Color::srgb(0.05, 0.05, 0.1),
                brightness: 50.0,
                affects_lightmapped_meshes: false,
            })
            // Add setup systems
            .add_systems(Startup, (
                setup_camera,
                spawn_entities,
                spawn_starfield,
                setup_lighting,
            ))
            // Add runtime systems for camera control and orbital mechanics
            .add_systems(Update, (
                toggle_cursor_lock,
                camera_look,
                camera_movement,
                update_orbits,
            ));
    }
}

