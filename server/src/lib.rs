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

use futures::{Future, IntoFuture, Stream};
use tokio_core::reactor::{Core, Handle};
use tokio_core::net::{TcpListener, TcpStream};
use tokio_io::codec::length_delimited::FramedRead;
use serde_json::Value;
use tokio_serde_json::ReadJson;

use catan_core::network::{ServerRequest, ServerResponse};

mod services;
use services::ServerInternalData;

fn _debugf<F: Future<Item = (), Error = ()>>(_: F) {}
fn _debugs<S: Stream<Item = (), Error = ()>>(_: S) {}

pub fn serve(port: u16) -> Result<(), std::io::Error> {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

    let server_data = Rc::new(ServerInternalData::default());

    let listener = TcpListener::bind(&address, &core.handle()).unwrap();
    info!("Server listening on {:?}", listener.local_addr());

    let server = listener.incoming().for_each(|(stream, addr)| {
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
) -> impl IntoFuture<Item = (), Error = std::io::Error> {
    info!("{} connected", addr);
    let length_delimited = FramedRead::new(stream);

    let deserialized = ReadJson::<FramedRead<TcpStream>, Value>::new(length_delimited)
        .map_err(|e| error!("ERR: {:?}", e));

    let handle_each_messages = deserialized.for_each(|msg| {
        let server_request: ServerRequest =
            serde_json::from_value(msg).expect("Expected value format");
        info!("GOT: {:?}", server_request);
        Ok(())
    });

    let disconnect = handle_each_messages.and_then(move |_| {
        info!("Client {} disconnected", addr);

        Ok(())
    });

    handle.spawn(disconnect);

    Ok(())
}