use crate::systems::*;
use bevy::{prelude::SystemSet, time::FixedTimestep};

pub struct KinematicsPlugin;

const TIMESTEP_60_PER_SECOND: f64 = 1.0 / 60.0;

impl super::Plugin for KinematicsPlugin {
    fn build(&self, app: &mut super::App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_60_PER_SECOND))
                .with_system(accelerate_components)
                .with_system(transform_components)
                .with_system(apply_position),
        );
    }
}
