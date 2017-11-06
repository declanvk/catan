#![feature(conservative_impl_trait)]

extern crate catan_core;
extern crate catan_server;
extern crate futures;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_serde_json;
extern crate tokio_service;
extern crate uuid;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use futures::prelude::*;
use tokio_core::reactor::{Core, Handle};
use tokio_core::net::TcpStream;
use tokio_io::codec::length_delimited::{FramedRead, FramedWrite};
use tokio_io::io::{ReadHalf, WriteHalf};
use tokio_io::AsyncRead;
use tokio_serde_json::{ReadJson, WriteJson};

use std::thread;
use futures::sync::mpsc;
use std::error::Error;

use catan_server::services::{ServerRequest, ServerResponse};

mod error;
use error::ClientError;

fn _debugf<F: Future<Item = (), Error = ()>>(_: F) {}
fn _debugs<S: Stream<Item = (), Error = ()>>(_: S) {}
fn _debug(_: ()) {}

pub enum AgentType {
    Simple,
}

pub fn setup_agent(port: u16, agent_type: AgentType) -> Result<(), ClientError> {
    // client -> server
    let (to_server, from_client) = mpsc::channel(1);
    // server -> client
    let (to_client, from_server) = mpsc::channel(1);

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let remote = core.remote();
    // Bind a server socket
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let socket = TcpStream::connect(&address, &core.handle());

    // Spawn agents on separate thread
    thread::spawn(move || {
        remote.spawn(move |handle| match agent_type {
            AgentType::Simple => run_simple_agent(to_server, from_server, handle),
        })
    });

    // Run agent server on main thread
    let agent_server = socket.map_err(|err| ClientError::from(err)).and_then(
        |socket| {
            continuous_server_connection(socket, from_client, to_client, handle.clone())
        },
    );

    core.run(agent_server)
}

fn run_simple_agent(
    to_server: mpsc::Sender<ServerRequest>,
    from_server: mpsc::Receiver<ServerResponse>,
    handle: &Handle,
) -> impl Future<Item = (), Error = ()> {
    let messages = to_server
        .send(ServerRequest::NewPlayer {
            username: String::from("declanvk")
        })
        .map_err(|err| ClientError::from(err))
        .join(from_server.into_future().map_err(|(err, _)| {
            ClientError::from("Server reciever failed! This should not happen")
        }));

    messages.map(|_| ()).map_err(|_| ())
}

fn continuous_server_connection<'a>(
    stream: TcpStream,
    from_client: mpsc::Receiver<ServerRequest>,
    to_client: mpsc::Sender<ServerResponse>,
    handle: Handle,
) -> impl Future<Item = (), Error = ClientError> {
    let (from_server, to_server): (ReadHalf<TcpStream>, WriteHalf<TcpStream>) = stream.split();

    // Serialize frames with JSON
    let write_server: WriteJson<_, ServerRequest> = WriteJson::new(FramedWrite::new(to_server));
    let read_server: ReadJson<_, ServerResponse> = ReadJson::new(FramedRead::new(from_server));

    let reader = read_server.map_err(|err| ClientError::from(err)).for_each(
        move |msg| {
            info!("Incoming message: {:?}", msg);

            to_client.clone().send(msg).map(|_| ()).map_err(|err| {
                ClientError::from(err)
            })
        },
    );

    let writer = from_client
        .map_err(|_| {
            ClientError::from("Reciever failed! This should not happen")
        })
        .fold(write_server, |write_server, msg| {
            info!("Outgoing message to server: {:?}", msg);

            write_server.send(msg).map_err(|err| ClientError::from(err))
        })
        .map(|_| ());

    reader.select(writer).map(|_| ()).map_err(|(err, _)| {
        ClientError::from(err)
    })
}
