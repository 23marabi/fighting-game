use crate::AppState;
use bevy::prelude::*;
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
pub struct JumpTimer(pub Timer);

#[derive(Component)]
struct Countdown(Timer);

#[derive(Component)]
struct Direction(Vec2);

#[derive(Component)]
struct Velocity(Vec2);

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

    commands.spawn(Countdown(Timer::from_seconds(0.5, TimerMode::Once)));
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
    mut count: Query<&mut Countdown>,
) {
    let mut m_move = false;
    count.for_each_mut(|mut timer| {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            m_move = true;
        }
    });

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

    let mut p1_j_accel = 0.0;
    let mut p2_j_accel = 0.0;
    for (t_num, timer) in timer_query.iter() {
        match t_num {
            PlayerNumber(1) => p1_j_accel = timer.0.percent() * time.delta_seconds() + 1.0,
            PlayerNumber(2) => p2_j_accel = timer.0.percent() * time.delta_seconds() + 1.0,
            _ => {}
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
        if num == &PlayerNumber(1) && m_move {
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
                        p1_to_move.y * movement.jump_speed * p1_j_accel,
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
        } else if num == &PlayerNumber(2) && m_move {
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
                        p2_to_move.y * movement.jump_speed * p2_j_accel,
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
