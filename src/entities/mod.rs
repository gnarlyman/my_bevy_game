use bevy::prelude::*;

/// System that spawns various 3D shapes and a floor plane
pub fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn floor plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn a red cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(-2.0, 0.5, 0.0),
    ));

    // Spawn a blue sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.2, 0.8),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.6, 2.0),
    ));

    // Spawn a yellow cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.8, 1.5, 0.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.9, 0.2),
            ..default()
        })),
        Transform::from_xyz(2.0, 0.75, -1.0),
    ));

    // Spawn a green sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 0.4),
            ..default()
        })),
        Transform::from_xyz(-1.5, 0.5, -2.0),
    ));
}

