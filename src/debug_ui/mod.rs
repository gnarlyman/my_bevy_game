use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

/// Marker component for the debug stats text
#[derive(Component)]
pub struct DebugStatsText;

/// System that spawns the debug UI overlay
pub fn setup_debug_ui(mut commands: Commands) {
    // Create a container node for the debug stats in the top-left corner
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Debug Stats Loading..."),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
                DebugStatsText,
            ));
        });
}

/// System that updates the debug stats display each frame
pub fn update_debug_stats(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<DebugStatsText>>,
) {
    for mut text in query.iter_mut() {
        let mut stats_text = String::new();
        
        // FPS (Frames Per Second)
        if let Some(fps_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
                stats_text.push_str(&format!("FPS: {:.1}\n", fps_smoothed));
            }
        }
        
        // Frame Time (ms)
        if let Some(frame_time_diagnostic) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME) {
            if let Some(frame_time_smoothed) = frame_time_diagnostic.smoothed() {
                stats_text.push_str(&format!("Frame Time: {:.2}ms\n", frame_time_smoothed));
            }
        }
        
        // Entity Count
        if let Some(entity_count_diagnostic) = diagnostics.get(&bevy::diagnostic::EntityCountDiagnosticsPlugin::ENTITY_COUNT) {
            if let Some(entity_count) = entity_count_diagnostic.value() {
                stats_text.push_str(&format!("Entities: {:.0}\n", entity_count));
            }
        }
        
        // Memory usage (approximate - Bevy doesn't have built-in memory diagnostics)
        // We can estimate based on system info if needed, for now show entity count as proxy
        
        **text = stats_text;
    }
}

