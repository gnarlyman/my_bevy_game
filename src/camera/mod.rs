use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};

/// Component that marks a camera as a free-fly camera (like spectator mode)
#[derive(Component)]
pub struct FreeFlyCam {
    /// Horizontal rotation angle in radians (yaw)
    pub yaw: f32,
    /// Vertical rotation angle in radians (pitch)
    pub pitch: f32,
    /// Mouse sensitivity for looking around
    pub mouse_sensitivity: f32,
    /// Movement speed in units per second
    pub move_speed: f32,
}

impl Default for FreeFlyCam {
    fn default() -> Self {
        Self {
            yaw: std::f32::consts::FRAC_PI_4,      // 45 degrees
            pitch: -std::f32::consts::FRAC_PI_6,   // -30 degrees (looking down)
            mouse_sensitivity: 0.003,
            move_speed: 5.0,
        }
    }
}

/// System that spawns a 3D camera positioned on the home planet
pub fn setup_camera(mut commands: Commands) {
    // Position camera on the surface of home planet (at orbital radius 18, planet radius 2.5)
    // Start slightly above the surface looking toward the star
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(18.0, 4.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        FreeFlyCam::default(),
    ));
}

/// System that locks/unlocks the cursor when the window is clicked or Escape is pressed
pub fn toggle_cursor_lock(
    mut windows: Query<(&Window, &mut CursorOptions)>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    key_input: Res<ButtonInput<KeyCode>>,
) {
    // Lock cursor when window is clicked
    if mouse_buttons.just_pressed(MouseButton::Left) || mouse_buttons.just_pressed(MouseButton::Right) {
        for (window, mut cursor_options) in &mut windows {
            if !window.focused {
                continue;
            }
            cursor_options.grab_mode = CursorGrabMode::Locked;
            cursor_options.visible = false;
        }
    }
    
    // Unlock cursor when Escape is pressed
    if key_input.just_pressed(KeyCode::Escape) {
        for (_, mut cursor_options) in &mut windows {
            cursor_options.grab_mode = CursorGrabMode::None;
            cursor_options.visible = true;
        }
    }
}

/// System that handles mouse look
pub fn camera_look(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut FreeFlyCam, &mut Transform)>,
    windows: Query<(&Window, &CursorOptions)>,
) {
    // Only rotate if cursor is locked on a focused window
    let cursor_grab = windows.iter().any(|(window, cursor_options)| {
        window.focused && cursor_options.grab_mode == CursorGrabMode::Locked
    });
    
    if !cursor_grab || accumulated_mouse_motion.delta == Vec2::ZERO {
        return;
    }

    for (mut cam, mut transform) in query.iter_mut() {
        // Update angles based on mouse movement
        cam.yaw -= accumulated_mouse_motion.delta.x * cam.mouse_sensitivity;
        cam.pitch -= accumulated_mouse_motion.delta.y * cam.mouse_sensitivity;

        // Clamp pitch to prevent camera flipping
        cam.pitch = cam.pitch.clamp(-std::f32::consts::FRAC_PI_2 + 0.01, std::f32::consts::FRAC_PI_2 - 0.01);

        // Calculate rotation from yaw and pitch
        transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, cam.yaw, cam.pitch);
    }
}

/// System that handles WASD movement and Ctrl/Space for up/down
pub fn camera_movement(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&FreeFlyCam, &mut Transform)>,
    windows: Query<(&Window, &CursorOptions)>,
) {
    // Only move if cursor is locked on a focused window
    let cursor_grab = windows.iter().any(|(window, cursor_options)| {
        window.focused && cursor_options.grab_mode == CursorGrabMode::Locked
    });
    
    if !cursor_grab {
        return;
    }

    for (cam, mut transform) in query.iter_mut() {
        let mut movement = Vec3::ZERO;
        let speed = cam.move_speed * time.delta_secs();

        // Get forward and right vectors based on camera rotation
        let forward = *transform.forward();
        let right = *transform.right();

        // WASD movement
        if key_input.pressed(KeyCode::KeyW) {
            movement += forward * speed;
        }
        if key_input.pressed(KeyCode::KeyS) {
            movement -= forward * speed;
        }
        if key_input.pressed(KeyCode::KeyA) {
            movement -= right * speed;
        }
        if key_input.pressed(KeyCode::KeyD) {
            movement += right * speed;
        }

        // Up/Down movement (world space, not relative to camera)
        if key_input.pressed(KeyCode::Space) {
            movement.y += speed;
        }
        if key_input.pressed(KeyCode::ControlLeft) || key_input.pressed(KeyCode::ControlRight) {
            movement.y -= speed;
        }

        transform.translation += movement;
    }
}

