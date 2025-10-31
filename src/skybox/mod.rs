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
            brightness: 3000.0,  // MUCH brighter for visibility
            ..default()
        });
        
        warn!("=== SKYBOX ADDED TO CAMERA {:?} ===", camera_entity);
    }
}

/// Creates a simple dark space procedural texture as a placeholder skybox
fn create_simple_space_skybox(images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureViewDimension};
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    
    // Create cubemap - 6 faces (right, left, top, bottom, front, back)
    let size = 512u32;
    let bytes_per_pixel = 4; // RGBA8 = 4 bytes per pixel
    let face_size = (size * size) as usize * bytes_per_pixel;
    let mut image_data = Vec::with_capacity(face_size * 6);
    
    // Generate 6 faces of the cubemap with stars
    for _face in 0..6 {
        for y in 0..size {
            for _x in 0..size {
                // Very dark blue-black space with slight variation
                let noise = rng.r#gen::<f32>() * 0.01;
                let t = (y as f32 / size as f32) * 0.01;
                
                let base_r = ((t + noise) * 255.0).min(5.0) as u8;
                let base_g = ((t + noise) * 255.0).min(5.0) as u8;
                let base_b = ((t * 2.0 + noise + 0.02) * 255.0).min(10.0) as u8;
                
                // Add bright stars randomly scattered - MUCH more visible
                let star_chance = rng.r#gen::<f32>();
                if star_chance < 0.003 {
                    // Bright white star (6x more stars)
                    image_data.push(255);
                    image_data.push(255);
                    image_data.push(255);
                } else if star_chance < 0.005 {
                    // Dimmer star
                    image_data.push(180);
                    image_data.push(180);
                    image_data.push(200);
                } else {
                    // Dark space
                    image_data.push(base_r);
                    image_data.push(base_g);
                    image_data.push(base_b);
                }
                image_data.push(255);  // Alpha
            }
        }
    }
    
    let mut image = Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 6,  // 6 faces for cubemap
        },
        TextureDimension::D2,
        image_data,
        TextureFormat::Rgba8UnormSrgb,
        Default::default(),
    );
    
    // Critical: Set texture view dimension to Cube for skybox
    image.texture_view_descriptor = Some(bevy::render::render_resource::TextureViewDescriptor {
        label: Some("skybox_cubemap"),
        dimension: Some(TextureViewDimension::Cube),
        ..Default::default()
    });
    
    images.add(image)
}

