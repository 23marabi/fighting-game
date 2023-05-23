use bevy::prelude::*;
use bevy_debug_text_overlay::OverlayPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::default())
            .add_plugin(OverlayPlugin {
                font_size: 32.0,
                ..default()
            });
    }
}
