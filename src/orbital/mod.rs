use bevy::prelude::*;

/// Component for bodies that orbit around a central point
#[derive(Component)]
pub struct OrbitalBody {
    /// Distance from the center point
    pub orbital_radius: f32,
    /// Speed of rotation in radians per second
    pub orbital_speed: f32,
    /// Current angle in radians
    pub current_angle: f32,
    /// Center point to orbit around
    pub center: Vec3,
}

impl OrbitalBody {
    pub fn new(radius: f32, speed: f32, starting_angle: f32) -> Self {
        Self {
            orbital_radius: radius,
            orbital_speed: speed,
            current_angle: starting_angle,
            center: Vec3::ZERO,
        }
    }
}

/// System that updates orbital positions each frame
pub fn update_orbits(
    time: Res<Time>,
    mut query: Query<(&mut OrbitalBody, &mut Transform)>,
) {
    for (mut orbital, mut transform) in query.iter_mut() {
        // Update the angle based on orbital speed
        orbital.current_angle += orbital.orbital_speed * time.delta_secs();
        
        // Calculate new position using circular orbit
        let x = orbital.center.x + orbital.orbital_radius * orbital.current_angle.cos();
        let z = orbital.center.z + orbital.orbital_radius * orbital.current_angle.sin();
        
        // Update transform position (keeping y the same)
        transform.translation.x = x;
        transform.translation.z = z;
    }
}

