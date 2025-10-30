use bevy::prelude::*;
use crate::orbital::OrbitalBody;

/// Marker component for the home planet where camera starts
#[derive(Component)]
pub struct HomePlanet;

/// System that spawns the solar system (star and planets)
pub fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn central star
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(8.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            emissive: Color::srgb(10.0, 8.0, 2.0).into(),
            base_color: Color::srgb(1.0, 0.9, 0.3),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        // Add point light to star
        PointLight {
            intensity: 1_000_000.0,
            range: 200.0,
            color: Color::srgb(1.0, 0.95, 0.8),
            shadows_enabled: true,
            ..default()
        },
    ));

    // Planet 1: Home planet (small blue-green, closest orbit)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(2.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.5, 0.8),
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(18.0, 0.0, 0.0),
        OrbitalBody::new(18.0, 0.3, 0.0),
        HomePlanet,
    ));

    // Planet 2: Rocky red planet (medium orbit)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.3, 0.2),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Transform::from_xyz(28.0, 0.0, 0.0),
        OrbitalBody::new(28.0, 0.2, std::f32::consts::FRAC_PI_2),
    ));

    // Planet 3: Gas giant (large, far orbit)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(4.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.7, 0.6, 0.4),
            perceptual_roughness: 0.6,
            ..default()
        })),
        Transform::from_xyz(42.0, 0.0, 0.0),
        OrbitalBody::new(42.0, 0.12, std::f32::consts::PI),
    ));

    // Planet 4: Small purple planet (medium-far orbit)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.3, 0.7),
            perceptual_roughness: 0.7,
            ..default()
        })),
        Transform::from_xyz(35.0, 0.0, 0.0),
        OrbitalBody::new(35.0, 0.15, std::f32::consts::FRAC_PI_4 * 3.0),
    ));

    // Planet 5: Icy white planet (farthest orbit)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.95, 1.0),
            perceptual_roughness: 0.3,
            metallic: 0.2,
            ..default()
        })),
        Transform::from_xyz(50.0, 0.0, 0.0),
        OrbitalBody::new(50.0, 0.08, std::f32::consts::FRAC_PI_4),
    ));
}

