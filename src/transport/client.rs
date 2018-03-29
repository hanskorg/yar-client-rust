// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use curl::easy::Easy;

pub const YAR_OPT_PACKAGER:u8 = 1 << 0;
pub const YAR_OPT_PERSISTENT:u8 = 1 << 1;
pub const YAR_OPT_TIMEOUT:u8 =  1<< 2;
pub const YAR_OPT_CONNECT_TIMEOUT:u8 = 1<< 3;

pub struct Builder{
    clientConf:YaClientConf
}
#[derive(Clone, Default)]
struct YaClientConf{
    url:String,
    timeout:i32,
    connect_timeout:i32,

    token:[char;32],
    provider:[char;32],

}
pub struct YaClient{
    curl_client:Easy,
}

impl YaClient{
    pub fn call(self,fn_name:&str, parameters:()){
        //self.curl_client.post_fields_copy([0u8]);
    }
}

impl Builder{

    fn setUrl(self,url:&str) -> Builder{
        self
    }

    fn setOpt(self, name:u8, value:&str) ->Result<Builder,Err>{
        if name.eq(&YAR_OPT_TIMEOUT){
            try!(self.clientConf.timeout = value.to_string().parse::<i32>());
        }
        if name.eq(&YAR_OPT_CONNECT_TIMEOUT){
            try!(self.clientConf.connect_timeout = value.to_string().parse::<i32>());
        }
        if name.eq(&YAR_OPT_PACKAGER){
            self.clientConf
        }
        self
    }
    ///Build a new YarClient ,Transport with curl
    /// `
    ///  Builder::default()
    ///  .setUrl("http://10limi.com/rpc.php")
    /// .setOpt(YAR_OPT_PACKAGER,"JSON").build();
    ///`
    ///
    fn build(self) -> YaClient {
        let mut client = YaClient{
            curl_client:Easy::new(),
        };
        client.curl_client.url(self.clientConf.url.as_ref());
        if self.clientConf.timeout > 0 {
            client.curl_client.timeout(self.clientConf.timeout);
        }
        if self.clientConf.connect_timeout > 0 {
            client.curl_client.connect_timeout(self.clientConf.connect_timeout);
        }
        client
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


#[test]
fn test_builder(){
    let client = Builder::default()
        .setUrl("http://10limi.com/rpc.php")
        .setOpt(YAR_OPT_PACKAGER,"JSON").build();
    let test = client.call("test",(1));
}