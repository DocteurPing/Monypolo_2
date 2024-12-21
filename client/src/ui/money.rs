use crate::game_state::GamesState;
use bevy::prelude::*;

pub(crate) fn scoreboard_system(game: Res<GamesState>, mut display: Single<&mut Text>) {
    if let Some(player) = game.players.get(&game.id) {
        display.0 = format!("Money: {}", player.money);
    }
}
