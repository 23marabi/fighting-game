use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Resource)]
struct GreetTimer(Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_players)
            .add_system(greet_players)
            .add_system(check_player_state);
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

#[derive(Component, Debug)]
enum Direction {
    Left,
    Right,
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
    Moving(Direction),
    Attacking(AttackState),
    Blocking(BlockState),
}

#[derive(Component)]
struct PlayerNumber(u8);

fn add_players(mut commands: Commands) {
    commands.spawn(
        (PlayerBundle {
            name: Name("Erin".to_string()),
            hp: Health(10.0),
            num: PlayerNumber(1),
            _p: Player,
            state: PlayerState::Idle,
            _d: Dead(false),
            _j: Jumping(false),
        }),
    );
    commands.spawn(
        (PlayerBundle {
            name: Name("tqbed".to_string()),
            hp: Health(10.0),
            num: PlayerNumber(2),
            _p: Player,
            state: PlayerState::Moving(Direction::Left),
            _d: Dead(false),
            _j: Jumping(false),
        }),
    );
}

fn greet_players(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Player>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Welcome, {}!", name.0);
        }
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
