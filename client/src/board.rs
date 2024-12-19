use crate::communication::MessageSender;
use crate::game_state::GamesState;
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::input::ButtonInput;
use bevy::prelude::{AssetServer, Camera2d, Commands, KeyCode, Res, ResMut, Sprite, Transform};
use bevy::tasks::AsyncComputeTaskPool;
use shared::action::{Action, PlayerAction};

const TILE_WIDTH: f32 = 110.0; // Width of an isometric tile
const TILE_HEIGHT: f32 = 63.0; // Height of an isometric tile
const GRID_SIZE: usize = 11; // Number of tiles along one edge (must be odd)

const SPRITES_PATH: [&'static str; 5] = [
    "sprites/alienBeige_badge2.png",
    "sprites/alienBlue_badge2.png",
    "sprites/alienGreen_badge2.png",
    "sprites/alienPink_badge2.png",
    "sprites/alienYellow_badge2.png",
];

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Load the texture for tiles
    let grass_texture = asset_server.load("textures/voxelTile_55.png");
    let test = asset_server.load("textures/platformerTile_36.png");

    for (i, (col, row)) in generate_positions().iter().enumerate() {
        let x = (col - row) * (TILE_WIDTH / 2.0);
        let y = -(col + row) * (TILE_HEIGHT / 2.0);
        //let texture = if i == 33 { grass_texture.clone() } else { test.clone() };
        spawn_tile(
            &mut commands,
            grass_texture.clone(),
            test.clone(),
            i as i32,
            *col,
            *row,
            x,
            y,
        );
    }
}

fn generate_positions() -> Vec<(f32, f32)> {
    let mut positions = Vec::new();
    for i in 0..GRID_SIZE - 1 {
        positions.push(((GRID_SIZE - 1 - i) as f32, (GRID_SIZE - 1) as f32));
        positions.push((0f32, (GRID_SIZE - 1 - i) as f32));
        positions.push((i as f32, 0f32));
        positions.push(((GRID_SIZE - 1) as f32, i as f32));
    }
    positions
}

fn spawn_tile(
    commands: &mut Commands,
    grass_texture: Handle<Image>,
    test: Handle<Image>,
    pos: i32,
    col: f32,
    row: f32,
    x: f32,
    y: f32,
) {
    commands.spawn((
        Sprite {
            image: if pos == 33 {
                grass_texture.clone()
            } else {
                test.clone()
            },
            ..Default::default()
        },
        Transform::from_xyz(
            // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
            x,
            y,
            row + col,
        ),
    ));
}

pub(crate) fn spawn_players(
    commands: &mut Commands,
    asset_server: &AssetServer,
    nbr_player: usize,
) {
    let pos = generate_positions();
    for i in 0..nbr_player {
        println!("Spawning player {}", i);
        let player_texture = asset_server.load(SPRITES_PATH[i]);
        commands.spawn((
            Sprite {
                image: player_texture,
                ..Default::default()
            },
            Transform::from_xyz(
                pos[0].0 + i as f32 * (TILE_WIDTH / nbr_player as f32),
                pos[0].1 + i as f32 * (TILE_WIDTH / nbr_player as f32),
                32f32,
            ),
        ));
    }
}

pub(crate) fn roll_dice(keyboard_input: Res<ButtonInput<KeyCode>>, mut sender: ResMut<MessageSender>, games_state: Res<GamesState>) {
    if keyboard_input.just_pressed(KeyCode::Space) && games_state.player_turn == games_state.id {
        println!("Rolling dice");
        // Spawn a new async task to send the action
        let sender = sender.clone();
        let task_pool = AsyncComputeTaskPool::get();
        task_pool.spawn(async move {
            sender.0.send(PlayerAction {
                action_type: Action::Roll,
                data: None,
            }).await.unwrap();
        }).detach();
    }
}
