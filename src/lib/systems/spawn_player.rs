use crate::bundles::Kinematics;
use crate::components::{Health, Player};

pub fn spawn_player(mut commands: super::Commands) {
    commands
        .spawn_empty()
        .insert((Player, Health::default(), Kinematics::default()));
}
