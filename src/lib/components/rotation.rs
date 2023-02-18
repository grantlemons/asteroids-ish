#[derive(super::Component, Default, Debug, PartialEq)]
pub struct Rotation(pub f32);

impl std::fmt::Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add<Self> for Rotation {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::AddAssign<Self> for Rotation {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0
    }
}
