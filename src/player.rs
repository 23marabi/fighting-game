use crate::AppState;
use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadConnectionEvent, GamepadEvent,
    },
    prelude::*,
};
use std::time::Duration;

const SPEED: f32 = 200.0;
const JUMP_SPEED: f32 = 1000.0;
const GRAVITY: f32 = 500.0;

pub struct PlayerPlugin;

#[derive(Component)]
struct JumpTimer(Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(greet_players.in_schedule(OnEnter(AppState::InGame)))
            .add_system(gamepad_ordered_events)
            .add_system(check_player_state.in_set(OnUpdate(AppState::InGame)))
            .add_system(keyboard_input.in_set(OnUpdate(AppState::InGame)))
            .add_system(player_animation.in_set(OnUpdate(AppState::InGame)))
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
    _d: Dead,
    _j: Jumping,

    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Component)]
struct Dead(bool);

#[derive(Component)]
struct Jumping(bool);

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

#[derive(Component, Debug, PartialEq, Copy, Clone)]
enum Direction {
    Negative = -1,
    Center = 0,
    Positive = 1,
}

#[derive(Component, Debug, PartialEq, Copy, Clone)]
struct Movement(Direction, Direction);

#[derive(Component)]
enum BlockState {
    Warmup,
    Counter,
    Block,
}

#[derive(Component)]
enum PlayerState {
    Idle,
    Moving(Movement),
    Attacking(AttackState),
    Blocking(BlockState),
}

#[derive(Component, PartialEq)]
struct PlayerNumber(u8);

fn add_players(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        (PlayerBundle {
            name: Name("Erin".to_string()),
            hp: Health(10.0),
            num: PlayerNumber(1),
            _p: Player,
            state: PlayerState::Idle,
            _d: Dead(false),
            _j: Jumping(false),
            sprite: SpriteBundle {
                texture: asset_server.load("characters/one.png"),
                transform: Transform::from_xyz(10., 10., 0.),
                ..default()
            },
        }),
    );
    commands.spawn(
        (PlayerBundle {
            name: Name("tqbed".to_string()),
            hp: Health(10.0),
            num: PlayerNumber(2),
            _p: Player,
            state: PlayerState::Idle,
            _d: Dead(false),
            _j: Jumping(false),
            sprite: SpriteBundle {
                texture: asset_server.load("characters/one.png"),
                transform: Transform::from_xyz(100., 0., 0.),
                ..default()
            },
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
            PlayerState::Moving(d) => println!("{} ({}HP) is Moving {:?}", name.0, health.0, d),
            PlayerState::Attacking(_) => println!("{} ({}HP) is Attacking", name.0, health.0),
            PlayerState::Blocking(_) => println!("{} ({}HP) is Blocking", name.0, health.0),
            _ => {}
        }
    }
}

fn keyboard_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerNumber, &mut PlayerState), With<Player>>,
    mut timer_query: Query<(Entity, &PlayerNumber, &mut JumpTimer)>,
    time: Res<Time>,
) {
    for (num, mut state) in query.iter_mut() {
        match num {
            PlayerNumber(1) => {
                let mut in_move = Movement(Direction::Center, Direction::Center);
                let no_move = Movement(Direction::Center, Direction::Center);

                if keys.just_pressed(KeyCode::W) {
                    in_move.1 = Direction::Positive;
                    commands.spawn((
                        JumpTimer(Timer::new(Duration::from_millis(500), TimerMode::Once)),
                        PlayerNumber(1),
                    ));
                }

                for (entity, player, mut timer) in timer_query.iter_mut() {
                    if player == &PlayerNumber(1) {
                        timer.0.tick(time.delta());
                        if timer.0.finished() && in_move.1 == Direction::Positive {
                            in_move.1 = Direction::Center;
                            commands.entity(entity).despawn();
                        } else if timer.0.finished() && keys.just_pressed(KeyCode::W) {
                            in_move.1 = Direction::Positive;
                            commands.entity(entity).despawn();
                            commands.spawn((
                                JumpTimer(Timer::new(Duration::from_millis(500), TimerMode::Once)),
                                PlayerNumber(1),
                            ));
                        } else if !timer.0.finished() {
                            in_move.1 = Direction::Positive;
                        }
                    }
                }
                if keys.pressed(KeyCode::S) {
                    in_move.1 = Direction::Negative
                };
                if keys.pressed(KeyCode::A) {
                    in_move.0 = Direction::Negative
                };
                if keys.pressed(KeyCode::D) {
                    in_move.0 = Direction::Positive
                };

                match *state {
                    PlayerState::Moving(no_move) => {
                        *state = PlayerState::Idle;
                    }
                    PlayerState::Moving(_) => {
                        println!("moving x: {:?}, y: {:?}", &in_move.0, &in_move.1);
                        if in_move != no_move {
                            *state = PlayerState::Moving(in_move);
                        }
                    }
                    _ => {
                        if in_move != no_move {
                            *state = PlayerState::Moving(in_move);
                        }
                    }
                }
            }
            _ => {
                let mut in_move = Movement(Direction::Center, Direction::Center);
                let no_move = Movement(Direction::Center, Direction::Center);
                if keys.pressed(KeyCode::Up) {
                    in_move.1 = Direction::Positive
                };
                if keys.pressed(KeyCode::Down) {
                    in_move.1 = Direction::Negative
                };
                if keys.pressed(KeyCode::Left) {
                    in_move.0 = Direction::Negative
                };
                if keys.pressed(KeyCode::Right) {
                    in_move.0 = Direction::Positive
                };

                match *state {
                    PlayerState::Moving(no_move) => {
                        *state = PlayerState::Idle;
                    }
                    PlayerState::Moving(_) => {
                        println!("moving x: {:?}, y: {:?}", &in_move.0, &in_move.1);
                        if in_move != no_move {
                            *state = PlayerState::Moving(in_move);
                        }
                    }
                    _ => {
                        if in_move != no_move {
                            *state = PlayerState::Moving(in_move);
                        }
                    }
                }
            }
        }
    }
}

fn gamepad_ordered_events(mut gamepad_events: EventReader<GamepadEvent>) {
    for gamepad_event in gamepad_events.iter() {
        match gamepad_event {
            GamepadEvent::Connection(connection_event) => info!("{:?}", connection_event),
            GamepadEvent::Button(button_event) => info!("{:?}", button_event),
            GamepadEvent::Axis(axis_event) => info!("{:?}", axis_event),
        }
    }
}

fn player_animation(
    time: Res<Time>,
    mut sprites: Query<(&PlayerState, &mut Transform), With<Player>>,
) {
    for (state, mut transform) in &mut sprites {
        match *state {
            PlayerState::Moving(p_move) => {
                println!("x: {:?}, y{:?}", &p_move.0, &p_move.1);
                match &p_move.0 {
                    &Direction::Positive => transform.translation.x += SPEED * time.delta_seconds(),
                    &Direction::Negative => transform.translation.x -= SPEED * time.delta_seconds(),
                    _ => {}
                }
                match &p_move.1 {
                    &Direction::Positive => {
                        transform.translation.y += JUMP_SPEED * time.delta_seconds()
                    }
                    &Direction::Negative => transform.translation.y -= SPEED * time.delta_seconds(),
                    &Direction::Center => transform.translation.y -= GRAVITY * time.delta_seconds(),
                    _ => {}
                }
            }
            _ => transform.translation.y -= GRAVITY * time.delta_seconds(),
        }
    }
}
