use crate::components::{Rotation, Thrust, Velocity};

const DRAG_COEFFICIENT: f32 = 0.05;
const SIGNIFICANT_ACCELERATION_THRESHOLD: f32 = 0.01;

pub fn accelerate_components(mut query: super::Query<(&mut Velocity, &Rotation, &Thrust)>) {
    for (mut velocity, rotation, thrust) in query.iter_mut() {
        velocity.x = super::apply_threshold(
            velocity.x * (1.0 - DRAG_COEFFICIENT),
            0.0,
            SIGNIFICANT_ACCELERATION_THRESHOLD,
        ) + (rotation.get_radians().cos() * (thrust.get()));
        velocity.y = super::apply_threshold(
            velocity.y * (1.0 - DRAG_COEFFICIENT),
            0.0,
            SIGNIFICANT_ACCELERATION_THRESHOLD,
        ) + (rotation.get_radians().sin() * (thrust.get()));
    }
}
