use super::{Camera2dBundle, Commands};

pub fn spawn_camera(mut commands: Commands) {
    let camera = Camera2dBundle::default();
    commands.spawn(camera);
}
