extern crate uuid;
extern crate byteorder;
extern crate bytes;
extern crate futures;
extern crate tokio_core;
#[macro_use]
extern crate redis_async;
extern crate prost;
#[macro_use]
extern crate lazy_static;
extern crate glob;

extern crate pretty_env_logger;
#[macro_use] extern crate log;

pub mod redis_scripts;
pub mod resource_naming;
pub mod uuid_generators;
pub mod error;
pub mod resp_helper;