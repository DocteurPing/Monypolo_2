use crate::game_state::GamesState;
use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct MoneyText;

pub(crate) fn scoreboard_system(
    game: Res<GamesState>,
    mut display: Single<&mut Text, With<MoneyText>>,
) {
    if let Some(player) = game.players.get(&game.id) {
        display.0 = format!("Money: {}", player.money);
    }
}
