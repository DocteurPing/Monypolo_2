use crate::communication::send_to_all_players;
use crate::server_state::ServerState;
use shared::maps::map_jail::MAP_JAIL;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Player {
    pub(crate) id: Uuid,
    pub(crate) name: String,
    pub(crate) tx: mpsc::Sender<String>, // Channel to communicate with the player
    pub(crate) money: u32,
    pub(crate) position: usize,
    pub(crate) is_in_jail: bool,
    pub(crate) jail_turns: u8,
}

#[derive(Debug)]
pub(crate) struct WaitingRoom {
    pub(crate) players: Vec<Player>,
}

#[derive(Debug)]
pub(crate) struct Game {
    pub(crate) id: Uuid,
    pub(crate) players: Vec<Player>,
    pub(crate) board: Vec<shared::board::Tile>,
    pub(crate) current_turn: usize,
    pub(crate) player_turn: usize,
}

impl Game {
    pub(crate) async fn advance_turn(&mut self) {
        self.current_turn += 1;
        self.player_turn = (self.player_turn + 1) % self.players.len();
        send_to_all_players(
            &self.players,
            shared::action::Action::PlayerTurn,
            Some(self.players[self.player_turn].id.to_string()),
        )
        .await;
    }
}

pub(crate) async fn start_new_game(state: Arc<ServerState>) {
    let mut waiting_room = state.waiting_room.lock().await;
    let mut active_games = state.active_games.lock().await;

    if waiting_room.players.len() < 1 {
        return;
    }

    let players = waiting_room.players.drain(0..2).collect::<Vec<_>>();
    let game_id = Uuid::new_v4();

    let game = Game {
        id: game_id,
        players: players.clone(),
        board: MAP_JAIL.clone(),
        current_turn: 0,
        player_turn: 0,
    };

    active_games.insert(game_id, game);
    println!("Started a new game with ID: {}", game_id);

    let current_game = active_games.get_mut(&game_id).unwrap();
    current_game.player_turn = rand::random::<u8>() as usize % (players.len() - 1);
    let player_ids = players
        .iter()
        .map(|p| p.id.to_string())
        .collect::<Vec<_>>()
        .join(",");
    send_to_all_players(
        &players,
        shared::action::Action::GameStart,
        Some(player_ids),
    )
    .await;
    send_to_all_players(
        &players,
        shared::action::Action::PlayerTurn,
        Some(players[current_game.player_turn].id.to_string()),
    )
    .await;
}
