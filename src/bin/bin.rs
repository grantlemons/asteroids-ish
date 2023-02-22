use asteroids_lib as lib;
use bevy::prelude::*;
use bevy_web_fullscreen::FullViewportPlugin;

#[allow(unused_imports)]
use lib::{components::*, plugins::*, systems::*};

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    create_app().run();
}

fn create_app() -> App {
    let app = App::new()
        .insert_resource(ClearColor(CLEAR))
        .add_plugins(DefaultPlugins.set(custom_window_plugin()))
        .add_plugin(MovementPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, spawn_camera)
        .add_startup_system(spawn_player);
    // #[cfg(target_family = "wasm")]
    app.add_plugin(FullViewportPlugin);
    app
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
