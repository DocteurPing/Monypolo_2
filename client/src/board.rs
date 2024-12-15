use bevy::asset::Handle;
use bevy::image::Image;
use bevy::prelude::{AssetServer, Camera2d, Commands, Res, Sprite, Transform};

const TILE_WIDTH: f32 = 110.0; // Width of an isometric tile
const TILE_HEIGHT: f32 = 63.0; // Height of an isometric tile
const GRID_SIZE: usize = 11; // Number of tiles along one edge (must be odd)

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Load the texture for tiles
    let grass_texture = asset_server.load("textures/voxelTile_55.png");
    let test = asset_server.load("textures/platformerTile_36.png");

    for (i, (col, row)) in generate_positions().iter().enumerate() {
        let x = (col - row) * (TILE_WIDTH / 2.0);
        let y = -(col + row) * (TILE_HEIGHT / 2.0);
        //let texture = if i == 33 { grass_texture.clone() } else { test.clone() };
        spawn_tile(&mut commands, grass_texture.clone(), test.clone(), i as i32, *col, *row, x, y);
    }
}

fn generate_positions() -> Vec<(f32, f32)> {
    let mut positions = Vec::new();
    for i in 0..GRID_SIZE-1 {
        positions.push(((GRID_SIZE - 1 - i) as f32, (GRID_SIZE - 1) as f32));
        positions.push((0f32, (GRID_SIZE - 1 - i) as f32));
        positions.push((i as f32, 0f32));
        positions.push(((GRID_SIZE - 1) as f32, i as f32));
    }
    positions
}

fn spawn_tile(commands: &mut Commands, grass_texture: Handle<Image>, test: Handle<Image>, mut pos: i32, col: f32, row: f32, x: f32, y: f32) {
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
            row as f32 + col as f32,
        ),
    ));
}
