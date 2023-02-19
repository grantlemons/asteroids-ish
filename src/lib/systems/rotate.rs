use super::{Input, KeyCode, Res};
use crate::components::{Player, Rotation};

pub fn player_rotation(
    mut query: super::Query<&mut Rotation, super::With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    let mut rotation_value: f32 = 0.0;
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        rotation_value = 0.6;
    } else if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        rotation_value = -0.6;
    }

    for mut rotation in query.iter_mut() {
        let new_value = rotation.get_degrees() + rotation_value;
        rotation.set_degrees(new_value);
    }
}
