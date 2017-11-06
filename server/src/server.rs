use uuid::Uuid;
use std::collections::HashMap;
use std::path::PathBuf;

/// `GameMetadata` is information about a game stored on the server, that is not used in the actual gameplay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMetadata {
    /// Status of the game, which will change over time.
    pub status: GameStatus,
    /// Unique indentification value
    pub id: Uuid,
    /// Unique shorthand name for a game
    pub name: String,
}

/// `GameStatus` is a type that represents the status of a game stored on the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameStatus {
    /// The game has not started yet, it is missing the required number of players,
    /// or some other condition has not been met.
    Open,
    /// The game is in progress
    Running,
    /// The game has completed with a decided results
    Complete,
}


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
            all_players: HashMap::new(),
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
        self.players.len() >= MAX_PLAYERS
    }

    pub fn add_player(&self, player_id: Uuid) -> bool {
        if self.is_full() { false } else { false }
    }
}
