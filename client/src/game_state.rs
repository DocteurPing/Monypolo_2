use crate::board::{add_player_banner, convert_pos_to_coords, generate_positions, spawn_players};
use crate::ui::buttons::spawn_buy_buttons;
use crate::ui::toast::spawn_toast;
use bevy::prelude::*;
use bevy::utils::default;
use shared::action::{Action, BuyPropertyData, PlayerAction, PlayerIdentifyData};
use shared::board::Tile::{Property, Railroad};
use shared::maps::map1::MAP1;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct Player {
    pub(crate) name: String,
    pub(crate) money: u32,
    pub(crate) position: usize,
    pub(crate) is_in_jail: bool,
    pub(crate) entity: Entity,
    pub(crate) player_number: usize,
}

#[derive(Resource, Debug)]
pub(crate) struct GamesState {
    pub(crate) id: Uuid,
    pub(crate) players: HashMap<Uuid, Player>,
    pub(crate) current_turn: usize,
    pub(crate) player_turn: Uuid,
    pub(crate) board: Vec<shared::board::Tile>,
    pub(crate) board_entity: Vec<Entity>,
    pub(crate) can_roll: bool,
    pub(crate) buy_button_node_id: Option<Entity>,
}

impl Default for GamesState {
    fn default() -> Self {
        GamesState {
            id: Uuid::new_v4(),
            players: HashMap::new(),
            current_turn: 0,
            player_turn: Uuid::new_v4(),
            board: MAP1.clone(),
            board_entity: vec![],
            can_roll: false,
            buy_button_node_id: None,
        }
    }
}

pub(crate) fn handle_message_in_game(
    message: &str,
    state: &mut GamesState,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut transforms: Query<&mut Transform>,
) {
    let action: PlayerAction = serde_json::from_str(message).unwrap();
    match action.action_type {
        Action::GameStart => {
            start_game(state, action, commands, asset_server);
        }
        Action::PlayerTurn => {
            state.can_roll = true;
            state.player_turn = action.data.unwrap().parse::<Uuid>().unwrap();
            println!("Player {} turn", state.player_turn);
        }
        Action::Identify => {
            state.id = action.data.unwrap().parse::<Uuid>().unwrap();
            println!("Player identified with ID: {}", state.id);
        }
        Action::Move => {
            move_player(
                state,
                &mut transforms,
                action.data.unwrap().parse::<usize>().unwrap(),
            );
        }
        Action::PayRent => {
            let data: shared::action::PayRentData =
                serde_json::from_str(&action.data.unwrap()).unwrap();
            let rent_price = data.rent;
            state.players.get_mut(&data.player).unwrap().money -= rent_price;
            state.players.get_mut(&data.owner).unwrap().money += rent_price;
            println!(
                "Player {} paid rent of {} to Player {}",
                data.player, rent_price, data.owner
            );
        }
        Action::AskBuyProperty => {
            let buy_property_data: shared::action::BuyPropertyData =
                serde_json::from_str(&action.data.unwrap()).unwrap();
            println!(
                "Player {} asked to buy property {}",
                buy_property_data.player, buy_property_data.position
            );
            if buy_property_data.player == state.id {
                spawn_buy_buttons(commands, state);
            }
        }
        Action::BuyProperty => {
            buy_property(state, commands, action, asset_server);
        }
        Action::GoToJail => {
            let jail_pos = state
                .board
                .iter()
                .position(|tile| matches!(tile, shared::board::Tile::Jail))
                .unwrap();
            move_player(state, &mut transforms, jail_pos);
            state
                .players
                .get_mut(&state.player_turn)
                .unwrap()
                .is_in_jail = true;
            println!("Player {} is in jail", state.player_turn);
        }
        Action::PlayerGoTile => {
            let data =
                serde_json::from_str::<shared::action::PlayerGoTileData>(&action.data.unwrap())
                    .unwrap();
            let player = state.players.get_mut(&data.player).unwrap();
            player.money += data.amount;
            println!("Player {} got {} money", data.player, data.amount);
            println!(
                "Player {} money: {}",
                data.player,
                state.players.get_mut(&data.player).unwrap().money
            );
        }
        Action::Roll => {
            let data = serde_json::from_str::<shared::action::DiceRollData>(&action.data.unwrap())
                .unwrap();
            println!("Player rolled {} and {}", data.dice1, data.dice2);
        }
        Action::SkipBuyProperty => {
            println!("Player skipped buying property");
            spawn_toast(
                commands,
                format!(
                    "{} skipped buying the property!",
                    state.players.get(&state.player_turn).unwrap().name
                ),
                2.0,
            );
        }
        _ => {}
    }
}

fn buy_property(
    state: &mut GamesState,
    commands: &mut Commands,
    action: PlayerAction,
    asset_server: &Res<AssetServer>,
) {
    let buy_property_data: BuyPropertyData =
        serde_json::from_str(action.data.unwrap().as_str()).unwrap();
    let player = state.players.get_mut(&buy_property_data.player).unwrap();
    let mut tile_owner: &mut Option<Uuid> = &mut None;
    let mut tile_cost = 0;
    // Get the property tile
    if let Some((owner, cost)) = match &mut state.board[player.position] {
        Property { owner, cost, .. } => Some((owner, cost[0])),
        Railroad { owner, cost, .. } => Some((owner, *cost)),
        _ => None,
    } {
        tile_owner = owner;
        tile_cost = cost;
    }
    player.money -= tile_cost;
    *tile_owner = Some(buy_property_data.player);
    println!(
        "Player {} bought property {}",
        buy_property_data.player, buy_property_data.position
    );
    println!("Property is now {:?}", state.board[player.position]);
    spawn_toast(
        commands,
        format!(
            "{} bought the property!",
            state.players.get(&state.player_turn).unwrap().name
        ),
        2.0,
    );
    add_player_banner(commands, asset_server, state);
}

fn move_player(state: &mut GamesState, transforms: &mut Query<&mut Transform>, roll: usize) {
    println!("uuid: {:?}", state.player_turn);
    state.players.get_mut(&state.player_turn).unwrap().position = roll;
    println!(
        "Player moved to position {} tile {:?}",
        state.players.get(&state.player_turn).unwrap().position,
        state.board[state.players.get(&state.player_turn).unwrap().position]
    );
    // Get player and move it
    let position = generate_positions();
    println!("Position x: {:?}", position[roll].0);
    println!("Position y: {:?}", position[roll].1);
    let pos = convert_pos_to_coords(roll);
    *transforms
        .get_mut(state.players.get(&state.player_turn).unwrap().entity)
        .unwrap() = Transform {
        translation: Vec3::new(pos.0, pos.1, 32f32),
        ..default()
    };
    state.can_roll = false;
}

fn start_game(
    state: &mut GamesState,
    action: PlayerAction,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    let data = action.data.unwrap();
    let players_data = serde_json::from_str::<Vec<PlayerIdentifyData>>(&data).unwrap();
    println!("Game started with {} players", players_data.len());
    println!("Players ID: {:?}", players_data);

    // Add a player for number of player stored in data
    for data in players_data.clone() {
        println!("Player {} joined the game", data.id);
    }
    spawn_players(commands, asset_server, players_data, state);
}
