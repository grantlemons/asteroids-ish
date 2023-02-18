use crate::components::Player;

pub fn spawn_player(mut commands: super::Commands) {
    commands.spawn(Player);
}
