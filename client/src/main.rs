mod communication;
mod game_state;
mod helpers;
mod screens;
mod tools;
mod ui;

use crate::communication::{setup_network, MessageReceiver, MessageSender};
use crate::game_state::GamesState;
use crate::screens::{add_camera, GameStateEnum};
use crate::ui::toast::ToastCount;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[tokio::main]
async fn main() {
    let (rx_server, tx_client) = setup_network().await;
    // Initialize Bevy app
    App::new()
        .add_plugins(DefaultPlugins) // Add default Bevy plugins
        .init_state::<GameStateEnum>()
        .insert_resource(GamesState::default())
        .insert_resource(MessageReceiver(rx_server))
        .insert_resource(MessageSender(tx_client))
        .insert_resource(ToastCount(0))
        .add_systems(Startup, add_camera)
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .add_plugins(screens::menu::menu_plugin)
        .add_plugins(screens::board::game_plugin)
        .run();
}
