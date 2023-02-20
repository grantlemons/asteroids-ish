use crate::components::{Rotation, Thrust, Velocity};

const DRAG: f32 = 0.05;
const THRESHOLD: f32 = 0.01;

fn apply_threshold<T: Into<f32> + From<f32>>(value: T, snap_value: f32) -> T {
    let val_float: f32 = value.into();
    if val_float.abs() < (snap_value + THRESHOLD) {
        T::from(0.0)
    } else {
        T::from(val_float)
    }
}

pub fn accelerate_components(mut query: super::Query<(&mut Velocity, &Rotation, &Thrust)>) {
    for (mut velocity, rotation, thrust) in query.iter_mut() {
        velocity.x = apply_threshold(velocity.x * (1.0 - DRAG), 0.0)
            + (rotation.get_radians().cos() * (thrust.get()));
        velocity.y = apply_threshold(velocity.y * (1.0 - DRAG), 0.0)
            + (rotation.get_radians().sin() * (thrust.get()));
    }
}
