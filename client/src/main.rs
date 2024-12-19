mod board;
mod communication;
mod game_state;
mod helpers;
mod tools;

use crate::communication::{setup_network, MessageReceiver, MessageSender};
use crate::game_state::GamesState;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shared::maps::map1::MAP1;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let (rx_server, tx_client) = setup_network().await;
    // Initialize Bevy app
    App::new()
        .add_plugins(DefaultPlugins) // Add default Bevy plugins
        .insert_resource(GamesState {
            id: Default::default(),
            players: HashMap::new(),
            current_turn: 0,
            player_turn: Default::default(),
            board: MAP1.clone(),
        })
        .insert_resource(MessageReceiver(rx_server))
        .insert_resource(MessageSender(tx_client))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, board::setup)
        .add_systems(Update, communication::receive_message)
        .add_systems(Update, board::roll_dice)
        .add_systems(Update, helpers::camera::movement)
        .run();
}
