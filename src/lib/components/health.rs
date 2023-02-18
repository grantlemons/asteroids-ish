#[derive(super::Component)]
pub struct Health(u8);

impl std::ops::Add<Self> for Health {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0)
    }
}
