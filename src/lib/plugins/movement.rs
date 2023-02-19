use crate::systems::*;

pub struct MovementPlugin;

impl super::Plugin for MovementPlugin {
    fn build(&self, app: &mut super::App) {
        app.add_system(player_thrust)
            .add_system(player_rotation)
            .add_system(apply_rotation)
            .add_plugin(super::KinematicsPlugin);
    }
}
