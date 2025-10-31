use bevy::prelude::*;

/// System that spawns directional lighting (cel-shaded style)
pub fn setup_lighting(mut commands: Commands) {
    // Directional light for cel shading - stronger for more defined shadows
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

