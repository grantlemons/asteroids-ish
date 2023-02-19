use super::{Kinematics, KinematicsBuilder};
use crate::components::{Health, Name, Player};

#[derive(super::Bundle, Default, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    pub name: Name,
    pub health: Health,
    pub kinematics: Kinematics,
}

impl PlayerBundle {
    pub fn builder() -> PlayerBundleBuilder {
        PlayerBundleBuilder::new()
    }
}

pub struct PlayerBundleBuilder {
    name: Option<Name>,
    health: Option<Health>,
    kinematics: KinematicsBuilder,
}

impl PlayerBundleBuilder {
    pub fn new() -> Self {
        PlayerBundleBuilder {
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
    pub fn build(self) -> PlayerBundle {
        PlayerBundle {
            player: Player::default(),
            name: self.name.unwrap_or_default(),
            health: self.health.unwrap_or_default(),
            kinematics: self.kinematics.build(),
        }
    }
}

impl Default for PlayerBundleBuilder {
    fn default() -> Self {
        Self::new()
    }
}
