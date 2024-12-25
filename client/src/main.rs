mod board;
mod communication;
mod game_state;
mod helpers;
mod tools;
mod ui;

use crate::communication::{setup_network, MessageReceiver, MessageSender};
use crate::game_state::GamesState;
use crate::ui::buttons::button_system;
use crate::ui::toast::ToastCount;
use crate::ui::{money, toast};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[tokio::main]
async fn main() {
    let (rx_server, tx_client) = setup_network().await;
    // Initialize Bevy app
    App::new()
        .add_plugins(DefaultPlugins) // Add default Bevy plugins
        .insert_resource(GamesState::default())
        .insert_resource(MessageReceiver(rx_server))
        .insert_resource(MessageSender(tx_client))
        .insert_resource(ToastCount(0))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_systems(Startup, board::setup)
        .add_systems(Update, communication::receive_message)
        .add_systems(Update, board::roll_dice)
        .add_systems(Update, button_system)
        .add_systems(Update, helpers::camera::movement)
        .add_systems(Update, money::scoreboard_system)
        .add_systems(Update, toast::update_toasts)
        .run();
}
