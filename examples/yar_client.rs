extern crate yar_client;

use yar_client::*;

fn main() {
    let mut client = Builder::default()
        .set_url("http://10limi.com/rpc.php").unwrap()
        .set_opt(YAR_OPT_PACKAGER, "JSON").unwrap()
        .set_opt(YAR_OPT_CONNECT_TIMEOUT, 10).unwrap()
        .set_opt(YAR_OPT_TIMEOUT, 30).unwrap()
        .set_token("token")
        .set_provider("org.hansk.net.yarclient")
        .build().unwrap();
    let ret = client.call("test", vec!["1".to_string(), "2".to_string()]).unwrap();
    assert_eq!(ret,"1");
}