use uuid::Uuid;
use tokio_service::Service;
use futures::prelude::*;
use futures::future;

use super::error::ServerError;
use super::server::{GameMetadata, GameStatus};

/// `ServerRequest` is a type that represents incoming requests to the game server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerRequest {
    /// Ask for a new player to be created, will failed with `ServerErrorResponse::PlayerExists` if the player exists
    /// The supplied username will be truncated to 8 characters if longer.
    NewPlayer { username: String },
    /// List all the players that have registered with the server.
    /// The server will only store these values for the duration of the process.
    ListPlayers,
    /// List all the games that are present in archives, running, or waiting to start.
    /// The server will store records of games to disk, and report their presence on a new startup.
    /// The filter will specify the type of games to return.
    ListGames { status_filter: GameStatus },
    /// let the server know that a specific Player is disconnecting
    DisconnectPlayer { player_id: Uuid },
}

/// `ServerResponse` is a type that represents the outgoing responses from the server to a player
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerResponse {
    /// Returned after a player is successfully created
    /// The UUID will be needed for all future identification
    NewPlayer { player_id: Uuid },
    /// Returns a list of usernames registered to this server
    PlayerList { players: Vec<String> },
    /// Returns a list of games archives, running, or waiting on this server
    GameList { games: Vec<GameMetadata> },
    /// Encompasses all error statuses
    Error { inner: ServerErrorResponse },
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

pub struct NewPlayerService;

impl Service for NewPlayerService {
    type Request = String;
    type Response = ServerResponse;
    type Error = ServerError;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
    fn call(&self, req: Self::Request) -> Self::Future {
        let content = ServerResponse::NewPlayer {
            player_id: Uuid::new_v4()
        };
        
        Box::new(future::ok::<ServerResponse, ServerError>(content))
    }
}
