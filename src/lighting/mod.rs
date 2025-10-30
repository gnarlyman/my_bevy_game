use bevy::prelude::*;

/// System that spawns directional lighting (distant star light)
pub fn setup_lighting(mut commands: Commands) {
    // Dim directional light simulating distant starlight
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

