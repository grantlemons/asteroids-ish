#[allow(unused_imports)]
use asteroids_lib as lib;
use bevy::prelude::*;

#[allow(unused_imports)]
use lib::{components::*, plugins::*, systems::*};

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(custom_window_plugin()))
        .add_plugin(MovementPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera)
        .add_startup_system(spawn_player)
        .run();
}

fn custom_window_plugin() -> WindowPlugin {
    WindowPlugin {
        window: WindowDescriptor {
            title: "Asteroids Game -- github.com/grantlemons/asteroids".to_string(),
            cursor_visible: false,
            fit_canvas_to_parent: true,
            ..Default::default()
        },
        ..Default::default()
    }
}
