use crate::AppState;
use bevy::prelude::*;
use bevy_proto::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_characters.in_schedule(OnEnter(AppState::MainMenu)));
    }
}

fn load_characters(mut prototypes: PrototypesMut) {
    prototypes.load("characters/Player1.prototype.ron");
    prototypes.load("characters/Player2.prototype.ron");
    prototypes.load("characters/Test/Test.prototype.ron");
}
