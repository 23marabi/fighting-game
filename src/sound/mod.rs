mod music;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use music::MusicPlugin;

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin).add_plugin(MusicPlugin);
    }
}
