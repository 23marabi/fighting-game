use bevy::prelude::*;
use bevy_proto::prelude::*;
use clap::Parser;
use config::Config;
use human_panic::setup_panic;
use std::collections::HashMap;
use std::process::exit;

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

/// A fun and over-the-top fighting game.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // IP Address to listen on
    #[arg(long, default_value = "localhost")]
    ip: String,

    #[arg(long, default_value_t = 1337)]
    port: u16,
}

mod settings;
use settings::Settings;

fn main() {
    setup_panic!();
    let args = Args::parse();

    let settings = Settings::new().unwrap();
    println!("{:?}", settings);

    let primary_window = Window {
        title: "Fighting Game".to_string(),
        resolution: settings.window.resolution.into(),
        resizable: settings.window.resizable,
        ..Default::default()
    };

    let app = App::new()
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
        .insert_resource(settings)
        .add_state::<AppState>()
        .add_plugin(GamePlugin)
        .add_plugin(UiPlugin)
        .add_plugin(SoundPlugin)
        .add_plugin(ProtoPlugin::default())
        .run();
}
