use crate::communication::MessageSender;
use crate::game_state::{GamesState, Player};
use crate::ui::money::MoneyText;
use crate::ui::name::NameText;
use bevy::asset::Handle;
use bevy::image::Image;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::tasks::AsyncComputeTaskPool;
use shared::action::{Action, PlayerAction, PlayerIdentifyData};
use shared::board::Tile;
use shared::maps::map1::MAP1;

pub(crate) const TILE_WIDTH: f32 = 110.0; // Width of an isometric tile
pub(crate) const TILE_HEIGHT: f32 = 63.0; // Height of an isometric tile
const GRID_SIZE: usize = 11; // Number of tiles along one edge (must be odd)

const SPRITES_PATH: [&str; 5] = [
    "sprites/alienBeige_badge2.png",
    "sprites/alienBlue_badge2.png",
    "sprites/alienGreen_badge2.png",
    "sprites/alienPink_badge2.png",
    "sprites/alienYellow_badge2.png",
];

fn get_texture(asset_server: &Res<AssetServer>, i: usize) -> Handle<Image> {
    match MAP1[i] {
        Tile::Property { .. } => asset_server.load("textures/voxelTile_55.png"),
        Tile::Chance(_) => asset_server.load("textures/platformerTile_36.png"),
        Tile::Jail => asset_server.load("textures/platformerTile_33.png"),
        Tile::GoToJail => asset_server.load("textures/platformerTile_46.png"),
        Tile::Go { .. } => asset_server.load("textures/abstractTile_12.png"),
        Tile::FreeParking => asset_server.load("textures/abstractTile_08.png"),
        Tile::Railroad { .. } => asset_server.load("textures/platformerTile_04.png"),
        Tile::Utility { .. } => asset_server.load("textures/abstractTile_29.png"),
        Tile::Tax { .. } => asset_server.load("textures/platformerTile_42.png"),
        Tile::LuxuryTax { .. } => asset_server.load("textures/platformerTile_44.png"),
    }
}

pub(crate) fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut games_state: ResMut<GamesState>,
) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(
            0.0,
            -TILE_HEIGHT * ((GRID_SIZE as f32 / 3.0).round() + 1.0),
            100.0,
        ),
    ));

    for (i, (col, row)) in generate_positions().iter().enumerate() {
        let x = (col - row) * (TILE_WIDTH / 2.0);
        let y = -(col + row) * (TILE_HEIGHT / 2.0);
        games_state.board_entity.push(
            commands
                .spawn((
                    Sprite {
                        image: get_texture(&asset_server, i),
                        ..Default::default()
                    },
                    Transform::from_xyz(x, y, row + col),
                    Name::new(format!("Tile_{}", i)),
                ))
                .id(),
        );
    }

    commands.spawn((
        Text::new("Money:"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
        MoneyText,
    ));

    commands.spawn((
        Text::new("Name:"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        },
        NameText,
    ));
}

pub(crate) fn generate_positions() -> Vec<(f32, f32)> {
    let mut positions = Vec::new();
    for col in 0..GRID_SIZE {
        positions.push((col as f32, 0f32));
    }

    // Right column (top to bottom)
    for row in 1..GRID_SIZE {
        positions.push(((GRID_SIZE - 1) as f32, row as f32));
    }

    // Bottom row (right to left)
    for col in (0..GRID_SIZE - 1).rev() {
        positions.push((col as f32, (GRID_SIZE - 1) as f32));
    }

    // Left column (bottom to top)
    for row in (1..GRID_SIZE - 1).rev() {
        positions.push((0f32, row as f32));
    }
    positions
}

pub(crate) fn convert_pos_to_coords(pos: usize) -> (f32, f32) {
    let positions = generate_positions();
    let x = (positions[pos].0 - positions[pos].1) * (TILE_WIDTH / 2.0);
    let y = -(positions[pos].0 + positions[pos].1) * (TILE_HEIGHT / 2.0) + TILE_HEIGHT / 2.0;
    (x, y)
}

pub(crate) fn spawn_players(
    commands: &mut Commands,
    asset_server: &AssetServer,
    players_data: Vec<PlayerIdentifyData>,
    state: &mut GamesState,
) {
    for (i, data) in players_data.iter().enumerate() {
        println!("Spawning player {}", i);
        let player_texture = asset_server.load(SPRITES_PATH[i]);
        let pos = convert_pos_to_coords(0);
        let player_entity = commands
            .spawn((
                Sprite {
                    image: player_texture,
                    ..Default::default()
                },
                Transform::from_xyz(pos.0, pos.1, 32f32),
                Name::new(format!("Player_{}", i)),
            ))
            .id();
        state.players.insert(
            data.id,
            Player {
                name: data.name.clone(),
                money: 1500,
                position: 0,
                is_in_jail: false,
                entity: player_entity,
                player_number: i,
                is_bankrupt: false,
            },
        );
    }
}

pub(crate) fn roll_dice(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    sender: Res<MessageSender>,
    games_state: Res<GamesState>,
) {
    if keyboard_input.just_pressed(KeyCode::Space)
        && games_state.player_turn == games_state.id
        && games_state.can_roll
    {
        println!("Rolling dice");
        // Spawn a new async task to send the action
        let sender = sender.clone();
        let task_pool = AsyncComputeTaskPool::get();
        task_pool
            .spawn(async move {
                sender
                    .0
                    .send(PlayerAction {
                        action_type: Action::Roll,
                        data: None,
                    })
                    .await
                    .unwrap();
            })
            .detach();
    }
}

pub(crate) fn add_player_banner(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    state: &mut GamesState,
) {
    let player_texture = asset_server
        .load(SPRITES_PATH[state.players.get(&state.player_turn).unwrap().player_number]);
    commands
        .entity(state.board_entity[state.players.get(&state.player_turn).unwrap().position])
        .despawn_descendants();
    commands
        .entity(state.board_entity[state.players.get(&state.player_turn).unwrap().position])
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: player_texture,
                    ..Default::default()
                },
                Transform::from_xyz(0.0, 0.0, 50.0),
                Name::new(format!("Banner_{}", state.player_turn)),
            ));
        });
}
