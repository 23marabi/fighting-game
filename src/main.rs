#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;
use bevy_proto::prelude::*;
use clap::Parser;
use config::Config;
use envmnt::{ExpandOptions, ExpansionType};
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

    if !envmnt::exists("LANG") {
        envmnt::set("LANG", "en_US.UTF-8");
    }
    let title = match settings.translation.get(&envmnt::get_or_panic("LANG")) {
        Some(v) => v.title.clone(),
        None => {
            eprintln!(
                "No field matching {} in settings",
                envmnt::get_or_panic("LANG")
            );
            exit(1);
        }
    };

    let primary_window = Window {
        title: title,
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
