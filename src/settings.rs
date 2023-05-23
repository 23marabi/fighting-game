use crate::game::player::PlayerInput;
use bevy::input::keyboard::KeyCode;
use bevy::prelude::Resource;
use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Resource)]
#[allow(unused)]
pub struct Window {
    pub resolution: (f32, f32),
    pub resizable: bool,
}

#[derive(Debug, Deserialize, Resource)]
#[allow(unused)]
pub struct Background {
    pub image: String,
    pub scale: (f32, f32, f32),
}

#[derive(Debug, Deserialize, Resource)]
#[allow(unused)]
pub struct Physics {
    pub ground: PhysicsObject,
    pub left_wall: PhysicsObject,
    pub right_wall: PhysicsObject,
    pub ceiling: PhysicsObject,
}

#[derive(Debug, Deserialize, Resource)]
#[allow(unused)]
pub struct PhysicsObject {
    pub position: (f32, f32),
    pub dimensions: (f32, f32),
}

#[derive(Debug, Deserialize, Resource)]
#[allow(unused)]
pub struct Settings {
    pub window: Window,
    pub framerate: f64,
    pub background: Background,
    pub physics: Physics,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?;
        s.try_deserialize()
    }
}
