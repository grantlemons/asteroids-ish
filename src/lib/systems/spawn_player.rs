use crate::bundles::PlayerBundle;
use bevy::prelude::*;

pub fn spawn_player(mut commands: super::Commands) {
    let player: PlayerBundle = PlayerBundle::builder()
        .name("Player Character")
        .health(0, 3, 3)
        .rotation(90.0)
        .build();
    commands.spawn_empty().insert(player).insert(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(25.0, 25.0)),
            ..default()
        },
        ..default()
    });
}
