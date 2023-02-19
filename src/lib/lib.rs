pub mod components {
    use bevy::prelude::Component;

    mod health;
    mod name;
    mod player;
    mod position;
    mod rotation;
    mod thrust;
    mod velocity;

    pub use health::Health;
    pub use name::Name;
    pub use player::Player;
    pub use position::Position;
    pub use rotation::Rotation;
    pub use thrust::Thrust;
    pub use velocity::Velocity;
}
pub mod systems {
    use bevy::prelude::*;

    mod accelerate;
    mod apply_transforms;
    mod damage_detection;
    mod rotate;
    mod spawn_camera;
    mod spawn_player;
    mod thrust;
    mod transform;

    pub use accelerate::accelerate_components;
    pub use apply_transforms::{apply_position, apply_rotation};
    pub use rotate::player_rotation;
    pub use spawn_camera::spawn_camera;
    pub use spawn_player::spawn_player;
    pub use thrust::player_thrust;
    pub use transform::transform_components;
}
pub mod plugins {
    use bevy::prelude::{App, Plugin};

    mod kinematics;
    mod movement;
    pub use kinematics::KinematicsPlugin;
    pub use movement::MovementPlugin;
}
pub mod resources {
    // use bevy::prelude::Resource;
}
pub mod bundles {
    use bevy::prelude::Bundle;

    mod kinematics;
    mod player;
    pub use kinematics::Kinematics;
    pub use kinematics::KinematicsBuilder;
    pub use player::PlayerBundle;
    pub use player::PlayerBundleBuilder;
}
