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
    for mut thrust_entity in query.iter_mut() {
        let thrust_value: f32 = if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
            0.02
        } else {
            taper_off(thrust_entity.get(), 0.95)
        };
        thrust_entity.set(thrust_value);
    }
}
