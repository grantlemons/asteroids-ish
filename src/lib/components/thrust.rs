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
        self.0 = new.into().clamp(0.0, 1.0);
    }
}

impl From<f32> for Thrust {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}

impl std::ops::AddAssign<Self> for Thrust {
    fn add_assign(&mut self, rhs: Self) {
        self.set(self.get() + rhs.get());
    }
}
