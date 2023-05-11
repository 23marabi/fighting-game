use crate::AppState;
use bevy::{
    input::gamepad::{
        GamepadAxisChangedEvent, GamepadButtonChangedEvent, GamepadConnectionEvent, GamepadEvent,
    },
    prelude::*,
};
use bevy_rapier2d::prelude::*;

const SPEED: f32 = 500.0;
const FRICTION: f32 = 2.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(greet_players.in_schedule(OnEnter(AppState::InGame)))
            .add_system(gamepad_ordered_events)
            .add_system(check_player_state.in_set(OnUpdate(AppState::InGame)))
            .add_system(keyboard_input.in_set(OnUpdate(AppState::InGame)))
            .add_system(init_physics.in_schedule(OnEnter(AppState::InGame)))
            .add_system(update_physics.in_set(OnUpdate(AppState::InGame)))
            .add_system(read_result_system.in_set(OnUpdate(AppState::InGame)))
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
                x = keys.pressed(KeyCode::A) as i32 - keys.pressed(KeyCode::D) as i32;
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
        velocity.0 += (target_velocity - old_velocity) * FRICTION;
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

fn init_physics(mut commands: Commands) {
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::capsule(
            Vec2::new(0.0, 20.0),
            Vec2::new(0.0, -20.0),
            20.0,
        ))
        .insert(KinematicCharacterController::default())
        .insert(PlayerNumber(1));
}

fn update_physics(
    mut controllers: Query<(&PlayerNumber, &mut KinematicCharacterController)>,
    mut velocities: Query<(&PlayerNumber, &mut Velocity)>,
) {
    for (num, mut controller) in controllers.iter_mut() {
        for (v_num, mut velocity) in velocities.iter_mut() {
            if v_num == num {
                controller.translation = Some(velocity.0);
                println!("Player {:?} Controller", num);
            }
        }
    }
}

fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!(
            "Entity {:?} moved by {:?} and touches the ground: {:?}",
            entity, output.effective_translation, output.grounded
        );
    }
}
