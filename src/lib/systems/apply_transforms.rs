use crate::components::{Position, Rotation};

pub fn apply_position(mut query: super::Query<(&mut super::Transform, &Position)>) {
    for (mut transform, position) in query.iter_mut() {
        transform.translation = super::Vec3::from((position.x, position.y, 0.0));
    }
}

pub fn apply_rotation(mut query: super::Query<(&mut super::Transform, &Rotation)>) {
    for (mut transform, rotation) in query.iter_mut() {
        transform.rotation = super::Quat::from_rotation_z(rotation.get());
    }
}
