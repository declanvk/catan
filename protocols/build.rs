extern crate glob;
extern crate capnpc;

use std::path;
use std::error::Error;
use std::env;

const PROTO_DIR: &'static str = "protos/";

fn main() {
    let protocol_files = match collect_all_proto_files(PROTO_DIR) {
        Ok(files) => files,
        Err(err) => panic!("Unable to match files! {}", err.description())
    };

    protocol_files.iter().for_each(|ref path| println!("cargo:rerun-if-changed={}", path.display()));

    let mut compiler_command = capnpc::CompilerCommand::new();

    protocol_files.iter().for_each(|ref path| {
        compiler_command.file(path.as_path().to_str().unwrap());
    });
    
    compiler_command.src_prefix("protocols").run().expect("Compiler run failed");
}

fn collect_all_proto_files(proto_folder: &str) -> Result<Vec<path::PathBuf>, glob::PatternError> {
    Ok(glob::glob(&format!("{}/**/*.capnp", proto_folder))?.map(|value| value.unwrap()).collect())
}