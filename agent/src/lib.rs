extern crate catan_core;
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
use futures::stream::Fold;
use futures::future::ok;
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use tokio_io::codec::length_delimited::{FramedRead, FramedWrite};
use tokio_io::io::{ReadHalf, WriteHalf};
use tokio_io::AsyncRead;
use tokio_serde_json::{ReadJson, WriteJson};

use std::thread;
use futures::sync::mpsc;
use std::io::{Error, ErrorKind};

use catan_core::network::{ServerRequest, ServerResponse};

fn _debugf<F: Future<Item = (), Error = ()>>(_: F) {}
fn _debugs<S: Stream<Item = (), Error = ()>>(_: S) {}
fn _debug(_: ()) {}

pub enum AgentType {
    Simple
}

pub fn setup_agent(port: u16, agent_type: AgentType) -> Result<(), std::io::Error> {
    let (to_server, from_client) = mpsc::channel(1);
    let (to_client, from_server) = mpsc::unbounded();

    thread::spawn(move || {
        continuous_server_connection(port, from_client, to_client).unwrap()
    });

    match agent_type {
        AgentType::Simple => run_simple_agent(to_server, from_server)
    };

    Ok(())
}

fn run_simple_agent(to_server: mpsc::Sender<ServerRequest>, from_server: mpsc::UnboundedReceiver<ServerResponse>) -> Result<(), std::io::Error> {
    std::thread::sleep(std::time::Duration::from_secs(5));

    to_server.send(ServerRequest::NewPlayer(String::from("declanvk")));

    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}

fn continuous_server_connection<'a>(
    port: u16,
    from_client: mpsc::Receiver<ServerRequest>,
    to_client: mpsc::UnboundedSender<ServerResponse>,
) -> Result<(), std::io::Error> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    // Bind a server socket
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let socket = TcpStream::connect(&address, &core.handle());

    let connection = socket.and_then(|socket: TcpStream| {
        let (from_server, to_server): (ReadHalf<TcpStream>, WriteHalf<TcpStream>) = socket.split();

        // Serialize frames with JSON
        let write_server: WriteJson<FramedWrite<WriteHalf<TcpStream>>, ServerRequest> =
            WriteJson::new(FramedWrite::new(to_server));
        let read_server: ReadJson<FramedRead<ReadHalf<TcpStream>>, ServerResponse> =
            ReadJson::new(FramedRead::new(from_server));

        info!("Attached both write and read from server.");

        let reader: futures::stream::ForEach<
            _,
            _,
            serde::export::Result<(), tokio_serde_json::Error>,
        > = read_server.for_each(move |msg| {
            info!("Incoming message: {:?}", msg);
            to_client.clone().send(msg);

            Ok(())
        });

        let writer = from_client
            .map_err(|_| {
                tokio_serde_json::Error::Io(Error::new(
                    ErrorKind::Interrupted,
                    "Reciever failed! This should not happen",
                ))
            })
            .fold(write_server, |write_server, msg| {
                info!("Outgoing message to server: {:?}", msg);
                write_server.send(msg)
            })
            .map(|_| ());

        reader
            .select(writer)
            .map(|(_, select_next)| ())
            .map_err(|(err, select_next)| match err {
                tokio_serde_json::Error::Io(err) => err,
                tokio_serde_json::Error::Serde(err) => Error::new(ErrorKind::Other, err),
            })
    });

    core.run(connection)
}
