mod ggrs;

use bevy::prelude::*;
use ggrs::GGRSPlugin;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(GGRSPlugin);
    }
}
