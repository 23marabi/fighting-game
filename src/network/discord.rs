use bevy::prelude::*;
use bevy_discord_presence::{ActivityState, RPCConfig, RPCPlugin};

pub struct DiscordPlugin;

impl Plugin for DiscordPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RPCPlugin {
        config: RPCConfig {
            app_id: 1107468459903832106,
            show_time: true,
        },
        }).add_system(update_presence);
    }
}

fn update_presence(mut state: ResMut<ActivityState>) {
    state.instance = Some(true);
    state.details = Some("Hello World".to_string());
    state.state = Some("Local".to_string());
}
