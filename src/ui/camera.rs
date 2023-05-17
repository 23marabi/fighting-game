use crate::settings::Settings;
use crate::AppState;
use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy_framepace::Limiter;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_framepace::FramepacePlugin)
            .add_system(fix_framerate.on_startup())
            .add_system(setup_camera.on_startup());
        // .add_system(draw_background.in_schedule(OnEnter(AppState::InGame)));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn fix_framerate(
    mut frame_settings: ResMut<bevy_framepace::FramepaceSettings>,
    settings: Res<Settings>,
) {
    frame_settings.limiter = Limiter::from_framerate(settings.framerate);
}

fn draw_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
) {
    commands.spawn(SpriteBundle {
        transform: Transform::from_scale(Vec3::new(
            settings.background.scale.0,
            settings.background.scale.1,
            settings.background.scale.2,
        )),
        texture: asset_server.load(settings.background.image.clone()),
        ..default()
    });
}
