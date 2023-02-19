use super::*;
use crate::bundles::{Kinematics, KinematicsBuilder};

#[derive(super::Component, Default)]
pub struct Player {
    pub name: Name,
    pub health: Health,
    pub kinematics: Kinematics,
}

impl Player {
    pub fn builder() -> PlayerBuilder {
        PlayerBuilder::new()
    }
}

pub struct PlayerBuilder {
    name: Option<Name>,
    health: Option<Health>,
    kinematics: KinematicsBuilder,
}

impl PlayerBuilder {
    pub fn new() -> Self {
        PlayerBuilder {
            name: None,
            health: None,
            kinematics: KinematicsBuilder::new(),
        }
    }
    pub fn name<T: Into<String>>(self, name: T) -> Self {
        Self {
            name: Some(Name::new(name)),
            ..self
        }
    }
    pub fn health(self, min: u8, max: u8, value: u8) -> Self {
        Self {
            health: Some(Health::new(min, max, value)),
            ..self
        }
    }
    pub fn position<T: Into<f32>, T2: Into<f32>>(self, x: T, y: T2) -> Self {
        Self {
            kinematics: self.kinematics.position(x, y),
            ..self
        }
    }
    pub fn rotation<T: Into<f32>>(self, angle: T) -> Self {
        Self {
            kinematics: self.kinematics.rotation(angle),
            ..self
        }
    }
    pub fn build(self) -> Player {
        Player {
            name: self.name.unwrap_or(Name::default()),
            health: self.health.unwrap_or(Health::default()),
            kinematics: self.kinematics.build(),
        }
    }
}

impl Default for PlayerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
