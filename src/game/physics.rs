use crate::AppState;
use bevy::{input::gamepad::GamepadEvent, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::game::player::{MovementData, PlayerNumber};
use crate::settings::Settings;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_system(setup_physics.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_physics.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Component)]
struct JumpTimer(Timer);

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

fn gamepad_ordered_events(mut gamepad_events: EventReader<GamepadEvent>) {
    for gamepad_event in gamepad_events.iter() {
        match gamepad_event {
            GamepadEvent::Connection(connection_event) => info!("{:?}", connection_event),
            GamepadEvent::Button(button_event) => info!("{:?}", button_event),
            GamepadEvent::Axis(axis_event) => info!("{:?}", axis_event),
        }
    }
}

fn setup_physics(mut commands: Commands, s: Res<Settings>) {
    /* Create the ground & ceiling */
    commands
        .spawn(Collider::cuboid(
            s.physics.ground.dimensions.0,
            s.physics.ground.dimensions.1,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            s.physics.ground.position.0,
            s.physics.ground.position.1,
            0.0,
        )));
    commands
        .spawn(Collider::cuboid(
            s.physics.ceiling.dimensions.0,
            s.physics.ceiling.dimensions.1,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            s.physics.ceiling.position.0,
            s.physics.ceiling.position.1,
            0.0,
        )));

    /* Create the walls */
    commands
        .spawn(Collider::cuboid(
            s.physics.left_wall.dimensions.0,
            s.physics.left_wall.dimensions.1,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            s.physics.left_wall.position.0,
            s.physics.left_wall.position.1,
            0.0,
        )));
    commands
        .spawn(Collider::cuboid(
            s.physics.right_wall.dimensions.0,
            s.physics.right_wall.dimensions.1,
        ))
        .insert(TransformBundle::from(Transform::from_xyz(
            s.physics.right_wall.position.0,
            s.physics.right_wall.position.1,
            0.0,
        )));
    commands.spawn((
        PlayerNumber(1),
        JumpTimer(Timer::from_seconds(0.35, TimerMode::Once)),
    ));

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
