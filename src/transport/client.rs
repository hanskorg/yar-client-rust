// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use curl::easy::Easy;
use curl::Error;
use time::Duration;

use std::num::{ParseIntError};
use std::result;
use std::sync::Arc;
use std::error;
use std::io::{Read};
use std::cell::RefCell;
use std::rc::Rc;

pub const YAR_OPT_PACKAGER:u8 = 1 << 0;
pub const YAR_OPT_PERSISTENT:u8 = 1 << 1;
pub const YAR_OPT_TIMEOUT:u8 =  1<< 2;
pub const YAR_OPT_CONNECT_TIMEOUT:u8 = 1<< 3;

type Result<T> = result::Result<T,YarError>;

pub struct Builder{
    clientConf:YaClientConf
}
#[derive(Clone, Default)]
struct YaClientConf{
    url:String,
    timeout:i64,
    connect_timeout:i64,
    packger_name:String,

    token:[char;32],
    provider:[char;32],

}
pub struct YaClient{
    curl_client:Easy,
}



impl YaClient{
    /// Call a `Yar` method
    /// `parameters` can only use string slice Arc, and it will be return a [`Response`]: struct.Response.html value.
    /// Example:
    ///`rust
    /// let client = Builder::default()
    /// .setUrl("http://10limi.com/rpc1.php").unwrap()
    /// .setOpt(YAR_OPT_PACKAGER,"JSON").unwrap()
    /// .build().unwrap();
    ///
    /// client.call("test",Arc::new(&["1".to_string(),"2".to_string()]));
    ///`
    ///
    pub fn call(&mut self,fn_name:&str, parameters:Arc<&[String]>) -> Rc<RefCell<Vec<u8>>> {
        self.curl_client.post(true);
        let mut transfer = self.curl_client.transfer();

        let mut data_to_upload = &b"foobar"[..];
        transfer.read_function(move |into| {
            Ok(data_to_upload.read(into).unwrap())
        }).unwrap();

        let data_from_resp = Rc::new(RefCell::new(Vec::<u8>::new()));
        let data_from_resp_rc = data_from_resp.clone();
        transfer.write_function(  move |data|  {
            data_from_resp_rc.borrow_mut().extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
        data_from_resp
    }
}



impl Builder{

    pub fn setUrl(mut self,url:&str) -> Result<Builder>{
        if url.is_empty() || (!url.starts_with("http") &&  !url.starts_with("https")){
           return Err(YarError::URLError("url must be contains http or https"));
        }
        self.clientConf.url = url.to_string();
        Ok(self)
    }

    pub fn setOpt(mut self, name:u8, value:&str) -> Result<Builder>{
        if name.eq(&YAR_OPT_TIMEOUT){
            self.clientConf.timeout = value.to_string().parse::<i64>()?;
        }
        if name.eq(&YAR_OPT_CONNECT_TIMEOUT){
            self.clientConf.connect_timeout = value.to_string().parse::<i64>()?;
        }
        if name.eq(&YAR_OPT_PACKAGER){
            self.clientConf.packger_name = value.to_string();
        }
        Ok(self)
    }
    ///Build a new YarClient ,Transport with curl
    /// `
    ///  Builder::default()
    ///  .setUrl("http://10limi.com/rpc.php")
    /// .setOpt(YAR_OPT_PACKAGER,"JSON").build();
    ///`
    ///
    pub fn build(mut self) -> Result<YaClient> {
        let mut client = YaClient{
            curl_client:Easy::new(),
        };
        client.curl_client.url(self.clientConf.url.as_ref());
        if self.clientConf.timeout > 0 {
            client.curl_client.timeout(Duration::seconds(self.clientConf.timeout).to_std().unwrap());
        }
        if self.clientConf.connect_timeout > 0 {
            client.curl_client.connect_timeout(Duration::seconds(self.clientConf.connect_timeout).to_std().unwrap());
        }
        client.curl_client.url(self.clientConf.url.as_ref())?;
        Ok(client)
    }
}

impl Default for Builder{
    fn default() -> Self {
        Builder{
            clientConf : YaClientConf{
                timeout: 0,
                connect_timeout: 0,
                ..Default::default()
            }
        }
    }
}


/// Error in [`Client`] calls
///
/// [`Client`]: struct.YaClient.html
///
#[derive(Debug)]
pub enum YarError {
    /// Error when url not contains http or https
    URLError(&'static str),
    /// The underlying error is num::ParseIntError.
    TimeError(ParseIntError),
    CURLError(Error),

}

impl From<ParseIntError> for YarError{
    fn from(err:ParseIntError) -> YarError {
        YarError::TimeError(err)
    }
}

impl From<Error> for YarError{
    fn from(err: Error) -> YarError {
        YarError::CURLError(err)
    }
}

impl error::Error for YarError{
    fn description(&self) -> &str {
        match *self {
            YarError::TimeError(ref err) => err.description(),
            YarError::CURLError(ref err) => err.description(),
            YarError::URLError(description) => description
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            YarError::URLError(_) => None,
            YarError::TimeError(ref err)=> Some(err as &error::Error),
            YarError::CURLError(ref err)=> Some(err as &error::Error),
        }
    }
}

use std::fmt;

impl fmt::Display for YarError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            YarError::TimeError(ref err) => fmt::Display::fmt(err, f),
            YarError::CURLError(ref err) => fmt::Display::fmt(err, f),
            YarError::URLError(desc) => f.write_str(desc)
        }
    }
}


#[test]
fn test_builder() {
    let mut client = Builder::default()
        .setUrl("http://10limi.com/rpc1.php").unwrap()
        .setOpt(YAR_OPT_PACKAGER,"JSON").unwrap()
        .build().unwrap();
    let a = client.call("test",Arc::new(&["1".to_string(),"2".to_string()]));
    println!("===={:?}===",a.borrow());
    println!("{}","a");

}