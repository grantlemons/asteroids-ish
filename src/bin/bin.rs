#[allow(unused_imports)]
use asteroids_lib as lib;
use bevy::prelude::*;

#[allow(unused_imports)]
use lib::{components::*, plugins::*, systems::*};

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins)
        .run();
}
