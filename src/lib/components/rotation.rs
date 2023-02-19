/// Rotation starts at 0 degrees from the left
#[derive(super::Component, Default, Debug, PartialEq)]
pub struct Rotation(pub f32);

impl Rotation {
    pub fn new<T: Into<f32>>(degrees: T) -> Self {
        Self(degrees.into().rem_euclid(365.0))
    }

    pub fn get_degrees(&self) -> f32 {
        self.0
    }

    pub fn get_radians(&self) -> f32 {
        self.0.to_radians()
    }

    pub fn set_degrees<T: Into<f32>>(&mut self, new: T) {
        self.0 = new.into().rem_euclid(365.0);
    }
}

impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<f32>> std::ops::Add<T> for Rotation {
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        Self::new(self.get_degrees() + other.into())
    }
}

impl<T: Into<f32>> std::ops::AddAssign<T> for Rotation {
    fn add_assign(&mut self, rhs: T) {
        self.set_degrees(self.get_degrees() + rhs.into());
    }
}

impl From<f32> for Rotation {
    fn from(value: f32) -> Self {
        Self::new(value)
    }
}
