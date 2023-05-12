mod physics;
mod player;

use bevy::prelude::*;
use physics::PhysicsPlugin;
use player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin).add_plugin(PlayerPlugin);
    }
}
