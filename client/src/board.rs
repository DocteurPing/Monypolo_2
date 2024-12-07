use bevy::prelude::{AssetServer, Camera2d, Commands, Res, Sprite, Transform};

const TILE_WIDTH: f32 = 110.0; // Width of an isometric tile
const TILE_HEIGHT: f32 = 63.0; // Height of an isometric tile
const GRID_SIZE: usize = 11; // Number of tiles along one edge (must be odd)

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    // Load the texture for tiles
    let grass_texture = asset_server.load("textures/voxelTile_55.png");
    let test = asset_server.load("textures/platformerTile_36.png");
    let mut pos = 0;

    // Loop through the grid and spawn tiles only along the edges
    for row in 0..GRID_SIZE {
        for col in 0..GRID_SIZE {
            // Check if the current tile is on the edge
            let is_edge = row == 0 || row == GRID_SIZE - 1 || col == 0 || col == GRID_SIZE - 1;

            if is_edge {
                // Calculate the isometric position
                let x = (col as f32 - row as f32) * (TILE_WIDTH / 2.0);
                let y = -(col as f32 + row as f32) * (TILE_HEIGHT / 2.0);

                commands.spawn((
                    Sprite {
                        image: if pos == 12 {
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
                pos += 1;
            }
        }
    }
}
