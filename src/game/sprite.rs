use crate::game::player::PlayerNumber;
use crate::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterControllerOutput;
use bevy_titan::SpriteSheetLoaderPlugin;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SpriteSheetLoaderPlugin)
            .add_system(load_spritesheets.in_schedule(OnEnter(AppState::InGame)))
            .add_system(animate_players.in_set(OnUpdate(AppState::InGame)));
        // .add_system(flip_players.in_set(OnUpdate(AppState::InGame)));
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
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
    )>,
    // mut sprites: Query<(&PlayerNumber, &mut Transform)>,
    // outputs: Query<(&PlayerNumber, &KinematicCharacterControllerOutput)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }

        // for (player, mut trans) in sprites.iter_mut() {
        //     for (num, output) in outputs.iter() {
        //         if num == player {
        //             trans.translation.x += output.effective_translation.x;
        //             trans.translation.y += output.effective_translation.y;
        //         }
        //     }
        // }
    }
}

fn load_spritesheets(mut commands: Commands, asset_server: Res<AssetServer>) {
    /* Player One */
    let texture_atlas_handle = asset_server.load("characters/test/idle.titan");
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_xyz(-464.002, -254.0, 0.0).with_scale(Vec3::splat(4.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    /* Player Two */
    // commands.spawn((
    //     SpriteSheetBundle {
    //         texture_atlas: texture_atlas_handle.clone(),
    //         transform: Transform::from_xyz(464.002, -254.0, 0.0).with_scale(Vec3::splat(4.0)),
    //         ..default()
    //     },
    //     AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    //     PlayerNumber(2),
    // ));
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
