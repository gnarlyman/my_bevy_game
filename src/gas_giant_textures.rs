use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Creates a procedural banded texture for a gas giant (like Jupiter)
/// with horizontal bands and turbulent variations
pub fn create_amber_titan_texture(images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
    let width = 2048u32;  // Higher res for detail
    let height = 1024u32;
    let mut data = vec![0u8; (width * height * 4) as usize];
    
    let mut rng = StdRng::seed_from_u64(12345);
    
    // More varied band colors
    let bands = vec![
        (0.95, 0.82, 0.55),  // Cream
        (0.82, 0.65, 0.38),  // Light brown
        (0.98, 0.88, 0.62),  // Pale yellow
        (0.75, 0.58, 0.32),  // Medium brown
        (0.88, 0.72, 0.45),  // Tan
        (0.68, 0.52, 0.28),  // Dark tan
        (0.92, 0.78, 0.50),  // Golden
        (0.72, 0.55, 0.30),  // Brown
    ];
    
    for y in 0..height {
        let v = y as f32 / height as f32;
        
        // Complex banding with multiple frequencies
        let band_pattern = v * 16.0 + 
                          (v * 25.0).sin() * 0.3 +  // Large waves
                          (v * 50.0).sin() * 0.15;  // Small ripples
        let band_index = (band_pattern as usize) % bands.len();
        
        for x in 0..width {
            let u = x as f32 / width as f32;
            
            // Multi-scale turbulence for storms and details
            let turb1 = (u * 120.0 + v * 80.0).sin() * (u * 60.0 - v * 90.0).cos() * 0.06;
            let turb2 = (u * 200.0 + v * 150.0).sin() * (u * 180.0).cos() * 0.03;
            let turb3 = (u * 350.0 - v * 280.0).sin() * 0.015;
            
            // Add storm spots occasionally
            let storm_x = (u * 8.0).sin() * 0.5 + 0.5;
            let storm_y = (v * 12.0).cos() * 0.5 + 0.5;
            let dist = ((u - storm_x).powi(2) + (v - storm_y).powi(2)).sqrt();
            let storm = if dist < 0.08 { -0.12 } else { 0.0 };
            
            let (r, g, b) = bands[band_index];
            let total_variation = turb1 + turb2 + turb3 + storm + (rng.r#gen::<f32>() * 0.04 - 0.02);
            
            let idx = ((y * width + x) * 4) as usize;
            data[idx] = ((r + total_variation).clamp(0.0, 1.0) * 255.0) as u8;
            data[idx + 1] = ((g + total_variation).clamp(0.0, 1.0) * 255.0) as u8;
            data[idx + 2] = ((b + total_variation).clamp(0.0, 1.0) * 255.0) as u8;
            data[idx + 3] = 255;
        }
    }
    
    images.add(Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        Default::default(),
    ))
}

/// Creates a procedural texture for an ice giant (like Neptune/Uranus)
/// with subtle bands and atmospheric swirls
pub fn create_azure_colossus_texture(images: &mut ResMut<Assets<Image>>) -> Handle<Image> {
    let width = 2048u32;  // Higher res for detail
    let height = 1024u32;
    let mut data = vec![0u8; (width * height * 4) as usize];
    
    let mut rng = StdRng::seed_from_u64(54321);
    
    // More varied blue tones
    let bands = vec![
        (0.78, 0.88, 0.98),  // Pale blue
        (0.45, 0.68, 0.92),  // Medium blue
        (0.85, 0.92, 1.0),   // Very light blue
        (0.35, 0.58, 0.88),  // Deep blue
        (0.65, 0.80, 0.96),  // Sky blue
        (0.55, 0.72, 0.90),  // Ocean blue
    ];
    
    for y in 0..height {
        let v = y as f32 / height as f32;
        
        // Varied banding with swirls
        let band_pattern = v * 12.0 + 
                          (v * 18.0).sin() * 0.4 +
                          (v * 35.0).cos() * 0.2;
        let band_index = (band_pattern as usize) % bands.len();
        
        for x in 0..width {
            let u = x as f32 / width as f32;
            
            // Atmospheric swirls and details
            let swirl1 = (u * 100.0 + v * 70.0).sin() * (u * 50.0 - v * 60.0).cos() * 0.05;
            let swirl2 = (u * 180.0 - v * 120.0).sin() * (u * 90.0).cos() * 0.025;
            let detail = (u * 300.0 + v * 250.0).sin() * 0.012;
            
            let (r, g, b) = bands[band_index];
            let total_variation = swirl1 + swirl2 + detail + (rng.r#gen::<f32>() * 0.03 - 0.015);
            
            let idx = ((y * width + x) * 4) as usize;
            data[idx] = ((r + total_variation).clamp(0.0, 1.0) * 255.0) as u8;
            data[idx + 1] = ((g + total_variation).clamp(0.0, 1.0) * 255.0) as u8;
            data[idx + 2] = ((b + total_variation).clamp(0.0, 1.0) * 255.0) as u8;
            data[idx + 3] = 255;
        }
    }
    
    images.add(Image::new(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        Default::default(),
    ))
}

