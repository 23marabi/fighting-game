use crate::game::player::{MoveBuffer, MoveSet, MovementData, PlayerInput, PlayerNumber};
use crate::AppState;
use bevy::{input::gamepad::GamepadEvent, prelude::*, utils::Duration};
use bevy_debug_text_overlay::screen_print;

#[derive(Component)]
pub struct AttackTimer(pub Timer);

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_input.in_set(OnUpdate(AppState::InGame)))
            .add_system(attack.after(handle_input));
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

fn handle_input(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut buffer: Query<(&PlayerNumber, &mut MoveBuffer)>,
) {
    for (num, mut buff) in buffer.iter_mut() {
        match num {
            PlayerNumber(1) => {
                if keyboard.just_pressed(KeyCode::A) {
                    buff.0.push((PlayerInput::Left, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::D) {
                    buff.0.push((PlayerInput::Right, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::W) {
                    buff.0.push((PlayerInput::Up, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::S) {
                    buff.0.push((PlayerInput::Down, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::F) {
                    buff.0.push((PlayerInput::Light, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::G) {
                    buff.0.push((PlayerInput::Heavy, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::R) {
                    buff.0.push((PlayerInput::Special, time.elapsed()));
                }
            }
            PlayerNumber(2) => {
                if keyboard.just_pressed(KeyCode::Left) {
                    buff.0.push((PlayerInput::Left, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::Right) {
                    buff.0.push((PlayerInput::Right, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::Up) {
                    buff.0.push((PlayerInput::Up, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::Down) {
                    buff.0.push((PlayerInput::Down, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::H) {
                    buff.0.push((PlayerInput::Light, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::J) {
                    buff.0.push((PlayerInput::Heavy, time.elapsed()));
                }
                if keyboard.just_pressed(KeyCode::K) {
                    buff.0.push((PlayerInput::Special, time.elapsed()));
                }
            }
            _ => {}
        }
    }
}

fn attack(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&PlayerNumber, &MoveBuffer, &MoveSet, &mut AttackTimer)>,
) {
    for (num, buff, moves, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());

        let last_entry = match buff.0.last() {
            Some(v) => v.1,
            None => return,
        };

        if (time.elapsed() - last_entry) <= Duration::from_millis(20) {
            // Check for recent movement
            for (name, combo) in moves.0.iter() {
                if buff.0.len() >= combo.inputs.len() {
                    let (_, subset) = buff.0.split_at(buff.0.len() - combo.inputs.len());
                    let mut comparison: Vec<PlayerInput> = vec![];
                    for (inp, time) in subset.iter() {
                        comparison.push(*inp);
                    }
                    // TODO: check time between inputs
                    let matching = comparison
                        .iter()
                        .zip(combo.inputs.clone())
                        .filter(|&(a, b)| a == &b)
                        .count();
                    if matching == combo.inputs.len() && timer.0.finished() {
                        // Can enter animation from here
                        screen_print!(push, sec: 2.0, "{} Combo Entered: {:?}", name, combo.inputs);
                        timer.0 = Timer::from_seconds(combo.time, TimerMode::Once);
                    }
                }
            }
        }
    }
}
