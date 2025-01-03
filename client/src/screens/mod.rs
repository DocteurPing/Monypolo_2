use crate::screens;
use crate::screens::board::TILE_HEIGHT;
use bevy::prelude::{Camera2d, Commands, States, Transform};

pub mod board;
pub mod menu;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub(crate) enum GameState {
    #[default]
    Menu,
    Game,
}

pub(crate) fn add_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(
            0.0,
            -TILE_HEIGHT * ((screens::board::GRID_SIZE as f32 / 3.0).round() + 1.0),
            100.0,
        ),
    ));
}
