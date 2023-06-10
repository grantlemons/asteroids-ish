use super::{Input, KeyCode, Res};
use crate::components::{Player, Rotation};
use crate::keybindings::{LEFT, RIGHT};

const ABS_ROTATION_PER_TIMESTEP: f32 = 1.6;

pub fn player_rotation(
    mut query: super::Query<&mut Rotation, super::With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    let mut rotation_value: f32 = 0.0;
    if keys.any_pressed(LEFT) {
        rotation_value = ABS_ROTATION_PER_TIMESTEP;
    } else if keys.any_pressed(RIGHT) {
        rotation_value = -ABS_ROTATION_PER_TIMESTEP;
    }

    for mut rotation in query.iter_mut() {
        let new_value = rotation.get_degrees() + rotation_value;
        rotation.set_degrees(new_value);
    }
}
