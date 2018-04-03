extern crate curl;

#[macro_use]
extern crate json;

extern crate snowflake_multi_threaded;
extern crate time;
#[macro_use]
extern crate error_chain;

pub mod transport;
mod packager;
mod protocol;

use transport::response::YarResponse;
use transport::request::YarRequest;
use packager::Packager;

pub use transport::client::{Builder,YaClient,YarError};

#[allow(dead_code)]
use snowflake_multi_threaded::SnowFlakeId;


mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }
}