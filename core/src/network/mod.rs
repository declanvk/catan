use uuid::Uuid;

/// `ServerRequest` is a type that represents incoming requests to the game server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerRequest {
    /// Ask for a new player to be created, will failed with `ServerErrorResponse::PlayerExists` if the player exists
    /// The supplied username will be truncated to 8 characters if longer.
    NewPlayer(String),
    /// List all the players that have registered with the server.
    /// The server will only store these values for the duration of the process.
    ListPlayers,
    /// List all the games that are present in archives, running, or waiting to start.
    /// The server will store records of games to disk, and report their presence on a new startup.
    ListGames,
    /// Select specific games by the their `GameStatus`
    SelectGames(GameStatus),
    /// let the server know that a specific Player is disconnecting
    DisconnectPlayer(Uuid),
}

/// `ServerResponse` is a type that represents the outgoing responses from the server to a player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerResponse {
    /// Returned after a player is successfully created
    /// The UUID will be needed for all future identification
    NewPlayer(Uuid),
    /// Returns a list of usernames registered to this server
    PlayerList(Vec<String>),
    /// Returns a list of games archives, running, or waiting on this server
    GameList(Vec<GameMetadata>),
    /// Encompasses all error statuses
    Error(ServerErrorResponse),
}

/// `ServerErrorResponse` is a type that represents specific error types and messages
/// Will be contained in `ServerResponse::Error` for transport
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerErrorResponse {
    /// This will be returned by the server if the transmitted playername is already taken
    PlayerExists,
    /// This will be returned if the request sent to the server is malformed or not recognized
    InvalidRequest,
    /// This will be returned for any other error, with an attempt made to describe the error in a string
    Other(String)
}

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
    Complete
}
