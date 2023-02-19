use super::{Input, KeyCode, Res};
use crate::components::{Player, Thrust};

fn taper_off(value: f32, coefficient: f32) -> f32 {
    if value < 0.01 {
        return 0.0;
    }
    value * coefficient
}

pub fn player_thrust(
    mut query: super::Query<&mut Thrust, super::With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    let mut thrust_value: f32 = 0.0;
    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        thrust_value = 1.0;
    } else {
        thrust_value = taper_off(thrust_value, 0.9);
    }

    for mut thrust_entity in query.iter_mut() {
        thrust_entity.set(thrust_value);
    }
}
