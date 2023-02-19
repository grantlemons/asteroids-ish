use crate::components::{Position, Rotation, Thrust, Velocity};

#[derive(super::Bundle, Default, Debug)]
pub struct Kinematics {
    rotation: Rotation,
    position: Position,
    velocity: Velocity,
    thrust: Thrust,
}
