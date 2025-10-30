use bevy::prelude::*;

/// System that spawns a 3D camera positioned to view the scene
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 6.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

