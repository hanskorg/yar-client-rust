// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use transport::{YarRequest};
use packager::MsgPackPackager;
use packager::JSONPackager;
use packager::Packager;
use packager::YarProtocol;

use snowflake_multi_threaded::SnowFlakeId;
use curl::easy::Easy;
//use http::{Request, Response};
use time::Duration;

use std::cell::RefCell;
use std::boxed::Box;
use std::io::Read;

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
    packager_name:String,

    token:String,
    provider:String,

}
pub struct YarClient{
    curl_client:Easy,
    packager:Box<Packager>,
    snow_flake_id:SnowFlakeId,
    provider:String,
    token:String,
}

impl YarClient {
    /// Call a `Yar` method
    /// `parameters` can only use string slice Arc, and it will be return a [`Response`]: struct.Response.html value.
    /// Example:
    ///
    ///```rust
    ///   use yar_client::*;
    ///   let mut client = Builder::default()
    ///   .set_url("http://10limi.com/rpc.php").unwrap()
    ///   .set_opt(YAR_OPT_PACKAGER,"JSON").unwrap()
    ///   .set_opt(YAR_OPT_CONNECT_TIMEOUT, 1).unwrap()
    ///   .set_opt(YAR_OPT_TIMEOUT, 3).unwrap()
    ///   .build().unwrap();
    ///
    ///    let a = client.call("test",vec!["1".to_string(),"2".to_string()]);
    ///```
    ///
    pub fn call(&mut self,fn_name:&str, parameters:Vec<String>)->Result<String> {
        let req_id = self.snow_flake_id.generate_id().unwrap() & 0xFFFFFFF ;
        let request  = YarRequest::new(req_id as u32, fn_name.to_string(), parameters.clone());
        let post_raw:Vec<u8> = request.encode( &self.packager, self.token.as_ref(), self.provider.as_ref())?;
        let resp_raw:RefCell<Vec<u8>>  = RefCell::new(Vec::new());
        self.curl_client.post(true)?;
        self.curl_client.post_field_size(post_raw.len() as u64)?;
        
	let mut transfer = self.curl_client.transfer();

        transfer.read_function(|buf| {
            let a = post_raw.as_slice().read(buf).unwrap_or(0);
            Ok(a)
        })?;

        transfer.write_function( |data|  {
            resp_raw.borrow_mut().extend_from_slice(data);
	    Ok(data.len())
        })?;
        transfer.perform()?;
        
	let protocol = YarProtocol::to_protocol(resp_raw.borrow().to_vec());
        let resp = self.packager.unpack(protocol.body.unwrap());
        String::from("111");
        match resp.s {
            0 => Ok(resp.r),
            _ => {
                Err(YarError::CallErr(format!("code: {}, msg: {}, output: {}", resp.s, resp.e, resp.o)))
            }
        }
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
            self.client_conf.packager_name = value.to_string();
        }
        Ok(self)
    }
    ///Set yar client Provider name, Default : yar_client_rust
    /// If provider's length long than 32 bytes, It will be truncated
    pub fn set_provider(mut self, provider:&str) -> Builder{
        self.client_conf.provider = provider.to_string();
        self
    }

    ///Set yar client Token name, Default : yar_client_rust
    /// If provider's length long than 32 bytes, It will be truncated
    pub fn set_token(mut self, token:&str) -> Builder{
        self.client_conf.token = token.to_string();
        self
    }

    ///Build a new YarClient ,Transport with curl
    ///```rust
    ///  use yar_client::Builder;
    ///  use yar_client::YAR_OPT_PACKAGER;
    ///
    ///  Builder::default()
    ///  .set_url("http://10limi.com/rpc.php").unwrap()
    ///  .set_opt(YAR_OPT_PACKAGER,"JSON").unwrap().build();
    ///```
    ///
    pub fn build(self) -> Result<YarClient> {

        let mut client = YarClient{
            curl_client : Easy::new(),
            packager : match self.client_conf.packager_name.as_str() {
                "JSON" => Box::new(JSONPackager{}),
                "MSGPACK" => Box::new( MsgPackPackager{}),
                _  =>  Box::new(JSONPackager{})
            },
            snow_flake_id:SnowFlakeId::new(1,1),
            token: self.client_conf.token,
            provider:self.client_conf.provider,
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
                provider: "yar_client_rust".to_string(),
                token:"yar_client_rust".to_string(),
                ..Default::default()
            }
        }
    }
}
