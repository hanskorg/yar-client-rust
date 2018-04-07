extern crate yar_client;

use yar_client::Builder;
use yar_client::transport::client::YAR_OPT_PACKAGER;
use std::sync::Arc;

fn main() {
    let mut client = Builder::default()
        .set_url("http://10limi.com/rpc1.php").unwrap()
        .set_opt(YAR_OPT_PACKAGER,"JSON").unwrap()
        .build().unwrap();
    let a = client.call("test",Arc::new(&["1".to_string(),"2".to_string()]));
    println!("===={:?}===",a.borrow());
    println!("{}","a");


}