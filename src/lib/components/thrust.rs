#[derive(super::Component, Default, Debug, PartialEq)]
pub struct Thrust(pub f32);

impl Thrust {
    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set(&mut self, new: f32) {
        self.0 = new.clamp(0.0, 1.0)
    }
}
