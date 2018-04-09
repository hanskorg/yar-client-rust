extern crate yar_client;

use yar_client::*;
use std::sync::Arc;

fn main() {
    let mut client = Builder::default()
        .set_url("http://10limi.com/rpc.php").unwrap()
        .set_opt(YAR_OPT_PACKAGER, "JSON").unwrap()
        .set_opt(YAR_OPT_CONNECT_TIMEOUT, 10).unwrap()
        .set_opt(YAR_OPT_TIMEOUT, 30).unwrap()
        .build().unwrap();
    let a = client.call("test", vec!["1".to_string(), "2".to_string()]);
    println!("===={:?}===",a);
    println!("{}","a");
}