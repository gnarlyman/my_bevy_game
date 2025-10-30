use bevy::prelude::*;
use rand::Rng;

/// System that spawns a starfield background
pub fn spawn_starfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let starfield_radius = 200.0;
    let star_count = 500;
    
    for _ in 0..star_count {
        // Generate random point on a sphere surface
        let theta = rng.gen_range(0.0..std::f32::consts::TAU);
        let phi = rng.gen_range(0.0..std::f32::consts::PI);
        
        let x = starfield_radius * phi.sin() * theta.cos();
        let y = starfield_radius * phi.sin() * theta.sin();
        let z = starfield_radius * phi.cos();
        
        // Random star size (very small)
        let size = rng.gen_range(0.05..0.15);
        
        // Random star color (white to slight blue tint)
        let color_intensity = rng.gen_range(0.8..1.0);
        let blue_tint = rng.gen_range(0.9..1.0);
        
        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(size))),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive: Color::srgb(color_intensity, color_intensity, blue_tint).into(),
                ..default()
            })),
            Transform::from_xyz(x, y, z),
        ));
    }
}

