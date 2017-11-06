#![feature(conservative_impl_trait)]

extern crate futures;

extern crate tokio_core;
extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_serde_json;
extern crate tokio_service;

#[macro_use]
extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

extern crate catan_core;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::rc::Rc;

use futures::{Future, IntoFuture, Sink};
use futures::Stream;
use futures::future;
use tokio_core::reactor::{Core, Handle};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_service::Service;
use tokio_io::io::{ReadHalf, WriteHalf};
use tokio_io::codec::length_delimited::{FramedRead, FramedWrite};
use serde_json::Value;
use tokio_io::AsyncRead;
use tokio_serde_json::{ReadJson, WriteJson};

mod error;
use error::ServerError;

mod server;
use server::ServerInternalData;

pub mod services;
use services::{ServerRequest, ServerResponse, NewPlayerService, ServerErrorResponse};

fn _debugf<F: Future<Item = (), Error = ()>>(_: F) {}
fn _debugs<S: Stream<Item = (), Error = ()>>(_: S) {}

pub fn serve(port: u16) -> Result<(), std::io::Error> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    let server_data = Rc::new(ServerInternalData::default());

    let listener = TcpListener::bind(&address, &core.handle()).unwrap();
    info!("Server listening on {:?}", listener.local_addr());

    let server = listener
        .incoming()
        .map_err(|err| ServerError::from(err))
        .for_each(|(stream, addr)| {
            info!("{} connected", addr);
            handle_new_client(server_data.clone(), stream, addr, handle.clone())
        });

    core.run(server)?;

    Ok(())
}

fn handle_new_client(
    server_data: Rc<ServerInternalData>,
    stream: TcpStream,
    addr: SocketAddr,
    handle: Handle,
) -> impl IntoFuture<Item = (), Error = ServerError> {
    let (from_client, to_client): (ReadHalf<TcpStream>, WriteHalf<TcpStream>) = stream.split();
    let write_client: WriteJson<_, ServerResponse> = WriteJson::new(FramedWrite::new(to_client));
    let read_client: ReadJson<_, ServerRequest> = ReadJson::new(FramedRead::new(from_client));

    let responses = read_client.map_err(|err| ServerError::from(err)).and_then(
        |msg| {
            match msg {
                ServerRequest::NewPlayer { username } => {
                    let service = NewPlayerService;

                    service.call(username)
                }
                _ => {
                    info!("GOT: {:?}", msg);

                    Box::new(future::ok::<ServerResponse, ServerError>(
                        ServerResponse::Error {
                            inner: ServerErrorResponse::Other(String::from("Sounds grrreat!")),
                        },
                    ))
                }
            }
        },
    );

    let disconnect = write_client.send_all(responses).and_then(move |_| {
        info!("Client {} disconnected", addr);

        Ok(())
    }).map_err(|_| ());

    handle.spawn(disconnect);

    Ok(())
}