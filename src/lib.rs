extern crate curl;
extern crate json;
extern crate snowflake_multi_threaded;
#[macro_use]
extern crate error_chain;

pub mod protocol;
pub mod transport;
pub mod packager;

use snowflake_multi_threaded::SnowFlakeId;

pub use protocol::{YarHeader,YarRequest,YarResponse};

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}