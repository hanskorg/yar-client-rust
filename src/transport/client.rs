// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use transport::request::YarRequest;
use packager::MsgPackPackager;
use packager::JSONPackager;
use packager::Packager;
use curl::easy::Easy;
use time::Duration;

use std::sync::Arc;
use std::io::{self,Read};
use std::cell::RefCell;
use std::rc::Rc;

use error::YarError;
use Result;

pub const YAR_OPT_PACKAGER:u8 = 1 << 0;
pub const YAR_OPT_PERSISTENT:u8 = 1 << 1;
pub const YAR_OPT_TIMEOUT:u8 =  1<< 2;
pub const YAR_OPT_CONNECT_TIMEOUT:u8 = 1<< 3;


pub struct Builder {
    client_conf:YarClientConf
}
#[derive(Clone, Default)]
struct YarClientConf{
    url:String,
    timeout:i64,
    connect_timeout:i64,
    packger_name:String,

    token:[char;32],
    provider:[char;32],

}
pub struct YarClient<'a>{
    curl_client:Easy,
    packager:&'a Packager,
}

impl <'a> YarClient <'a> {
    /// Call a `Yar` method
    /// `parameters` can only use string slice Arc, and it will be return a [`Response`]: struct.Response.html value.
    /// Example:
    ///
    ///```rust
    ///     use std::sync::Arc;
    ///     use yar_client::Builder;
    ///     use yar_client::transport::client::YAR_OPT_PACKAGER;
    ///     let mut client = Builder::default()
    ///     .set_url("http://10limi.com/rpc1.php").unwrap()
    ///     .set_opt(YAR_OPT_PACKAGER,"JSON").unwrap()
    ///     .build().unwrap();
    ///
    ///     client.call("test",Arc::new(&["1".to_string(),"2".to_string()]));
    ///```
    ///
    pub fn call(&mut self,fn_name:&str, parameters:Arc<&[&str]>) -> Rc<RefCell<Vec<u8>>> {
        self.curl_client.post(true);
        let request = YarRequest::new(1, String::from("method"), parameters.to_vec());
        // let mut data_to_upload =  &b"foobar"[..];self.packager.pack(&request).unwrap().as_slice());
        let mut data_to_upload = self.packager.pack(&request).unwrap();
        let mut transfer = self.curl_client.transfer();
        let stdin = io::stdin();
        transfer.read_function(move |into| {
            Ok(stdin.read(data_to_upload.as_slice()).unwrap())
        }).unwrap();

        let data_from_resp = Rc::new(RefCell::new(Vec::<u8>::new()));
        let data_from_resp_rc = data_from_resp.clone();
        transfer.write_function(  move |data|  {
            data_from_resp_rc.borrow_mut().extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();

        // transfer.perform().unwrap();

        // println!("call {} - {:?}", fn_name, parameters);

        data_from_resp
    }
}



impl <'a>Builder{

    /// Set Yar Api address
    /// **http or https only**
    ///
    pub fn set_url(mut self,url:&str) -> Result<Builder>{
        if url.is_empty() || (!url.starts_with("http") &&  !url.starts_with("https")){
           return Err(YarError::URLError("url must be contains http or https"));
        }
        self.client_conf.url = url.to_string();
        Ok(self)
    }
    /// Set Request Options
    /// - [x] YAR_OPT_PACKAGER :JSON 、MsgPack、PHP , the msg body encoding method.
    /// - [ ] YAR_OPT_PERSISTENT , Temporarily not supported
    /// - [x] YAR_OPT_TIMEOUT unit second , transport timeout
    /// - [x] YAR_OPT_CONNECT_TIMEOUT unit second ,TCP connect timeout
    ///
    pub fn set_opt<T>(mut self, name:u8, value:T) -> Result<Builder>
        where T:Sized + ToString
    {
        if name.eq(&YAR_OPT_TIMEOUT){
            self.client_conf.timeout = value.to_string().parse::<i64>()?;
        }
        if name.eq(&YAR_OPT_CONNECT_TIMEOUT){
            self.client_conf.connect_timeout = value.to_string().parse::<i64>()?;
        }
        if name.eq(&YAR_OPT_PACKAGER){
            self.client_conf.packger_name = value.to_string();
        }
        Ok(self)
    }
    ///Build a new YarClient ,Transport with curl
    ///```rust
    ///  use yar_client::Builder;
    ///  use yar_client::transport::client::YAR_OPT_PACKAGER;
    ///
    ///  Builder::default()
    ///  .set_url("http://10limi.com/rpc.php").unwrap()
    ///  .set_opt(YAR_OPT_PACKAGER,"JSON").unwrap().build();
    ///```
    ///
    pub fn build(self) -> Result<YarClient<'a>> {

        let mut client = YarClient{
            curl_client : Easy::new(),
            packager : match self.client_conf.packger_name.as_str() {
                "JSON" => &JSONPackager{},
                "MSGPACK" => &MsgPackPackager{},
                _  => &JSONPackager{}
            }
        };
        client.curl_client.url(self.client_conf.url.as_ref())?;
        if self.client_conf.timeout > 0 {
            client.curl_client.timeout(Duration::seconds(self.client_conf.timeout).to_std().unwrap())?;
        }
        if self.client_conf.connect_timeout > 0 {
            client.curl_client.connect_timeout(Duration::seconds(self.client_conf.connect_timeout).to_std().unwrap())?;
        }
        client.curl_client.url(self.client_conf.url.as_ref())?;
        Ok(client)
    }
}

impl <'a> Default for Builder{
    fn default() -> Self {
        Builder{
            client_conf : YarClientConf{
                timeout: 0,
                connect_timeout: 0,
                ..Default::default()
            }
        }
    }
}



#[test]
fn test_builder() {
    let mut client = Builder::default()
        .set_url("http://10limi.com/rpc1.php").unwrap()
        .set_opt(YAR_OPT_PACKAGER,"JSON").unwrap()
        .set_opt(YAR_OPT_CONNECT_TIMEOUT, 1).unwrap()
        .set_opt(YAR_OPT_TIMEOUT, 30).unwrap()
        .build().unwrap();
    let a = client.call("test",Arc::new(&["1".to_string(),"2".to_string()]));
    println!("===={:?}===",a.borrow());
}