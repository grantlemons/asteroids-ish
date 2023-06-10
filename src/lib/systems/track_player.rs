use super::{Camera, Query, Transform, With, Without};
use crate::components::Player;

const FOLLOW_INTENSITY: f32 = 0.1;
const SIGNIFICANT_DIFFERENCE_THRESHOLD: f32 = 0.1;

pub fn track_player(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: super::Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let Some(player_transform) = player_query.iter().next() {
        for mut camera_transform in camera_query.iter_mut() {
            let x_diff = player_transform.translation.x - camera_transform.translation.x;
            let y_diff = player_transform.translation.y - camera_transform.translation.y;

            camera_transform.translation.x += super::apply_threshold(
                x_diff * FOLLOW_INTENSITY,
                0.0,
                SIGNIFICANT_DIFFERENCE_THRESHOLD,
            );
            camera_transform.translation.y += super::apply_threshold(
                y_diff * FOLLOW_INTENSITY,
                0.0,
                SIGNIFICANT_DIFFERENCE_THRESHOLD,
            );
        }
    }
}
