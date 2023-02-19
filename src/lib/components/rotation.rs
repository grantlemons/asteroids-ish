/// Rotation starts at 0 degrees from the left
#[derive(super::Component, Default, Debug, PartialEq)]
pub struct Rotation(pub f32);

impl Rotation {
    pub fn new<T: Into<f32>>(angle: T) -> Self {
        Self(angle.into())
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn set<T: Into<f32>>(&mut self, new: T) {
        self.0 = new.into() % 365.0;
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
        Self((self.0 + other.into()) % 365.0)
    }
}

impl<T: Into<f32>> std::ops::AddAssign<T> for Rotation {
    fn add_assign(&mut self, rhs: T) {
        self.set((self.get() + rhs.into()) % 365.0);
    }
}

impl From<f32> for Rotation {
    fn from(value: f32) -> Self {
        Self::new(value % 365.0)
    }
}
