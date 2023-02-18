pub mod components {
    use bevy::prelude::Component;

    mod acceleration;
    mod health;
    mod name;
    mod player;
    mod position;
    mod rotation;
    mod velocity;

    pub use acceleration::Acceleration;
    pub use health::Health;
    pub use name::Name;
    pub use player::Player;
    pub use position::Position;
    pub use rotation::Rotation;
    pub use velocity::Velocity;
}
pub mod systems {
    use bevy::prelude::*;

    mod accelerate;
    mod damage_detection;
    mod player_movement;
    mod spawn_player;
    mod transform;

    pub use accelerate::accelerate_components;
    pub use spawn_player::spawn_player;
    pub use transform::transform_components;
}
pub mod plugins {
    // use bevy::prelude::{App, Plugin};
}
pub mod resources {
    // use bevy::prelude::Resource;
}
pub mod bundles {
    // use crate::components;
    // use bevy::prelude::Bundle;
}
