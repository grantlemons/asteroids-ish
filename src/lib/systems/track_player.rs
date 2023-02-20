use super::{Camera, Query, Transform, With, Without};
use crate::components::Player;

const FOLLOW_INTENSITY: f32 = 0.1;
const THRESHOLD: f32 = 0.1;

fn apply_threshold<T: Into<f32> + From<f32>>(value: T, snap_value: f32) -> T {
    let val_float: f32 = value.into();
    if val_float.abs() < (snap_value + THRESHOLD) {
        T::from(0.0)
    } else {
        T::from(val_float)
    }
}

pub fn track_player(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: super::Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let Some(player_transform) = player_query.iter().next() {
        for mut camera_transform in camera_query.iter_mut() {
            let x_diff = player_transform.translation.x - camera_transform.translation.x;
            let y_diff = player_transform.translation.y - camera_transform.translation.y;

            camera_transform.translation.x += apply_threshold(x_diff * FOLLOW_INTENSITY, 0.0);
            camera_transform.translation.y += apply_threshold(y_diff * FOLLOW_INTENSITY, 0.0);
        }
    }
}
