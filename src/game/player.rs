use crate::AppState;
use bevy::prelude::*;
use bevy_proto::prelude::*;
use std::fmt;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<PlayerNumber>()
            .add_system(
                add_players
                    .run_if(
                        prototype_ready("Test")
                            .and_then(prototype_ready("Player1"))
                            .and_then(prototype_ready("Player2")),
                    )
                    .in_schedule(OnEnter(AppState::InGame)),
            );
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

fn add_players(mut commands: ProtoCommands) {
    commands.spawn("Test").insert("Player2");
    commands.spawn("Test").insert("Player1");
    info!("Spawned characters!");
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
