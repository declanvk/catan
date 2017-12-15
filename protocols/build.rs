extern crate glob;
extern crate prost_build;

use std::path;
use std::error::Error;
use std::convert::AsRef;

const PROTO_DIR: &'static str = "protos/";
const FILE_EXTENSION: &'static str = "proto";

fn main() {
    let protocol_files: Vec<path::PathBuf> = match collect_all_proto_files(PROTO_DIR, FILE_EXTENSION) {
        Ok(files) => files,
        Err(err) => panic!("Unable to match files! {}", err.description())
    };

    protocol_files.iter().for_each(|ref path| println!("cargo:rerun-if-changed={}", path.display()));

    prost_build::compile_protos::<path::PathBuf>(protocol_files.as_ref(), &[path::PathBuf::from(PROTO_DIR)]).unwrap();
}

fn collect_all_proto_files(proto_folder: &str, file_extension: &str) -> Result<Vec<path::PathBuf>, glob::PatternError> {
    Ok(glob::glob(&format!("{}/**/*.{}", proto_folder, file_extension))?.map(|value| value.unwrap()).collect())
}