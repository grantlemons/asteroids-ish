use crate::components::{Position, Rotation, Thrust, Velocity};

#[derive(super::Bundle, Default, Debug)]
pub struct Kinematics {
    rotation: Rotation,
    position: Position,
    velocity: Velocity,
    thrust: Thrust,
}

impl Kinematics {
    pub fn builder() -> KinematicsBuilder {
        KinematicsBuilder::new()
    }
    pub fn rotation<T: Into<f32>>(&mut self, angle: T) {
        self.rotation.set(angle);
    }
    pub fn position<T: Into<f32>, T2: Into<f32>>(&mut self, x: T, y: T2) {
        self.position = Position {
            x: x.into(),
            y: y.into(),
        };
    }
    pub fn velocity<T: Into<f32>, T2: Into<f32>>(&mut self, x: T, y: T2) {
        self.velocity = Velocity {
            x: x.into(),
            y: y.into(),
        };
    }
    pub fn thrust<T: Into<f32>>(&mut self, thrust: T) {
        self.thrust.set(thrust)
    }
}

pub struct KinematicsBuilder {
    rotation: Option<Rotation>,
    position: Option<Position>,
    velocity: Option<Velocity>,
    thrust: Option<Thrust>,
}

impl KinematicsBuilder {
    pub fn new() -> Self {
        Self {
            rotation: None,
            position: None,
            velocity: None,
            thrust: None,
        }
    }
    pub fn rotation<T: Into<f32>>(self, angle: T) -> Self {
        Self {
            rotation: Some(Rotation::new(angle)),
            ..self
        }
    }
    pub fn position<T: Into<f32>, T2: Into<f32>>(self, x: T, y: T2) -> Self {
        Self {
            position: Some(Position::new(x, y)),
            ..self
        }
    }
    pub fn velocity<T: Into<f32>, T2: Into<f32>>(self, x: T, y: T2) -> Self {
        Self {
            velocity: Some(Velocity::new(x, y)),
            ..self
        }
    }
    pub fn thrust<T: Into<f32>>(self, thrust: T) -> Self {
        Self {
            thrust: Some(Thrust::new(thrust)),
            ..self
        }
    }
    pub fn build(self) -> Kinematics {
        Kinematics {
            rotation: self.rotation.unwrap_or_default(),
            position: self.position.unwrap_or_default(),
            velocity: self.velocity.unwrap_or_default(),
            thrust: self.thrust.unwrap_or_default(),
        }
    }
}

impl Default for KinematicsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
