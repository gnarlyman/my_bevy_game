use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

/// Component that marks a camera as an orbit camera and stores its state
#[derive(Component)]
pub struct OrbitCamera {
    /// The point the camera orbits around
    pub target: Vec3,
    /// Distance from the target
    pub distance: f32,
    /// Horizontal rotation angle in radians
    pub yaw: f32,
    /// Vertical rotation angle in radians
    pub pitch: f32,
    /// Mouse sensitivity
    pub sensitivity: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            distance: 10.0,
            yaw: std::f32::consts::FRAC_PI_4,      // 45 degrees
            pitch: std::f32::consts::FRAC_PI_6,    // 30 degrees
            sensitivity: 0.003,
        }
    }
}

/// System that spawns a 3D camera positioned to view the scene
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(8.0, 6.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        OrbitCamera::default(),
    ));
}

/// System that handles mouse input and orbits the camera around the target
pub fn orbit_camera(
    mut mouse_motion: MessageReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    // Only rotate when right mouse button is held (like many 3D applications)
    // Or if you prefer always rotating, remove this check
    let should_rotate = mouse_buttons.pressed(MouseButton::Right) 
        || mouse_buttons.pressed(MouseButton::Left);
    
    if !should_rotate {
        return;
    }

    let mut total_delta = Vec2::ZERO;
    for event in mouse_motion.read() {
        total_delta += event.delta;
    }

    if total_delta == Vec2::ZERO {
        return;
    }

    for (mut orbit, mut transform) in query.iter_mut() {
        // Update angles based on mouse movement
        orbit.yaw -= total_delta.x * orbit.sensitivity;
        orbit.pitch -= total_delta.y * orbit.sensitivity;

        // Clamp pitch to prevent camera flipping
        orbit.pitch = orbit.pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.1, std::f32::consts::FRAC_PI_2 - 0.1);

        // Calculate new camera position using spherical coordinates
        let x = orbit.distance * orbit.pitch.cos() * orbit.yaw.cos();
        let y = orbit.distance * orbit.pitch.sin();
        let z = orbit.distance * orbit.pitch.cos() * orbit.yaw.sin();

        transform.translation = orbit.target + Vec3::new(x, y, z);
        transform.look_at(orbit.target, Vec3::Y);
    }
}

