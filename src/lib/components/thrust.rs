#[derive(super::Component, Default, Debug, PartialEq)]
pub struct Thrust(f32);

impl Thrust {
    pub fn new<T: Into<f32>>(thrust: T) -> Self {
        Self(thrust.into())
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set<T: Into<f32>>(&mut self, new: T) {
        self.0 = new.into().clamp(0.0, 1.0)
    }
}
