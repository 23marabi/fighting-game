use bevy::prelude::*;

mod player;
use player::PlayerPlugin;

#[derive(Resource, Debug)]
pub enum MenuState {
    SplashScreen,
    MainMenu,
    Versus,
    Online,
    CharacterSelect,
    Playing,
}

pub fn start(menu_state: Res<MenuState>) {
    println!("Starting on {:?}", *menu_state);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .insert_resource(MenuState::SplashScreen)
        .add_startup_system(start)
        .add_system(bevy::window::close_on_esc)
        .run();
}
