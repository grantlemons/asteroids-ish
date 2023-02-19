use crate::components::{Player, Rotation, Thrust, Velocity};

const DRAG: f32 = 0.1;

pub fn accelerate_components(
    mut query: super::Query<(&mut Velocity, &Rotation, &Thrust), super::With<Player>>,
) {
    for (mut velocity, rotation, thrust) in query.iter_mut() {
        velocity.x = (velocity.x * (1.0 - DRAG)) + (rotation.get().cos() * thrust.get());
        velocity.y = (velocity.y * (1.0 - DRAG)) + (rotation.get().sin() * thrust.get());
    }
}
