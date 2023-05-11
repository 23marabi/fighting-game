use crate::AppState;
use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadConnectionEvent, GamepadEvent,
    },
    prelude::*,
};
use bevy_rapier2d::prelude::*;

const SPEED: f32 = 8.0;
const JUMP_SPEED: f32 = 15.0;
const FRICTION: f32 = 0.2;
const GRAVITY: f32 = 9.8;

#[derive(Component)]
struct JumpTimer(Timer);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(greet_players.in_schedule(OnEnter(AppState::InGame)))
            .add_system(gamepad_ordered_events)
            // .add_system(init_physics.in_schedule(OnEnter(AppState::InGame)))
            // .add_system(check_player_state.in_set(OnUpdate(AppState::InGame)))
            // .add_system(keyboard_input.in_set(OnUpdate(AppState::InGame)))
            .add_system(setup_physics.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_physics.in_set(OnUpdate(AppState::InGame)))
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
    direction: Direction,
    velocity: Velocity,
    // #[bundle]
    // sprite: SpriteBundle,
}

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

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
struct PlayerNumber(u8);

fn add_players(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(
        (PlayerBundle {
            name: Name("Erin".to_string()),
            hp: Health(10.0),
            num: PlayerNumber(1),
            _p: Player,
            state: PlayerState::Idle,
            direction: Direction(Vec2::ZERO),
            velocity: Velocity(Vec2::ZERO),
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
            direction: Direction(Vec2::ZERO),
            velocity: Velocity(Vec2::ZERO),
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

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerNumber, &mut Velocity, &mut Direction), With<Player>>,
) {
    for (num, mut velocity, mut direction) in query.iter_mut() {
        let mut x = 0;
        let mut y = 0;
        match num {
            PlayerNumber(1) => {
                x = keys.pressed(KeyCode::D) as i32 - keys.pressed(KeyCode::A) as i32;
                y = keys.pressed(KeyCode::S) as i32 - keys.pressed(KeyCode::W) as i32;
            }
            PlayerNumber(2) => {
                x = keys.pressed(KeyCode::Left) as i32 - keys.pressed(KeyCode::Right) as i32;
                y = keys.pressed(KeyCode::Down) as i32 - keys.pressed(KeyCode::Up) as i32;
            }
            _ => {}
        }

        *direction = Direction(Vec2 {
            x: x as f32,
            y: y as f32,
        });

        if direction.0.length() > 1.0 {
            direction.0.normalize();
        }

        let target_velocity = direction.0 * SPEED;
        let old_velocity = velocity.0.clone();
        velocity.0 += (target_velocity - old_velocity);
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

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(575.0, 20.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -325.0, 0.0)));
    /* Create the walls */
    commands
        .spawn(Collider::cuboid(20.0, 325.0))
        .insert(TransformBundle::from(Transform::from_xyz(-575.0, 0.0, 0.0)));
    commands
        .spawn(Collider::cuboid(20.0, 325.0))
        .insert(TransformBundle::from(Transform::from_xyz(575.0, 0.0, 0.0)));

    commands
        .spawn(KinematicCharacterController {
            custom_mass: Some(10.0),
            filter_flags: QueryFilterFlags::EXCLUDE_KINEMATIC,
            ..default()
        })
        .insert(Collider::ball(50.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            -464.002, -254.0, 0.0,
        )))
        .insert(PlayerNumber(1));
    commands.spawn((
        PlayerNumber(1),
        JumpTimer(Timer::from_seconds(0.15, TimerMode::Once)),
    ));
    commands
        .spawn(KinematicCharacterController {
            custom_mass: Some(10.0),
            filter_flags: QueryFilterFlags::EXCLUDE_KINEMATIC,
            ..default()
        })
        .insert(Collider::ball(50.0))
        .insert(TransformBundle::from(Transform::from_xyz(
            464.002, -254.0, 0.0,
        )))
        .insert(PlayerNumber(2));
    commands.spawn((
        PlayerNumber(2),
        JumpTimer(Timer::from_seconds(0.15, TimerMode::Once)),
    ));
}

fn update_physics(
    mut commands: Commands,
    time: Res<Time>,
    mut timer_query: Query<(&PlayerNumber, &mut JumpTimer)>,
    mut controllers: Query<(&PlayerNumber, &mut KinematicCharacterController)>,
    outputs: Query<(&PlayerNumber, &KinematicCharacterControllerOutput)>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut p1_to_move = Vec2::ZERO;
    let mut p2_to_move = Vec2::ZERO;

    for (t_num, mut timer) in &mut timer_query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            if t_num == &PlayerNumber(1) {
                p1_to_move.y -= GRAVITY;
            } else if t_num == &PlayerNumber(2) {
                p2_to_move.y -= GRAVITY;
            }
        } else {
            if t_num == &PlayerNumber(1) {
                p1_to_move.y += JUMP_SPEED;
            } else if t_num == &PlayerNumber(2) {
                p2_to_move.y += JUMP_SPEED;
            }
        }
    }

    if keyboard.pressed(KeyCode::A) {
        p1_to_move.x -= SPEED;
    }
    if keyboard.pressed(KeyCode::D) {
        p1_to_move.x += SPEED;
    }

    if keyboard.pressed(KeyCode::Left) {
        p2_to_move.x -= SPEED;
    }
    if keyboard.pressed(KeyCode::Right) {
        p2_to_move.x += SPEED;
    }

    for (num, mut controller) in controllers.iter_mut() {
        if num == &PlayerNumber(1) {
            let mut on_ground = false;
            for (o_num, output) in outputs.iter() {
                if o_num == &PlayerNumber(1) {
                    on_ground = output.clone().grounded;
                }
            }

            if keyboard.just_pressed(KeyCode::W) && on_ground {
                for (t_num, mut timer) in &mut timer_query {
                    if t_num == &PlayerNumber(1) {
                        if timer.0.finished() {
                            timer.0.reset();
                        }
                    }
                }
            }

            controller.translation = Some(p1_to_move);
        } else if num == &PlayerNumber(2) {
            let mut on_ground = false;
            for (o_num, output) in outputs.iter() {
                if o_num == &PlayerNumber(2) {
                    on_ground = output.clone().grounded;
                }
            }

            if keyboard.just_pressed(KeyCode::Up) && on_ground {
                for (t_num, mut timer) in &mut timer_query {
                    if t_num == &PlayerNumber(2) {
                        if timer.0.finished() {
                            timer.0.reset();
                        }
                    }
                }
            }

            controller.translation = Some(p2_to_move);
        }
    }
}
