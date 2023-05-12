use bevy::prelude::*;
use bevy_kira_audio::prelude::Audio;
use bevy_kira_audio::prelude::*;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(start_background_audio.on_startup());
    }
}

fn start_background_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("background_audio.wav"))
        .looped();
}
