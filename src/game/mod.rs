mod physics;
mod player;
mod sprite;

use bevy::prelude::*;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use sprite::AnimationPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(AnimationPlugin);
    }
}
