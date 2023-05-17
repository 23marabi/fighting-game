use crate::AppState;
use bevy::prelude::*;
use bevy_proto::prelude::*;
use bevy_rapier2d::prelude::*;
use std::fmt;

use crate::game::character::CharacterMap;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<PlayerNumber>()
            .register_type::<MovementData>()
            .register_type::<bevy_proto::custom::TransformBundle>()
            .register_type::<Transform>()
            .add_system(add_players.in_schedule(OnEnter(AppState::InGame)));
        // .add_system(
        //     check_player_state
        //         .in_set(OnUpdate(AppState::InGame))
        //         .after(add_players),
        // );
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
            .insert(KinematicCharacterController::default())
            .insert(Collider::capsule(
                Vec2::new(0.0, -input.collider),
                Vec2::new(0.0, input.collider),
                input.collider,
            ))
            // .insert(TransformBundle::from(Transform::from_xyz(
            //     -464.002, -254.0, 0.0,
            // )))
            .insert(input.physics)
            .insert(PlayerState::Idle);
    }

    fn remove(_input: &Self::Input, context: &mut SchematicContext) {
        context
            .entity_mut()
            .unwrap()
            .remove::<Name>()
            .remove::<Health>()
            .remove::<KinematicCharacterController>()
            .remove::<Collider>()
            .remove::<MovementData>()
            .remove::<PlayerNumber>()
            .remove::<PlayerState>();
    }
}

#[derive(Reflect, FromReflect)]
struct PlayerSetup {
    name: String,
    health: f64,
    physics: MovementData,
    collider: f32,
}

#[derive(Component, Reflect, Default, FromReflect, Schematic, Copy, Clone)]
#[reflect(Schematic)]
pub struct MovementData {
    pub velocity: Vec2,
    pub acceleration: f32,
    pub friction: f32,
    pub max_speed: f32,
    pub jump_speed: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Schematic)]
struct Player;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Health(f64);

#[derive(Component, PartialEq, Debug, Schematic, Reflect, FromReflect)]
#[reflect(Schematic)]
pub struct PlayerNumber(pub u8);

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

fn add_players(mut commands: ProtoCommands, characters: Res<CharacterMap>) {
    commands
        .spawn(characters.0.get("Test").unwrap().get_name())
        .insert("Player1");
    commands
        .spawn(characters.0.get("Test").unwrap().get_name())
        .insert("Player2");
}

fn check_player_state(query: Query<(&Name, &Health, &PlayerState)>) {
    for (name, health, state) in &query {
        match state {
            PlayerState::Idle => println!("{} ({}HP) is Idling", name.0, health.0),
            PlayerState::Moving => println!("{} ({}HP) is Moving", name.0, health.0),
            PlayerState::Attacking(_) => println!("{} ({}HP) is Attacking", name.0, health.0),
            PlayerState::Blocking(_) => println!("{} ({}HP) is Blocking", name.0, health.0),
        }
    }
}
