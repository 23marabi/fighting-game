use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy_framepace::Limiter;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_splash_screen::{SplashAssetType, SplashItem, SplashPlugin, SplashScreen};
use bevy_tweening::EaseFunction;
use std::time::Duration;

mod player;
use player::PlayerPlugin;

mod main_menu;
use main_menu::{cleanup_menu, generate_splashscreen, menu, setup_menu};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Splash,
    MainMenu,
    CharacterSelect,
    InGame,
    Paused,
}

fn main() {
    let primary_window = Window {
        title: "Fighting Game".to_string(),
        resolution: (1920.0, 1080.0).into(),
        resizable: false,
        ..Default::default()
    };

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Tell the asset server to watch for asset changes on disk:
                    watch_for_changes: true,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(primary_window),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_state::<AppState>()
        .add_plugin(
            SplashPlugin::new(AppState::Splash, AppState::MainMenu)
                .skipable()
                .add_screen(generate_splashscreen()),
        )
        .add_plugin(bevy_framepace::FramepacePlugin)
        .add_system(fix_framerate.on_startup())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(WorldInspectorPlugin::default())
        .add_plugin(PlayerPlugin)
        .add_system(setup_camera.on_startup())
        .add_system(setup_menu.in_schedule(OnEnter(AppState::MainMenu)))
        .add_system(menu.in_set(OnUpdate(AppState::MainMenu)))
        .add_system(cleanup_menu.in_schedule(OnExit(AppState::MainMenu)))
        .add_system(bevy::window::close_on_esc.in_set(OnUpdate(AppState::MainMenu)))
        .run();
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
