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
            brightness: 1000.0,  // Increased brightness for visibility
            ..default()
        });
        
        info!("Skybox added to camera entity {:?}", camera_entity);
    }
}

/// Creates a simple dark space procedural texture as a placeholder skybox
fn create_simple_space_skybox(images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    
    // Create cubemap - 6 faces (right, left, top, bottom, front, back)
    let size = 512u32;
    let face_size = (size * size * 4) as usize;
    let mut image_data = Vec::with_capacity(face_size * 6);
    
    // Generate 6 faces of the cubemap
    for _face in 0..6 {
        for y in 0..size {
            for _x in 0..size {
                // Very dark blue-black space with some variation
                let noise = rng.r#gen::<f32>() * 0.02;
                let t = (y as f32 / size as f32) * 0.02;
                
                let r = ((t + noise) * 255.0).min(10.0) as u8;
                let g = ((t + noise) * 255.0).min(10.0) as u8;
                let b = ((t * 2.0 + noise + 0.05) * 255.0).min(15.0) as u8;
                
                // Occasionally add a bright star
                if rng.r#gen::<f32>() < 0.0003 {
                    image_data.push(255);
                    image_data.push(255);
                    image_data.push(255);
                } else {
                    image_data.push(r);
                    image_data.push(g);
                    image_data.push(b);
                }
                image_data.push(255);  // Alpha
            }
        }
    }
    
    let image = Image::new(
        Extent3d {
            width: size,
            height: size,
            depth_or_array_layers: 6,  // 6 faces for cubemap
        },
        TextureDimension::D2,  // Bevy handles cubemap as 2D array
        image_data,
        TextureFormat::Rgba8UnormSrgb,
        Default::default(),
    );
    
    images.add(image)
}

