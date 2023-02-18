use crate::components::{Acceleration, Player, Rotation};

pub fn player_movement(
    mut query: super::Query<(&mut Acceleration, &Rotation), super::With<Player>>,
) {
    for (mut acceleration, rotation) in query.iter_mut() {}
}
