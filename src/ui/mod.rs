mod camera;
mod debug;
mod menu;
mod splashscreen;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use menu::MenuPlugin;
use splashscreen::SplashscreenPlugin;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MenuPlugin)
            .add_plugin(SplashscreenPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(DebugPlugin);
    }
}
