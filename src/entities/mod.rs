use bevy::prelude::*;
use bevy::camera::visibility::NoFrustumCulling;
use crate::orbital::OrbitalBody;
use crate::gas_giant_textures::{create_amber_titan_texture, create_azure_colossus_texture};

/// Marker component for the home planet where camera starts
#[derive(Component)]
pub struct HomePlanet;

/// System that spawns the solar system (star and planets)
pub fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    info!("Generating gas giant textures...");
    let amber_texture = create_amber_titan_texture(&mut images);
    let azure_texture = create_azure_colossus_texture(&mut images);
    info!("Gas giant textures generated!");
    // Spawn central star (highly emissive, main light source)
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(8.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            // Much stronger emissive for bright star glow
            emissive: Color::srgb(50.0, 40.0, 10.0).into(),
            base_color: Color::srgb(1.0, 0.95, 0.7),
            // Unlit appearance - star should glow, not be lit by other sources
            unlit: true,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        // Strong point light - this should be the primary light source
        PointLight {
            intensity: 2_000_000.0,
            range: 250.0,
            color: Color::srgb(1.0, 0.95, 0.8),
            shadows_enabled: true,
            ..default()
        },
        // Prevent frustum culling so the light stays active even when star is off-screen
        NoFrustumCulling,
    ));

    // Planet 1: Home planet (small blue-green, closest orbit) - Cel shaded
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(2.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.6, 0.95),
            perceptual_roughness: 1.0,
            metallic: 0.0,
            reflectance: 0.0,
            ..default()
        })),
        Transform::from_xyz(18.0, 0.0, 0.0),
        OrbitalBody::new(18.0, 0.3, 0.0),
        HomePlanet,
    ));

    // Planet 2: Rocky red planet (medium orbit) - Cel shaded
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.95, 0.35, 0.25),
            perceptual_roughness: 1.0,
            metallic: 0.0,
            reflectance: 0.0,
            ..default()
        })),
        Transform::from_xyz(28.0, 0.0, 0.0),
        OrbitalBody::new(28.0, 0.2, std::f32::consts::FRAC_PI_2),
    ));

    // Planet 3: Gas Giant "Amber Titan" (large cream/tan gas giant with bands)
    commands.spawn((
        Mesh3d(meshes.add(
            Sphere::new(4.5)
                .mesh()
                .ico(7)  // High subdivision for smooth surface
                .unwrap()
        )),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(amber_texture.clone()),
            // No transmission effects
            diffuse_transmission: 0.0,
            specular_transmission: 0.0,
            thickness: 0.0,
            ior: 1.0,
            // Very matte to reduce lighting artifacts
            perceptual_roughness: 1.0,
            metallic: 0.0,
            reflectance: 0.0,
            // Subtle atmospheric glow
            emissive: Color::srgb(0.05, 0.04, 0.02).into(),
            ..default()
        })),
        Transform::from_xyz(42.0, 0.0, 0.0),
        OrbitalBody::new(42.0, 0.12, std::f32::consts::PI),
    ));

    // Planet 4: Small purple planet (medium-far orbit) - Cel shaded
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.75, 0.4, 0.85),
            perceptual_roughness: 1.0,
            metallic: 0.0,
            reflectance: 0.0,
            ..default()
        })),
        Transform::from_xyz(35.0, 0.0, 0.0),
        OrbitalBody::new(35.0, 0.15, std::f32::consts::FRAC_PI_4 * 3.0),
    ));

    // Planet 5: Gas Giant "Azure Colossus" (massive blue-white ice giant)
    commands.spawn((
        Mesh3d(meshes.add(
            Sphere::new(5.5)
                .mesh()
                .ico(7)  // High subdivision for smooth surface
                .unwrap()
        )),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            base_color_texture: Some(azure_texture.clone()),
            // No transmission effects
            diffuse_transmission: 0.0,
            specular_transmission: 0.0,
            thickness: 0.0,
            ior: 1.0,
            // Very matte to reduce lighting artifacts
            perceptual_roughness: 1.0,
            metallic: 0.0,
            reflectance: 0.0,
            // Subtle atmospheric glow
            emissive: Color::srgb(0.06, 0.08, 0.12).into(),
            ..default()
        })),
        Transform::from_xyz(50.0, 0.0, 0.0),
        OrbitalBody::new(50.0, 0.08, std::f32::consts::FRAC_PI_4),
    ));
}

