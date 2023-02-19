use crate::components::Player;

pub fn spawn_player(mut commands: super::Commands) {
    let player: Player = Player::builder()
        .name("Player Character")
        .health(0, 3, 3)
        .rotation(90.0)
        .build();
    commands.spawn_empty().insert(player);
}
