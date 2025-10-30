use bevy::prelude::*;

/// System that spawns directional lighting (sun) and ambient lighting (sky)
pub fn setup_lighting(mut commands: Commands) {
    // Directional light simulating the sun
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

