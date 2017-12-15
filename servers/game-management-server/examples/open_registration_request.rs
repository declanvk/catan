extern crate bytes;
extern crate futures;
extern crate tokio_core;
#[macro_use]
extern crate redis_async;
extern crate catan_protocols;
extern crate prost;

use std::sync::Arc;
use std::env;
use std::net::SocketAddr;

use futures::{future, Future, Stream};
use tokio_core::reactor::Core;
use redis_async::error::Error as RedisError;
use redis_async::client;
use redis_async::client::{PairedConnection, PubsubConnection};
use redis_async::resp::{RespValue, FromResp};
use std::error::Error;
use prost::Message;
use catan_protocols::services::game_management::{GameManagementRequest, RegisterNewGame, OpenPlayerRegistration};
use catan_protocols::services::game_management::game_management_request::RequestType;
use bytes::{IntoBuf, BytesMut, Bytes};

fn main() {
    let message = GameManagementRequest {
        game_name: "stenner-game-1".to_owned(),
        request_type: Some(RequestType::OpenPlayerRegistration(OpenPlayerRegistration {})),
    };

    let mut message_buffer = BytesMut::new();
    message.encode(&mut message_buffer).unwrap();
    let vec_content: Vec<u8> = message_buffer.freeze().to_vec();

    let topic: &'static str = "service:game-management";
    let address: SocketAddr = "127.0.0.1:6379".parse::<SocketAddr>().unwrap();
    let mut core = Core::new().unwrap();
    let connection = client::paired_connect(&address, &core.handle());

    let send_data = connection.and_then(|connection| {
        connection.send::<i64>(resp_array!["PUBLISH", topic, vec_content])
    });

    println!("{}", core.run(send_data).unwrap());
}