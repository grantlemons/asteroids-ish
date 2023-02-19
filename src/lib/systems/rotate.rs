use super::{Input, KeyCode, Res};
use crate::components::{Player, Rotation};

fn taper_off(value: f32, coefficient: f32) -> f32 {
    if value < 0.01 {
        return 0.0;
    }
    value * coefficient
}

pub fn player_rotation(
    mut query: super::Query<&mut Rotation, super::With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    let mut rotation_value: f32 = 0.0;
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        rotation_value = 1.0;
    } else if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        rotation_value = -1.0;
    } else {
        rotation_value = taper_off(rotation_value, 0.3);
    }

    for mut rotation in query.iter_mut() {
        let new_value = rotation.get() + rotation_value;
        rotation.set(new_value);
    }
}
