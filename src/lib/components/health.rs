#[derive(super::Component, Debug, PartialEq)]
pub struct Health {
    min: u8,
    max: u8,
    value: u8,
}

impl Health {
    pub fn new(min: u8, max: u8, value: u8) -> Self {
        Self { min, max, value }
    }

    pub fn get(&self) -> u8 {
        self.value
    }

    pub fn set(&mut self, new: u8) {
        self.value = new.clamp(self.min, self.max);
    }
}

impl Default for Health {
    fn default() -> Self {
        Self {
            min: 0,
            max: 10,
            value: 10,
        }
    }
}

impl<T: Into<u8>> std::ops::AddAssign<T> for Health {
    fn add_assign(&mut self, rhs: T) {
        self.set(self.get() + rhs.into());
    }
}

impl From<Health> for u8 {
    fn from(value: Health) -> Self {
        value.get()
    }
}
