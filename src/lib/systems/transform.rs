use crate::components::{Position, Velocity};

pub fn transform_components(mut query: super::Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}
