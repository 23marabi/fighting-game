use bevy::prelude::*;
use bevy_proto::prelude::*;

mod game;
use game::GamePlugin;
mod ui;
use ui::UiPlugin;
mod sound;
use sound::SoundPlugin;

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
        .add_plugin(GamePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(SoundPlugin)
        .add_plugin(ProtoPlugin::default())
        .run();
}
