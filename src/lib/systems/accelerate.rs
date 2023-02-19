use crate::components::{Rotation, Thrust, Velocity};

const DRAG: f32 = 0.005;

pub fn accelerate_components(mut query: super::Query<(&mut Velocity, &Rotation, &Thrust)>) {
    for (mut velocity, rotation, thrust) in query.iter_mut() {
        velocity.x = (velocity.x * (1.0 - DRAG)) + (rotation.get_radians().cos() * thrust.get());
        velocity.y = (velocity.y * (1.0 - DRAG)) + (rotation.get_radians().sin() * thrust.get());
    }
}
