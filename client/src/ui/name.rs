use crate::game_state::GamesState;
use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct NameText;

pub(crate) fn name_system(game: Res<GamesState>, mut display: Single<&mut Text, With<NameText>>) {
    if let Some(player) = game.players.get(&game.id) {
        display.0 = format!("Name: {}", player.name);
    }
}
