use crate::systems::*;

pub struct KinematicsPlugin;

impl super::Plugin for KinematicsPlugin {
    fn build(&self, app: &mut super::App) {
        app.add_system(accelerate_components)
            .add_system(transform_components)
            .add_system(apply_position);
    }
}
