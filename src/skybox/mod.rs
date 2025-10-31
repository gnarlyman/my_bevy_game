use bevy::{
    prelude::*,
    core_pipeline::Skybox,
};

/// Component to mark cameras that should have a skybox
#[derive(Component)]
pub struct SkyboxCamera;

/// System that adds a skybox to the camera
/// Note: For now uses a simple dark color. To use a texture:
/// 1. Download a space skybox (e.g., from polyhaven.com)
/// 2. Place in assets/textures/skybox.png
/// 3. Uncomment the asset loading code below
pub fn setup_skybox(
    mut commands: Commands,
    camera_query: Query<Entity, (With<Camera3d>, Without<Skybox>)>,
    _asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    for camera_entity in camera_query.iter() {
        // Option 1: Use a texture file (uncomment if you have a skybox texture)
        // let skybox_handle = asset_server.load("textures/skybox.png");
        
        // Option 2: Create a simple procedural dark space texture
        let skybox_handle = create_simple_space_skybox(&mut images);
        
        commands.entity(camera_entity).insert(Skybox {
            image: skybox_handle,
            brightness: 500.0,  // Toned down for subtlety
            ..default()
        });
        
        warn!("=== SKYBOX ADDED TO CAMERA {:?} ===", camera_entity);
    }
}

/// Creates a simple dark space procedural texture with seamless starfield
fn create_simple_space_skybox(images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureViewDimension};
    use rand::SeedableRng;
    use rand::rngs::StdRng;
    
    // ========== CONFIGURATION ==========
    // Adjust these values to control star appearance:
    
    // Resolution per face (512, 1024, 2048, 4096)
    // Higher = crisper stars but slower startup
    let size = 2048u32;
    
    let bytes_per_pixel = 4usize;
    
    // Create stacked 2D image: 6 faces vertically (posx, negx, posy, negy, posz, negz)
    // This matches Bevy's expected cubemap format
    let total_height = size * 6;
    let mut image_data = vec![0u8; (size * total_height) as usize * bytes_per_pixel];
    
    let _rng = StdRng::seed_from_u64(42);
    
    info!("Generating high-res starfield skybox: {}x{} per face...", size, size);
    
    // Cubemap face order: +X, -X, +Y, -Y, +Z, -Z
    // Each face is a view direction with up and right vectors
    let faces = [
        ("PosX", Vec3::X, Vec3::NEG_Y, Vec3::NEG_Z),
        ("NegX", Vec3::NEG_X, Vec3::NEG_Y, Vec3::Z),
        ("PosY", Vec3::Y, Vec3::Z, Vec3::X),
        ("NegY", Vec3::NEG_Y, Vec3::NEG_Z, Vec3::X),
        ("PosZ", Vec3::Z, Vec3::NEG_Y, Vec3::X),
        ("NegZ", Vec3::NEG_Z, Vec3::NEG_Y, Vec3::NEG_X),
    ];
    
    for (face_idx, (_name, forward, up, right)) in faces.iter().enumerate() {
        let face_offset = face_idx * size as usize * size as usize * bytes_per_pixel;
        
        for y in 0..size {
            for x in 0..size {
                // Map to [-1, 1] range
                let u = (x as f32 / (size - 1) as f32) * 2.0 - 1.0;
                let v = 1.0 - (y as f32 / (size - 1) as f32) * 2.0;
                
                // Get 3D direction for this pixel
                let direction = (*forward + *right * u + *up * v).normalize();
                
                // Use multiple scales for star variety and seamlessness
                let scale1 = 15.0;
                let scale2 = 30.0;
                let scale3 = 60.0;
                
                let p1 = direction * scale1;
                let p2 = direction * scale2;
                let p3 = direction * scale3;
                
                let hash1 = ((p1.x * 12.9898 + p1.y * 78.233 + p1.z * 45.164).sin() * 43758.5453).fract();
                let hash2 = ((p2.x * 17.1234 + p2.y * 91.567 + p2.z * 23.891).sin() * 27182.8182).fract();
                let hash3 = ((p3.x * 31.4159 + p3.y * 62.831 + p3.z * 14.142).sin() * 31415.9265).fract();
                
                let pixel_idx = face_offset + (y * size + x) as usize * bytes_per_pixel;
                
                // Star density thresholds (LOWER = MORE STARS)
                // Fine-grained control: 0.990-0.999
                // Examples: 0.998 (sparse), 0.9985 (medium), 0.999 (very sparse), 0.9995 (ultra sparse)
                const BRIGHT_THRESHOLD: f32 = 0.9995;  // Bright stars
                const MEDIUM_THRESHOLD: f32 = 0.9992;  // Medium stars  
                const DIM_THRESHOLD: f32 = 0.9990;     // Dim stars
                
                let mut is_star = false;
                let mut brightness = 0u8;
                
                // Bright stars (sparse)
                if hash1 > BRIGHT_THRESHOLD {
                    is_star = true;
                    brightness = ((hash1 - BRIGHT_THRESHOLD) / (1.0 - BRIGHT_THRESHOLD) * 150.0 + 100.0) as u8;
                }
                // Medium stars (more common)
                else if hash2 > MEDIUM_THRESHOLD {
                    is_star = true;
                    brightness = ((hash2 - MEDIUM_THRESHOLD) / (1.0 - MEDIUM_THRESHOLD) * 100.0 + 60.0) as u8;
                }
                // Dim stars (even more common)
                else if hash3 > DIM_THRESHOLD {
                    is_star = true;
                    brightness = ((hash3 - DIM_THRESHOLD) / (1.0 - DIM_THRESHOLD) * 80.0 + 40.0) as u8;
                }
                
                if is_star {
                    image_data[pixel_idx] = brightness;
                    image_data[pixel_idx + 1] = brightness;
                    image_data[pixel_idx + 2] = brightness.saturating_add(15);
                    image_data[pixel_idx + 3] = 255;
                } else {
                    // Very dark space background
                    image_data[pixel_idx] = 1;
                    image_data[pixel_idx + 1] = 1;
                    image_data[pixel_idx + 2] = 3;
                    image_data[pixel_idx + 3] = 255;
                }
            }
        }
    }
    
    info!("Starfield generation complete!");
    
    // Create image as stacked 2D (width x height*6)
    let mut image = Image::new(
        Extent3d {
            width: size,
            height: total_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        image_data,
        TextureFormat::Rgba8UnormSrgb,
        Default::default(),
    );
    
    // Reinterpret as cubemap array (this is the key!)
    image.reinterpret_stacked_2d_as_array(6);
    image.texture_view_descriptor = Some(bevy::render::render_resource::TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..Default::default()
    });
    
    images.add(image)
}

