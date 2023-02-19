use crate::systems::*;

pub struct MovementPlugin;

impl super::Plugin for MovementPlugin {
    fn build(&self, app: &mut super::App) {
        app.add_system(accelerate_components)
            .add_system(transform_components)
            .add_system(player_thrust)
            .add_system(player_rotation);
    }
}
