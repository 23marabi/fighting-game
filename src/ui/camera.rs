use crate::AppState;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy_framepace::Limiter;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_framepace::FramepacePlugin)
            .add_system(fix_framerate.on_startup())
            .add_system(setup_camera.on_startup())
            .add_system(draw_background.in_schedule(OnEnter(AppState::InGame)));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        BloomSettings {
            intensity: 0.3,
            ..default()
        },
    ));
}

fn fix_framerate(mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    settings.limiter = Limiter::from_framerate(60.0);
}

fn draw_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_scale(Vec3::new(1.0, 1.0, 0.0)),
        texture: asset_server.load("background.png"),
        ..default()
    });
}
