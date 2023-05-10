use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Resource)]
struct GreetTimer(Timer);

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_players)
            .add_system(greet_players);
        // add things to the app here
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Health(f64);

#[derive(Component)]
struct PlayerNumber(u8);

#[derive(Component)]
enum Character {
    Brisket,
    Testament,
    Whatever,
}

fn add_players(mut commands: Commands) {
    commands.spawn((
        Player,
        Name("Erin".to_string()),
        Health(10.0),
        PlayerNumber(1),
        Character::Testament,
    ));
    commands.spawn((
        Player,
        Name("tqbed".to_string()),
        Health(10.0),
        PlayerNumber(2),
        Character::Brisket,
    ));
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
