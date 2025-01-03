use crate::screens::board::{
    add_player_banner, convert_pos_to_coords, generate_positions, spawn_players,
};
use crate::ui::buttons::spawn_buy_buttons;
use crate::ui::toast::{spawn_toast, ToastCount};
use bevy::prelude::*;
use bevy::utils::default;
use shared::action::{Action, BuyPropertyData, PlayerAction, PlayerIdentifyData};
use shared::board::Tile::{Property, Railroad, Utility};
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
    pub(crate) is_bankrupt: bool,
}

#[derive(Resource, Debug)]
pub(crate) struct GamesState {
    pub(crate) id: Uuid,
    pub(crate) players: HashMap<Uuid, Player>,
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
    toast_count: ResMut<ToastCount>,
) {
    let action: PlayerAction = serde_json::from_str(message).unwrap();
    match action.action_type {
        Action::GameStart => {
            start_game(state, action, commands, asset_server, toast_count);
        }
        Action::PlayerTurn => {
            start_player_turn(state, commands, toast_count, action);
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
            pay_rent(state, commands, toast_count, action);
        }
        Action::AskBuyProperty => {
            ask_buy_property(state, commands, action);
        }
        Action::BuyProperty => {
            buy_property(state, commands, action, asset_server, toast_count);
        }
        Action::GoToJail => {
            send_to_jail(state, commands, &mut transforms, toast_count);
        }
        Action::PlayerGoTile => {
            move_to_go_tile(state, commands, toast_count, action);
        }
        Action::Roll => {
            show_roll_data(state, commands, toast_count, action);
        }
        Action::SkipBuyProperty => {
            skip_buy_property(state, commands, toast_count);
        }
        Action::PayTax => {
            pay_tax(state, commands, toast_count, action);
        }
        Action::PlayerBankrupt => {
            set_player_bankrupt(state, commands, toast_count, action);
        }
        Action::GameOver => {
            end_game(state, commands, toast_count, action);
        }
        _ => {}
    }
}

fn skip_buy_property(
    state: &mut GamesState,
    commands: &mut Commands,
    toast_count: ResMut<ToastCount>,
) {
    println!("Player skipped buying property");
    spawn_toast(
        commands,
        format!(
            "{} skipped buying the property!",
            state.players.get(&state.player_turn).unwrap().name
        ),
        2.0,
        toast_count,
    );
}

fn pay_tax(
    state: &mut GamesState,
    commands: &mut Commands,
    toast_count: ResMut<ToastCount>,
    action: PlayerAction,
) {
    let data =
        serde_json::from_str::<shared::action::PlayerPayTaxData>(&action.data.unwrap()).unwrap();
    let player = state.players.get_mut(&data.player).unwrap();
    player.money -= data.amount;
    println!("Player {} paid {} tax", data.player, data.amount);
    spawn_toast(
        commands,
        format!(
            "Player {} paid {} tax",
            state.players.get(&state.player_turn).unwrap().name,
            data.amount
        ),
        2.0,
        toast_count,
    );
}

fn show_roll_data(
    state: &mut GamesState,
    commands: &mut Commands,
    toast_count: ResMut<ToastCount>,
    action: PlayerAction,
) {
    let data = serde_json::from_str::<shared::action::DiceRollData>(&action.data.unwrap()).unwrap();
    println!("Player rolled {} and {}", data.dice1, data.dice2);
    spawn_toast(
        commands,
        format!(
            "{} rolled {} and {}",
            state.players.get(&state.player_turn).unwrap().name,
            data.dice1,
            data.dice2
        ),
        2.0,
        toast_count,
    );
}

fn move_to_go_tile(
    state: &mut GamesState,
    commands: &mut Commands,
    toast_count: ResMut<ToastCount>,
    action: PlayerAction,
) {
    let data =
        serde_json::from_str::<shared::action::PlayerGoTileData>(&action.data.unwrap()).unwrap();
    let player = state.players.get_mut(&data.player).unwrap();
    player.money += data.amount;
    println!("Player {} got {} money", data.player, data.amount);
    println!(
        "Player {} money: {}",
        data.player,
        state.players.get_mut(&data.player).unwrap().money
    );
    spawn_toast(
        commands,
        format!(
            "{} got {} money!",
            state.players.get(&state.player_turn).unwrap().name,
            data.amount
        ),
        2.0,
        toast_count,
    );
}

fn send_to_jail(
    state: &mut GamesState,
    commands: &mut Commands,
    transforms: &mut Query<&mut Transform>,
    toast_count: ResMut<ToastCount>,
) {
    let jail_pos = state
        .board
        .iter()
        .position(|tile| matches!(tile, shared::board::Tile::Jail))
        .unwrap();
    move_player(state, transforms, jail_pos);
    state
        .players
        .get_mut(&state.player_turn)
        .unwrap()
        .is_in_jail = true;
    println!("Player {} is in jail", state.player_turn);
    spawn_toast(
        commands,
        format!(
            "{} is going to jail!",
            state.players.get(&state.player_turn).unwrap().name
        ),
        2.0,
        toast_count,
    );
}

fn ask_buy_property(state: &mut GamesState, commands: &mut Commands, action: PlayerAction) {
    let buy_property_data: BuyPropertyData = serde_json::from_str(&action.data.unwrap()).unwrap();
    println!(
        "Player {} asked to buy property {}",
        buy_property_data.player, buy_property_data.position
    );
    if buy_property_data.player == state.id {
        spawn_buy_buttons(commands, state);
    }
}

fn pay_rent(
    state: &mut GamesState,
    commands: &mut Commands,
    toast_count: ResMut<ToastCount>,
    action: PlayerAction,
) {
    let data: shared::action::PayRentData = serde_json::from_str(&action.data.unwrap()).unwrap();
    let rent_price = data.rent;
    state.players.get_mut(&data.player).unwrap().money -= rent_price;
    state.players.get_mut(&data.owner).unwrap().money += rent_price;
    println!(
        "Player {} paid rent of {} to Player {}",
        data.player, rent_price, data.owner
    );
    spawn_toast(
        commands,
        format!(
            "{} paid rent of {} to {}",
            state.players.get(&state.player_turn).unwrap().name,
            rent_price,
            state.players.get(&data.owner).unwrap().name
        ),
        2.0,
        toast_count,
    );
}

fn set_player_bankrupt(
    state: &mut GamesState,
    commands: &mut Commands,
    toast_count: ResMut<ToastCount>,
    action: PlayerAction,
) {
    let data = action.data.unwrap().parse::<Uuid>().unwrap();
    let player = state.players.get_mut(&data).unwrap();
    player.is_bankrupt = true;
    player.money = 0;
    println!("Player {} is bankrupt", player.name);
    spawn_toast(
        commands,
        format!("{} is bankrupt!", player.name),
        2.0,
        toast_count,
    );
}

fn end_game(
    state: &mut GamesState,
    commands: &mut Commands,
    toast_count: ResMut<ToastCount>,
    action: PlayerAction,
) {
    let winner = state
        .players
        .get(&action.data.unwrap().parse::<Uuid>().unwrap())
        .unwrap();
    println!("Game over!");
    spawn_toast(
        commands,
        format!("Game over! The winner is {}", winner.name),
        20.0,
        toast_count,
    );
}

fn start_player_turn(
    state: &mut GamesState,
    commands: &mut Commands,
    toast_count: ResMut<ToastCount>,
    action: PlayerAction,
) {
    state.can_roll = true;
    state.player_turn = action.data.unwrap().parse::<Uuid>().unwrap();
    println!("Player {} turn", state.player_turn);
    spawn_toast(
        commands,
        format!(
            "{} turns!",
            state.players.get(&state.player_turn).unwrap().name
        ),
        2.0,
        toast_count,
    );
}

fn buy_property(
    state: &mut GamesState,
    commands: &mut Commands,
    action: PlayerAction,
    asset_server: &Res<AssetServer>,
    toast_count: ResMut<ToastCount>,
) {
    let buy_property_data: BuyPropertyData =
        serde_json::from_str(action.data.unwrap().as_str()).unwrap();
    let player = state.players.get_mut(&buy_property_data.player).unwrap();
    let mut tile_owner: &mut Option<Uuid> = &mut None;
    let mut tile_cost = 0;
    // Get the property tile
    if let Some((owner, cost)) = match &mut state.board[player.position] {
        Property { owner, costs, .. } => Some((owner, costs[0])),
        Railroad { owner, cost, .. } => Some((owner, *cost)),
        Utility { owner, cost, .. } => Some((owner, *cost)),
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
        toast_count,
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
    toast_count: ResMut<ToastCount>,
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
    spawn_toast(commands, "Game started!".to_string(), 2.0, toast_count);
}
