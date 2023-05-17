use crate::game::player::PlayerNumber;
use crate::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterControllerOutput;
use bevy_titan::SpriteSheetLoaderPlugin;

use crate::game::character::CharacterMap;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpriteSheetLoaderPlugin)
            .add_system(load_spritesheets.in_schedule(OnEnter(AppState::InGame)))
            .add_system(animate_players.in_set(OnUpdate(AppState::InGame)))
            .add_system(flip_players.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_players(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &PlayerNumber,
        &mut Transform,
    )>,
    // mut sprites: Query<(&PlayerNumber, &mut Transform)>,
    outputs: Query<(&PlayerNumber, &KinematicCharacterControllerOutput)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, num, mut trans) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = match texture_atlases.get(texture_atlas_handle) {
                Some(t) => t,
                None => return,
            };
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }

        for (player, output) in outputs.iter() {
            if num == player {
                trans.translation.x += output.effective_translation.x;
                trans.translation.y += output.effective_translation.y;
            }
        }
    }
}

fn load_spritesheets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: ResMut<CharacterMap>,
) {
    /* Player One */
    let mut path = map
        .0
        .get("Test")
        .unwrap()
        .get_path()
        .as_ref()
        .unwrap()
        .strip_prefix("./assets/")
        .unwrap()
        .to_path_buf();

    path.pop();
    path.push("idle.titan");
    let path = path.to_string_lossy().into_owned();

    let texture_atlas_handle = asset_server.load(path);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(-500.0, 0.0, 0.0).with_scale(Vec3::splat(8.0)),
            ..default()
        },
        PlayerNumber(1),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    /* Player Two */
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(500.0, 0.0, 0.0).with_scale(Vec3::splat(8.0)),
            ..default()
        },
        PlayerNumber(2),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

fn flip_players(mut query: Query<(&PlayerNumber, &Transform, &mut TextureAtlasSprite)>) {
    let mut player1_location = Vec3::ZERO;
    let mut player2_location = Vec3::ZERO;

    for (num, trans, mut sprite) in query.iter_mut() {
        match num {
            PlayerNumber(1) => player1_location = trans.translation.clone(),
            PlayerNumber(2) => player2_location = trans.translation.clone(),
            _ => {}
        }
    }

    if player1_location.x < player2_location.x {
        for (num, trans, mut sprite) in query.iter_mut() {
            match num {
                PlayerNumber(1) => sprite.flip_x = false,
                PlayerNumber(2) => sprite.flip_x = true,
                _ => {}
            }
        }
    } else {
        for (num, trans, mut sprite) in query.iter_mut() {
            match num {
                PlayerNumber(1) => sprite.flip_x = true,
                PlayerNumber(2) => sprite.flip_x = false,
                _ => {}
            }
        }
    }
}
