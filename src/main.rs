use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;

mod player;
use player::PlayerPlugin;

mod main_menu;
use main_menu::{cleanup_menu, menu, setup_game, setup_menu};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    CharacterSelect,
    InGame,
    Paused,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(PlayerPlugin)
        .add_system(setup.on_startup())
        .add_system(setup_menu.in_schedule(OnEnter(AppState::MainMenu)))
        .add_system(menu.in_set(OnUpdate(AppState::MainMenu)))
        .add_system(cleanup_menu.in_schedule(OnExit(AppState::MainMenu)))
        .add_system(setup_game.in_schedule(OnEnter(AppState::InGame)))
        .add_system(bevy::window::close_on_esc.in_set(OnUpdate(AppState::MainMenu)))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        BloomSettings {
            intensity: 0.3,
            ..default()
        },
    ));
}
