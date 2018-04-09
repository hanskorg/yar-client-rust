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
        println!("===={:?}===", a.borrow());
    }
}

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

error_chain! {
    types {
        Error, ErrorKind, ResultExt;
    }
}