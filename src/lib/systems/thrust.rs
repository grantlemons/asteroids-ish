use super::{Input, KeyCode, Res};
use crate::components::{Player, Thrust};

const THRESHOLD: f32 = 0.005;

fn taper_off(value: f32, coefficient: f32) -> f32 {
    value * coefficient
}

fn apply_threshold<T: Into<f32> + From<f32>>(value: T, snap_value: f32) -> T {
    let val_float: f32 = value.into();
    if val_float.abs() < (snap_value + THRESHOLD) {
        T::from(0.0)
    } else {
        T::from(val_float)
    }
}

pub fn player_thrust(
    mut query: super::Query<&mut Thrust, super::With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    for mut thrust_entity in query.iter_mut() {
        let thrust_value: f32 = if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
            0.08
        } else {
            apply_threshold(taper_off(thrust_entity.get(), 0.95), 0.0)
        };
        thrust_entity.set(thrust_value);
    }
}
