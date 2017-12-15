use redis_async::resp::RespValue;
use error::{ServerError, ServerResult};

pub fn resp_value_as_bulk_contents(value: RespValue) -> ServerResult<Vec<u8>> {
    if let RespValue::BulkString(contents) = value {
        Ok(contents)
    } else {
        Err(ServerError::RespParse(
            "RespValue not BulkString".to_owned(),
        ))
    }
}