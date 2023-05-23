use crate::AppState;
use ahash::AHashMap;
use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;
use bevy_proto::prelude::*;
use ignore::{types::TypesBuilder, WalkBuilder};
use std::path::PathBuf;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CharacterMap>()
            .add_startup_system(load_characters);
    }
}

#[derive(Resource, Default, Debug)]
pub struct Character {
    name: String,
    path: Option<PathBuf>,
    sprite: Option<String>,
}

impl Character {
    pub fn get_name(&self) -> &String {
        return &self.name;
    }

    pub fn get_path(&self) -> &Option<PathBuf> {
        return &self.path;
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn set_path(&mut self, path: PathBuf) {
        self.path = Some(path);
    }

    fn new(name: &str, path: Option<PathBuf>) -> Self {
        return Self {
            name: name.to_string(),
            path,
            sprite: None,
        };
    }
}

#[derive(Resource, Default)]
pub struct CharacterMap(pub AHashMap<String, Character>, Vec<u8>);

impl CharacterMap {
    fn index(&mut self) {
        let mut builder = TypesBuilder::new();
        builder.add("proto", "*.prototype.ron");
        builder.select("proto");
        let matcher = builder.build().unwrap();

        let walk = WalkBuilder::new("./assets/characters/")
            .types(matcher)
            .build();

        for res in walk {
            match res {
                Ok(entry) => {
                    if entry.depth() == 2 {
                        let name = entry
                            .file_name()
                            .to_str()
                            .unwrap()
                            .split_at(entry.file_name().len() - 14)
                            .0;
                        info!("Found character {}", name);
                        let new_char = Character::new(&name, Some(entry.clone().into_path()));
                        self.0.insert(name.to_string(), new_char);
                    }
                }
                Err(err) => error!("ERROR: {}", err),
            }
        }
    }
}

fn load_characters(mut prototypes: PrototypesMut, mut map: ResMut<CharacterMap>) {
    map.index();

    for (name, char) in &map.0 {
        let path = char
            .path
            .clone()
            .unwrap()
            .clone()
            .strip_prefix("./assets/")
            .unwrap()
            .to_string_lossy()
            .into_owned();

        prototypes.load(path);
    }

    prototypes.load("characters/Player1.prototype.ron");
    prototypes.load("characters/Player2.prototype.ron");
}
