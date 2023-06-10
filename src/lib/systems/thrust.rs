use super::{Input, KeyCode, Res};
use crate::components::{Player, Thrust};
use crate::keybindings::UP;

const SIGNIFICANT_THRUST_THRESHOLD: f32 = 0.005;
const ADDITION_PER_PRESSED_TIMESTEP: f32 = 0.008;
const COOEF_LOSS_PER_UNPRESSED_TIMESTEP: f32 = 0.06;

fn taper_off(value: f32, coefficient: f32) -> f32 {
    value * (1.0 - coefficient)
}

pub fn player_thrust(
    mut query: super::Query<&mut Thrust, super::With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    for mut thrust_entity in query.iter_mut() {
        let thrust_value: f32 = if keys.any_pressed(UP) {
            thrust_entity.get() + ADDITION_PER_PRESSED_TIMESTEP
        } else {
            super::apply_threshold(
                taper_off(thrust_entity.get(), COOEF_LOSS_PER_UNPRESSED_TIMESTEP),
                0.0,
                SIGNIFICANT_THRUST_THRESHOLD,
            )
        };
        thrust_entity.set(thrust_value);
    }
}
