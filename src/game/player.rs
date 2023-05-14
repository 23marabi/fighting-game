use crate::AppState;
use bevy::prelude::*;
use bevy_proto::prelude::*;
use std::fmt;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(greet_players.in_schedule(OnEnter(AppState::InGame)))
            .register_type::<Player>()
            .add_system(load.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(
                add_players
                    .run_if(prototype_ready("Bridget"))
                    .in_schedule(OnEnter(AppState::InGame))
                    .before(greet_players),
            );
    }
}

impl Schematic for Player {
    type Input = PlayerSetup;

    fn apply(input: &Self::Input, context: &mut SchematicContext) {
        context
            .entity_mut()
            .unwrap()
            .insert(Name(input.name.clone()))
            .insert(Health(input.health))
            .insert(PlayerNumber(input.number))
            .insert(PlayerState::Idle);
    }

    fn remove(_input: &Self::Input, context: &mut SchematicContext) {
        context
            .entity_mut()
            .unwrap()
            .remove::<Name>()
            .remove::<Health>()
            .remove::<PlayerNumber>()
            .remove::<PlayerState>();
    }
}

#[derive(Reflect, FromReflect)]
struct PlayerSetup {
    name: String,
    number: u8,
    health: f64,
}

#[derive(Component, Default)]
struct MovementData {
    velocity: Vec2,
    acceleration: f32,
    friction: f32,
    max_speed: f32,
    jump_speed: f32,
}

#[derive(Component, Reflect)]
#[reflect(Schematic)]
struct Player;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Health(f64);

#[derive(Component, FromReflect, Reflect)]
enum AttackState {
    Warmup,
    Hit,
    Recovery,
}

#[derive(Component, FromReflect, Reflect)]
enum BlockState {
    Warmup,
    Counter,
    Block,
}

#[derive(Component, FromReflect, Reflect)]
enum PlayerState {
    Idle,
    Moving,
    Attacking(AttackState),
    Blocking(BlockState),
}

#[derive(Debug, Clone, Copy, FromReflect, Reflect)]
enum Character {
    Bridget,
}

#[derive(Component, PartialEq, Debug, Schematic, Reflect, FromReflect)]
pub struct PlayerNumber(pub u8);

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Character::Bridget => write!(f, "Bridget"),
        }
    }
}

fn load(mut prototypes: PrototypesMut) {
    prototypes.load("characters/Bridget.prototype.ron");
}

fn add_players(mut commands: ProtoCommands) {
    commands.spawn(Character::Bridget.to_string());
    info!("Spawned character!");
    // commands.spawn(PlayerBundle {
    //     name: Name("Erin".to_string()),
    //     hp: Health(10.0),
    //     num: PlayerNumber(1),
    //     _p: Player,
    //     state: PlayerState::Idle,
    //     // sprite: SpriteBundle {
    //     //     texture: asset_server.load("characters/one.png"),
    //     //     transform: Transform::from_xyz(10., 10., 0.),
    //     //     ..default()
    //     // },
    // });
    // commands.spawn(PlayerBundle {
    //     name: Name("tqbed".to_string()),
    //     hp: Health(10.0),
    //     num: PlayerNumber(2),
    //     _p: Player,
    //     state: PlayerState::Idle,
    //     // sprite: SpriteBundle {
    //     //     texture: asset_server.load("characters/one.png"),
    //     //     transform: Transform::from_xyz(100., 0., 0.),
    //     //     ..default()
    //     // },
    // });
}

fn greet_players(query: Query<&Name, With<Player>>) {
    for name in &query {
        println!("Welcome, {}!", name.0);
    }
}

fn check_player_state(query: Query<(&Name, &Health, &PlayerState), With<Player>>) {
    for (name, health, state) in &query {
        match state {
            PlayerState::Idle => println!("{} ({}HP) is Idling", name.0, health.0),
            PlayerState::Moving => println!("{} ({}HP) is Moving", name.0, health.0),
            PlayerState::Attacking(_) => println!("{} ({}HP) is Attacking", name.0, health.0),
            PlayerState::Blocking(_) => println!("{} ({}HP) is Blocking", name.0, health.0),
        }
    }
}
