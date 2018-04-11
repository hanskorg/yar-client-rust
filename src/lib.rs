// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//!
//! yar_client is a crate what can transport info to Yar Server, About Yar see: <https://github.com/laruence/yar>
//!# Examples
//!```rust
//! use yar_client::*;
//! let mut client = Builder::default()
//! .set_url("http://10limi.com/rpc.php").unwrap()
//! .set_opt(YAR_OPT_PACKAGER, "JSON").unwrap()
//! .set_opt(YAR_OPT_CONNECT_TIMEOUT, 30).unwrap()
//! .set_opt(YAR_OPT_TIMEOUT, 30).unwrap()
//! .set_token("token")
//! .set_provider("org.hansk.net.yarclient")
//! .build().unwrap();
//! let ret = client.call("test", vec!["1".to_string(), "2".to_string()]).unwrap();
//! ```
//!# Builder
//!  use `yar_client::Build` build a new client,need call `*yar_client::Builder::set_url()*`,
//!  and can call `*set_opt()*` set up some options, this is the list of options:
//! - [x] YAR_OPT_PACKAGER :JSON 、MsgPack、PHP , the msg body encoding method.
//!    **current version only can use JSON**
//! - [ ] YAR_OPT_PERSISTENT , curl lib support keep-alive
//! - [x] YAR_OPT_TIMEOUT unit second , transport timeout
//! - [x] YAR_OPT_CONNECT_TIMEOUT unit second ,TCP connect timeout
//!
//!

extern crate curl;
//extern crate http;
extern crate snowflake_multi_threaded;
extern crate time;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
extern crate bincode;


use std::result;

mod transport;
mod packager;
pub mod error;

pub use transport::client::{Builder,YarClient};
pub use transport::client::{YAR_OPT_CONNECT_TIMEOUT,YAR_OPT_PACKAGER,YAR_OPT_PERSISTENT,YAR_OPT_TIMEOUT};


pub type Result<T> = result::Result<T,error::YarError>;



#[cfg(test)]
mod test {
    use Builder;
    use YAR_OPT_PACKAGER;
    use YAR_OPT_CONNECT_TIMEOUT;
    use YAR_OPT_TIMEOUT;
    #[test]
    fn test_builder() {
        let mut client = Builder::default()
            .set_url("http://10limi.com/rpc.php").unwrap()
            .set_opt(YAR_OPT_PACKAGER, "JSON").unwrap()
            .set_opt(YAR_OPT_CONNECT_TIMEOUT, 1).unwrap()
            .set_opt(YAR_OPT_TIMEOUT, 30).unwrap()
            .build().unwrap();
        let a = client.call("test", vec!["1".to_string(), "2".to_string()]);
    }
}