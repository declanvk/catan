use uuid::Uuid;
use std::collections::HashMap;
use std::path::PathBuf;
use catan_core::network::*;

/// `PlayerStub` is a type that represents player registered on the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerStub {
    /// Unique username of player, maybe be 8 characters or less
    username: String,
    /// Unique identification value
    id: Uuid,
}

#[derive(Debug)]
pub struct ServerInternalData {
    games_metadata: HashMap<Uuid, GameMetadata>,
    active_games: HashMap<Uuid, ActiveGameContainer>,
    complete_games: HashMap<Uuid, ArchivedGame>,
    all_players: HashMap<Uuid, PlayerStub>,
}

impl Default for ServerInternalData {
    fn default() -> Self {
        ServerInternalData {
            games_metadata: HashMap::new(),
            active_games: HashMap::new(),
            complete_games: HashMap::new(),
            all_players: HashMap::new()
        }
    }
}

#[derive(Debug)]
pub struct ArchivedGame {
    log_file: PathBuf,
    players: Vec<Uuid>,
}

#[derive(Debug)]
pub struct ActiveGameContainer {
    players: Vec<Uuid>,
    game: (),
}

pub const MAX_PLAYERS: usize = 4;

impl ActiveGameContainer {
    pub fn new(players: Vec<Uuid>, name: String) -> (ActiveGameContainer, GameMetadata) {
        let game_id = Uuid::new_v4();
        let metadata = GameMetadata {
            status: GameStatus::Open,
            id: game_id,
            name,
        };
        let container = ActiveGameContainer { players, game: () };

        (container, metadata)
    }

    pub fn is_full(&self) -> bool {
        self.players.len() == MAX_PLAYERS
    }

    pub fn add_player(&self, player_id: Uuid) -> bool {
        false
    }
}
