extern crate prost;
#[macro_use]
extern crate prost_derive;

pub mod services {
    pub mod game_management {
        include!(concat!(env!("OUT_DIR"), "/services.game_management.rs"));
    }
}
