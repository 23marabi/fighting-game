use crate::AppState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::Audio;
use bevy_kira_audio::prelude::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_menu_audio.in_schedule(OnEnter(AppState::MainMenu)))
            .add_audio_channel::<Background>()
            .add_system(start_combat_audio.in_schedule(OnEnter(AppState::InGame)));
    }
}

fn start_menu_audio(asset_server: Res<AssetServer>, background: Res<AudioChannel<Background>>) {
    background.stop();
    background
        .play(asset_server.load("audio/music/menu.wav"))
        .fade_in(AudioTween::default())
        .looped();
}

fn start_combat_audio(asset_server: Res<AssetServer>, background: Res<AudioChannel<Background>>) {
    background.stop();
    background
        .play(asset_server.load("audio/music/combat.wav"))
        .fade_in(AudioTween::default())
        .looped();
}

#[derive(Resource)]
struct Background;
