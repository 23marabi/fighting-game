mod character;
mod control;
mod physics;
mod player;
mod sprite;

use bevy::prelude::*;
use character::CharacterPlugin;
use control::ControlPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use sprite::AnimationPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin)
            .add_plugin(CharacterPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ControlPlugin)
            .add_plugin(AnimationPlugin);
    }
}
