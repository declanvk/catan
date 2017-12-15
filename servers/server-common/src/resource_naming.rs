use std::fmt;

const SERVICE_PREFIX: &'static str = "service";

pub fn service_key<S: fmt::Display>(name: S) -> String {
    format!("{}:{}", SERVICE_PREFIX, name)
}

const TEMPORARY_RESOURCE_PREFIX: &'static str = "temp";

pub fn temporary_resource_key<S: fmt::Display>(name: S) -> String {
    format!("{}:{}", TEMPORARY_RESOURCE_PREFIX, name)
}

const COLLECTION_PREFIX: &'static str = "collection";

pub fn collection_key<S: fmt::Display>(name: S) -> String {
    format!("{}:{}", COLLECTION_PREFIX, name)
}