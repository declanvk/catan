extern crate capnp;

mod game_management_capnp {
    include!(concat!(env!("OUT_DIR"), "/protos/game_management_capnp.rs"));
}

mod player_management_capnp {
    include!(concat!(env!("OUT_DIR"), "/protos/player_management_capnp.rs"));
}

mod game_server_capnp {
    include!(concat!(env!("OUT_DIR"), "/protos/game_server_capnp.rs"));
}