use crate::communication::send_to_all_players;
use crate::server_state::ServerState;
use shared::action::Action;
use shared::action::PlayerIdentifyData;
use shared::list_const::NUMBER_PLAYERS_PER_GAME;
use shared::maps::map1::MAP1;
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
    pub(crate) is_bankrupt: bool,
}

impl Player {
    pub(crate) fn default(tx: mpsc::Sender<String>, name: String) -> Self {
        Player {
            id: Uuid::new_v4(),
            name,
            tx,
            money: 1500,
            position: 0,
            is_in_jail: false,
            jail_turns: 0,
            is_bankrupt: false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct WaitingRoom {
    pub(crate) players: Vec<Player>,
}

#[derive(Debug, Clone)]
pub(crate) struct Game {
    pub(crate) id: Uuid,
    pub(crate) players: Vec<Player>,
    pub(crate) board: Vec<shared::board::Tile>,
    pub(crate) current_turn: usize,
    pub(crate) player_turn: usize,
    pub(crate) is_active: bool,
}

impl Game {
    pub(crate) async fn advance_turn(&mut self) {
        // Check if more than one player is not bankrupt
        let number_players_left = self.players.iter().filter(|p| !p.is_bankrupt).count();
        if number_players_left == 1 {
            let winner = self.players.iter().find(|p| !p.is_bankrupt).unwrap();
            send_to_all_players(&self.players, Action::GameOver, Some(winner.id.to_string())).await;
            self.is_active = false;
            return;
        }
        self.current_turn += 1;
        self.player_turn = (self.player_turn + 1) % self.players.len();
        while self.players[self.player_turn].is_bankrupt {
            self.player_turn = (self.player_turn + 1) % self.players.len();
        }
        send_to_all_players(
            &self.players,
            Action::PlayerTurn,
            Some(self.players[self.player_turn].id.to_string()),
        )
        .await;
    }

    pub(crate) fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            players: vec![],
            board: MAP1.clone(),
            current_turn: 0,
            player_turn: 0,
            is_active: true,
        }
    }
}

pub(crate) async fn start_new_game(state: Arc<ServerState>) {
    let mut waiting_room = state.waiting_room.lock().await;
    let mut active_games = state.active_games.lock().await;

    if waiting_room.players.len() < NUMBER_PLAYERS_PER_GAME {
        return;
    }

    let players = waiting_room
        .players
        .drain(0..NUMBER_PLAYERS_PER_GAME)
        .collect::<Vec<_>>();

    let mut game = Game::default();
    let game_id = game.id;
    game.players = players.clone();

    active_games.insert(game_id, game);
    log::debug!("Started a new game with ID: {game_id}");

    let current_game = active_games.get_mut(&game_id).unwrap();
    current_game.player_turn = rand::random::<u8>() as usize % (players.len() - 1);
    let players_data: Vec<PlayerIdentifyData> = players
        .iter()
        .map(|p| PlayerIdentifyData {
            id: p.id,
            name: p.name.clone(),
        })
        .collect();
    send_to_all_players(
        &players,
        Action::GameStart,
        Some(serde_json::to_string(&players_data).unwrap()),
    )
    .await;
    send_to_all_players(
        &players,
        Action::PlayerTurn,
        Some(players[current_game.player_turn].id.to_string()),
    )
    .await;
}
