use crate::bundles::PlayerBundle;
use bevy::prelude::*;

const MIN_HEALTH: u8 = 0;
const MAX_HEALTH: u8 = 3;

const DEFAULT_ROTATION: f32 = 90.0;

const SPRITE_COLOR: Color = Color::rgb(0.25, 0.25, 0.75);

pub fn spawn_player(mut commands: super::Commands) {
    let player: PlayerBundle = PlayerBundle::builder()
        .name("Player Character")
        .health(MIN_HEALTH, MAX_HEALTH, MAX_HEALTH)
        .rotation(DEFAULT_ROTATION)
        .build();
    commands.spawn_empty().insert(player).insert(SpriteBundle {
        sprite: Sprite {
            color: SPRITE_COLOR,
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        ..default()
    });
}
