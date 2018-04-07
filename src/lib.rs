extern crate curl;

#[macro_use]
extern crate json;

extern crate snowflake_multi_threaded;
extern crate time;
#[macro_use]
extern crate error_chain;

use std::result;

pub mod transport;
pub mod error;

mod packager;


pub use transport::client::{Builder,YarClient};
pub use transport::client::{YAR_OPT_CONNECT_TIMEOUT,YAR_OPT_PACKAGER,YAR_OPT_PERSISTENT,YAR_OPT_TIMEOUT};


pub type Result<T> = result::Result<T,error::YarError>;


mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }
}