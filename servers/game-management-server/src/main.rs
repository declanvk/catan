#![feature(try_trait)]
#![feature(conservative_impl_trait)]

extern crate uuid;
extern crate byteorder;
extern crate bytes;
extern crate futures;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_timer;
#[macro_use]
extern crate redis_async;
extern crate prost;
extern crate glob;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

extern crate catan_protocols;
extern crate server_common;

pub mod game_management;

use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

use futures::{Future, Stream};
use tokio_core::reactor::Core;
use tokio_service::Service;

use redis_async::client;

use server_common::error::{ServerError, ServerResult};
use server_common::uuid_generators::UuidV1Generator;

use game_management::{GameManagementService, deserialize_request};

fn main() {
    pretty_env_logger::init().unwrap();

    let address = "127.0.0.1:6379".parse().expect(
        "Socket address parsing failed",
    );
    let script_folder_path = From::from("./../scripts/");

    let num_requests_served = run_server(address, script_folder_path).expect("Management service failed");

    info!("Served {} requests", num_requests_served)
}

pub fn run_server(address: SocketAddr, script_folder: PathBuf) -> ServerResult<usize> {
    info!(
        "Current working directory: {}",
        env::current_dir().unwrap().display()
    );

    let mut core = Core::new()?;
    let handle = core.handle();
    let uuid_context = Arc::new(UuidV1Generator::new_context());

    let create_service = client::paired_connect(&address, &handle)
        .map_err(ServerError::from)
        .and_then(move |paired_connection| {
            GameManagementService::new(
                address,
                script_folder,
                handle,
                uuid_context.clone(),
                paired_connection,
            )
        });

    let create_subscription = client::pubsub_connect(&address, &core.handle())
        .map_err(ServerError::from)
        .and_then(move |pubsub_connection| {
            pubsub_connection
                .subscribe(GameManagementService::service_topic())
                .map_err(ServerError::from)
        });

    let handling_requests = create_service.join(create_subscription).and_then(
        move |(service, subscription_messages)| {
            let inner_service = Rc::new(service);
            subscription_messages
                .map_err(|_| {
                    ServerError::Custom("Error in message stream".to_owned())
                })
                .and_then(deserialize_request)
                .and_then(move |request| Rc::clone(&inner_service).call(request))
                .or_else(|err| {
                    error!("Request handling error! {}", err.description());
                    Ok(())
                })
                .collect()
        },
    );

    let results: Vec<()> = core.run(handling_requests)?;

    Ok(results.len())
}