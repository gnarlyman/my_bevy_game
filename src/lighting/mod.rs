use bevy::prelude::*;

/// System that spawns lighting
/// Note: No directional light - the star's point light is the sole light source
pub fn setup_lighting(mut _commands: Commands) {
    // Intentionally empty - star provides all lighting via its PointLight
    // Ambient light is configured in setup.rs
}

