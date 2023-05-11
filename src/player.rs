use crate::AppState;
use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadConnectionEvent, GamepadEvent,
    },
    prelude::*,
};
use bevy_easings::Lerp;
use bevy_rapier2d::prelude::*;

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
    /* Create the ground & ceiling */
    commands
        .spawn(Collider::cuboid(575.0, 20.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -325.0, 0.0)));
    commands
        .spawn(Collider::cuboid(575.0, 20.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 325.0, 0.0)));
    /* Create the walls */
    commands
        .spawn(Collider::cuboid(20.0, 325.0))
        .insert(TransformBundle::from(Transform::from_xyz(-575.0, 0.0, 0.0)));
    commands
        .spawn(Collider::cuboid(20.0, 325.0))
        .insert(TransformBundle::from(Transform::from_xyz(575.0, 0.0, 0.0)));

    /* Create the Players */
    commands
        .spawn(KinematicCharacterController {
            custom_mass: Some(10.0),
            filter_flags: QueryFilterFlags::EXCLUDE_KINEMATIC,
            ..default()
        })
        .insert(Collider::capsule(
            Vec2::new(0.0, -20.0),
            Vec2::new(0.0, 20.0),
            20.0,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            -464.002, -254.0, 0.0,
        )))
        .insert(MovementData {
            velocity: Vec2::ZERO,
            acceleration: 5.0,
            friction: 0.9,
            max_speed: 8.0,
            jump_speed: 25.0,
        })
        .insert(PlayerNumber(1));
    commands.spawn((
        PlayerNumber(1),
        JumpTimer(Timer::from_seconds(0.35, TimerMode::Once)),
    ));

    commands
        .spawn(KinematicCharacterController {
            custom_mass: Some(10.0),
            filter_flags: QueryFilterFlags::EXCLUDE_KINEMATIC,
            ..default()
        })
        .insert(Collider::capsule(
            Vec2::new(0.0, -20.0),
            Vec2::new(0.0, 20.0),
            20.0,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            464.002, -254.0, 0.0,
        )))
        .insert(MovementData {
            velocity: Vec2::ZERO,
            acceleration: 5.0,
            friction: 0.9,
            max_speed: 8.0,
            jump_speed: 25.0,
        })
        .insert(PlayerNumber(2));
    commands.spawn((
        PlayerNumber(2),
        JumpTimer(Timer::from_seconds(0.35, TimerMode::Once)),
    ));
}

fn update_physics(
    mut commands: Commands,
    time: Res<Time>,
    mut timer_query: Query<(&PlayerNumber, &mut JumpTimer)>,
    mut controllers: Query<(
        &PlayerNumber,
        &mut MovementData,
        &mut KinematicCharacterController,
    )>,
    outputs: Query<(&PlayerNumber, &KinematicCharacterControllerOutput)>,
    keyboard: Res<Input<KeyCode>>,
) {
    let mut p1_to_move = Vec2::ZERO;
    let mut p2_to_move = Vec2::ZERO;

    for (t_num, mut timer) in &mut timer_query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            if t_num == &PlayerNumber(1) {
                p1_to_move.y -= 1.0;
            } else if t_num == &PlayerNumber(2) {
                p2_to_move.y -= 1.0;
            }
        } else {
            if t_num == &PlayerNumber(1) {
                p1_to_move.y += 1.0;
            } else if t_num == &PlayerNumber(2) {
                p2_to_move.y += 1.0;
            }
        }
    }

    if keyboard.pressed(KeyCode::A) {
        p1_to_move.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) {
        p1_to_move.x += 1.0;
    }

    if keyboard.pressed(KeyCode::Left) {
        p2_to_move.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::Right) {
        p2_to_move.x += 1.0;
    }

    for (num, mut movement, mut controller) in controllers.iter_mut() {
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

            p1_to_move = p1_to_move.normalize_or_zero();
            if p1_to_move != Vec2::ZERO {
                movement.velocity = movement.velocity.lerp(
                    Vec2::new(
                        p1_to_move.x * movement.max_speed,
                        p1_to_move.y * movement.jump_speed,
                    ),
                    movement.acceleration * time.delta_seconds(),
                );
            } else {
                movement.velocity = movement
                    .velocity
                    .lerp(Vec2::ZERO, movement.friction * time.delta_seconds());
            }

            let old_translation = match controller.translation {
                Some(t) => t,
                None => Vec2::ZERO,
            };
            controller.translation = Some(old_translation + movement.velocity);
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

            p2_to_move = p2_to_move.normalize_or_zero();
            if p2_to_move != Vec2::ZERO {
                movement.velocity = movement.velocity.lerp(
                    Vec2::new(
                        p2_to_move.x * movement.max_speed,
                        p2_to_move.y * movement.jump_speed,
                    ),
                    movement.acceleration * time.delta_seconds(),
                );
            } else {
                movement.velocity = movement
                    .velocity
                    .lerp(Vec2::ZERO, movement.friction * time.delta_seconds());
            }

            let old_translation = match controller.translation {
                Some(t) => t,
                None => Vec2::ZERO,
            };
            controller.translation = Some(old_translation + movement.velocity);
        }
    }
}
