mod discord;

use bevy::prelude::*;
use discord::DiscordPlugin;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NetworkPlugin);
    }
}
