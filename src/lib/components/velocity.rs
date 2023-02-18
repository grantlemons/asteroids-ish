use super::Acceleration;

#[derive(Default, Debug, super::Component, PartialEq)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl std::fmt::Display for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.x, self.y)
    }
}

impl std::ops::Add<Self> for Velocity {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Add<Acceleration> for Velocity {
    type Output = Self;

    fn add(self, other: Acceleration) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign<Self> for Velocity {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::AddAssign<Acceleration> for Velocity {
    fn add_assign(&mut self, other: Acceleration) {
        self.x += other.x;
        self.y += other.y;
    }
}
