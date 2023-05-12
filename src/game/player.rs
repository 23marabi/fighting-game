use crate::AppState;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(greet_players.in_schedule(OnEnter(AppState::InGame)))
            .add_system(
                add_players
                    .in_schedule(OnEnter(AppState::InGame))
                    .before(greet_players),
            );
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    hp: Health,
    _p: Player,
    state: PlayerState,
    num: PlayerNumber,
    // #[bundle]
    // sprite: SpriteBundle,
}

#[derive(Component, Default)]
struct MovementData {
    velocity: Vec2,
    acceleration: f32,
    friction: f32,
    max_speed: f32,
    jump_speed: f32,
}

#[derive(Component)]
struct Player;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Health(f64);

#[derive(Component)]
enum AttackState {
    Warmup,
    Hit,
    Recovery,
}

#[derive(Component)]
enum BlockState {
    Warmup,
    Counter,
    Block,
}

#[derive(Component)]
enum PlayerState {
    Idle,
    Moving,
    Attacking(AttackState),
    Blocking(BlockState),
}

#[derive(Component, PartialEq, Debug)]
pub struct PlayerNumber(pub u8);

fn add_players(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        (PlayerBundle {
            name: Name("Erin".to_string()),
            hp: Health(10.0),
            num: PlayerNumber(1),
            _p: Player,
            state: PlayerState::Idle,
            // sprite: SpriteBundle {
            //     texture: asset_server.load("characters/one.png"),
            //     transform: Transform::from_xyz(10., 10., 0.),
            //     ..default()
            // },
        }),
    );
    commands.spawn(
        (PlayerBundle {
            name: Name("tqbed".to_string()),
            hp: Health(10.0),
            num: PlayerNumber(2),
            _p: Player,
            state: PlayerState::Idle,
            // sprite: SpriteBundle {
            //     texture: asset_server.load("characters/one.png"),
            //     transform: Transform::from_xyz(100., 0., 0.),
            //     ..default()
            // },
        }),
    );
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
