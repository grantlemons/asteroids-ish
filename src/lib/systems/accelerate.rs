use crate::components::{Acceleration, Velocity};

pub fn accelerate_components(mut query: super::Query<(&mut Velocity, &Acceleration)>) {
    for (mut velocity, acceleration) in query.iter_mut() {
        velocity.x += acceleration.x;
        velocity.y += acceleration.y;
    }
}
