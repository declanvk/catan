use std::cell::RefCell;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use futures::prelude::*;
use futures::future;

use tokio_core::reactor::Handle;
use tokio_service::Service;

use redis_async::resp::RespValue;
use redis_async::client::PairedConnection;

use server_common::error::{ServerError, ServerResult};
use server_common::resp_helper::resp_value_as_bulk_contents;
use server_common::uuid_generators::{UuidV1Generator, generate_game_uuid};
use server_common::resource_naming::service_key;
use server_common::redis_scripts::load_scripts;

use catan_protocols::services::game_management::game_management_request::RequestType;
use catan_protocols::services::game_management::{GameManagementRequest, RegisterNewGame,
                                                 StartGame, EndGame, CleanupGame};
use prost::Message;
use bytes::IntoBuf;
use uuid::UuidV1Context;

const SERVICE_NAME: &'static str = "game-management";

const ALL_GAMES_SET: &'static str = "collection:games:all";
const GAME_INITIAL_STATE_SET: &'static str = "collection:games:initialization";
const GAME_STARTED_SET: &'static str = "collection:games:started";
const GAME_ENDED_SET: &'static str = "collection:games:ended";

const GAME_NAME_INDEX_KEY: &'static str = "index:game-name";
const GAME_OPEN_SPOTS_RANKING: &'static str = "index:game-open-spots";
const GAME_TIME_ADDED_RANKING: &'static str = "index:game-time-added";

pub fn deserialize_request(message: RespValue) -> ServerResult<GameManagementRequest> {
    trace!("Raw message: {:?}", message);

    let input_buffer = resp_value_as_bulk_contents(message)?.into_buf();

    Ok(GameManagementRequest::decode(input_buffer)?)
}

pub struct GameManagementService {
    pub redis_address: SocketAddr,
    pub script_folder: PathBuf,
    pub redis_scripts: HashMap<String, String>,
    pub uuid_generator: RefCell<UuidV1Generator>,
    pub handle: Handle,
    pub service_topic: String,
    pub connection: Rc<PairedConnection>,
}

impl GameManagementService {
    pub fn new(
        address: SocketAddr,
        script_folder: PathBuf,
        handle: Handle,
        uuid_context: Arc<UuidV1Context>,
        paired_connection: PairedConnection,
    ) -> ServerResult<Self> {
        let scripts = load_scripts(&script_folder, &address)?;
        let uuid_generator = UuidV1Generator::new(&SERVICE_NAME.to_owned(), uuid_context);
        let connection = Rc::new(paired_connection);

        let service = GameManagementService {
            redis_address: address,
            script_folder: script_folder,
            redis_scripts: scripts,
            uuid_generator: RefCell::new(uuid_generator),
            handle: handle,
            service_topic: service_key(SERVICE_NAME),
            connection: connection,
        };

        Ok(service)
    }

    fn register_new_game<'req>(&'req self, game_name: &'req str) -> RegisterNewGameService<'req> {
        RegisterNewGameService {
            game_name: game_name,
            handle: &self.handle,
            connection: Rc::clone(&self.connection),
        }
    }

    fn start_game<'req>(&'req self, game_name: &'req str) -> StartGameService<'req> {
        StartGameService {
            game_name,
            connection: Rc::clone(&self.connection),
        }
    }

    fn end_game<'req>(&'req self, game_name: &'req str) -> EndGameService<'req> {
        EndGameService {
            game_name,
            connection: Rc::clone(&self.connection),
        }
    }

    fn cleanup_game<'req>(&'req self, game_name: &'req str) -> CleanupGameService<'req> {
        CleanupGameService {
            game_name,
            connection: Rc::clone(&self.connection),
        }
    }

    pub fn service_topic() -> String {
        service_key(SERVICE_NAME)
    }
}

impl Service for GameManagementService {
    type Request = GameManagementRequest;
    type Response = ();
    type Error = ServerError;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        let result = if let Some(request_type) = request.request_type {
            match request_type {
                RequestType::RegisterNewGame(options) => {
                    self.register_new_game(request.game_name.as_ref()).call(
                        options,
                    )
                }
                RequestType::StartGame(options) => {
                    self.start_game(request.game_name.as_ref()).call(options)
                }
                RequestType::EndGame(options) => {
                    self.end_game(request.game_name.as_ref()).call(options)
                }
                RequestType::CleanupGame(options) => {
                    self.cleanup_game(request.game_name.as_ref()).call(options)
                }
            }
        } else {
            Box::new(
                Err(ServerError::Custom(
                    "Missing message request_type".to_owned(),
                )).into_future(),
            )
        };

        result
    }
}

struct RegisterNewGameService<'req> {
    game_name: &'req str,
    handle: &'req Handle,
    connection: Rc<PairedConnection>,
}

impl<'req> Service for RegisterNewGameService<'req> {
    type Request = RegisterNewGame;
    type Response = ();
    type Error = ServerError;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        info!(
            "Register new game request: (name: {}, options: {:?})",
            self.game_name,
            request
        );

        let uuid = generate_game_uuid(self.game_name);
        trace!("Generated uuid: ({})", uuid.hyphenated());

        let set_main_hash = self.connection.send::<String>(resp_array![
            "HMSET",
            format!("game:{}", uuid.hyphenated()),
            "id",
            Vec::from(&uuid.as_bytes()[..]),
            "game_name",
            self.game_name,
            "num_players",
            format!("{}", request.num_players),
            "turn_timeout_ms",
            format!("{}", request.turn_timeout_ms)
        ]);

        let add_to_status_set = self.connection.send::<String>(resp_array![
            "SADD",
            GAME_INITIAL_STATE_SET,
            format!("{}", uuid.hyphenated())
        ]);

        let add_to_all_games_set = self.connection.send::<String>(resp_array![
            "SADD",
            ALL_GAMES_SET,
            format!("{}", uuid.hyphenated())
        ]);


        let add_index_entry = self.connection.send::<String>(resp_array![
            "ZADD",
            GAME_NAME_INDEX_KEY,
            "0",
            format!(
                "{}:{}",
                self.game_name,
                uuid.hyphenated()
            )
        ]);

        let actions = vec![
            set_main_hash,
            add_to_status_set,
            add_to_all_games_set,
            add_index_entry,
        ];

        let complete_output = future::join_all(actions)
            .and_then(move |return_values| {
                trace!("Returned: {:?}", return_values);

                if return_values[1] != "1" {
                    warn!("Game already in pre-registration set");
                }

                if return_values[2] != "1" {
                    warn!("Game already in all games set");

                }

                if return_values[3] != "1" {
                    warn!("Game already in public listing");
                }

                Ok(()).into_future()
            })
            .map_err(From::from);

        Box::new(complete_output)
    }
}

struct StartGameService<'req> {
    game_name: &'req str,
    connection: Rc<PairedConnection>,
}

impl<'req> Service for StartGameService<'req> {
    type Request = StartGame;
    type Response = ();
    type Error = ServerError;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        info!("Start game request: (name: {})", self.game_name);
        Box::new(Ok(()).into_future())
    }
}

struct EndGameService<'req> {
    game_name: &'req str,
    connection: Rc<PairedConnection>,
}

impl<'req> Service for EndGameService<'req> {
    type Request = EndGame;
    type Response = ();
    type Error = ServerError;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        info!("End game request: (name: {})", self.game_name);
        Box::new(Ok(()).into_future())
    }
}

struct CleanupGameService<'req> {
    game_name: &'req str,
    connection: Rc<PairedConnection>,
}

impl<'req> Service for CleanupGameService<'req> {
    type Request = CleanupGame;
    type Response = ();
    type Error = ServerError;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Self::Request) -> Self::Future {
        info!("Cleanup game request: (name: {})", self.game_name);
        Box::new(Ok(()).into_future())
    }
}
