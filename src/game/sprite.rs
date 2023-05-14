use crate::game::player::PlayerNumber;
use crate::AppState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::KinematicCharacterControllerOutput;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(load_spritesheets.in_schedule(OnEnter(AppState::InGame)))
            .add_system(animate_players.in_set(OnUpdate(AppState::InGame)));
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
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &PlayerNumber,
        &mut Transform,
    )>,
    outputs: Query<(&PlayerNumber, &KinematicCharacterControllerOutput)>,
) {
    for (indices, mut timer, mut sprite, player, mut trans) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }

        for (num, output) in outputs.iter() {
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    /* Player One */
    let texture_handle = asset_server.load("characters/test/idle.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(-464.002, -254.0, 0.0).with_scale(Vec3::splat(4.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerNumber(1),
    ));

    /* Player Two */
    let texture_handle = asset_server.load("characters/test/idle.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 1, last: 6 };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_xyz(464.002, -254.0, 0.0).with_scale(Vec3::splat(4.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerNumber(2),
    ));
}
