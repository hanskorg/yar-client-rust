extern crate yar_client;

use yar_client::Builder;
use yar_client::transport::client::*;
use std::sync::Arc;

fn main() {
    let mut client = Builder::default()
        .set_url("http://10limi.com/rpc1.php").unwrap()
        .set_opt(YAR_OPT_PACKAGER,"JSON").unwrap()
        .set_opt(YAR_OPT_CONNECT_TIMEOUT, 1).unwrap()
        .set_opt(YAR_OPT_TIMEOUT, 3).unwrap()
        .build().unwrap();
    let a = client.call("test",Arc::new(&["1","2"]));
    println!("===={:?}===",a);
    println!("{}","a");


}