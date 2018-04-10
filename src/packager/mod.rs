// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod json_packager;
mod msgpack_packager;

use bincode::{config,serialize};

use transport::request::YarRequest;
use transport::response::YarResponse;
use Result;
use std::vec::Vec;




pub use self::json_packager::JSONPackager;
pub use self::msgpack_packager::MsgPackPackager;

mod protocol;


pub trait Packager {
    fn pack(&self,request:&YarRequest) -> Vec<u8>;
    fn unpack(&self, Vec<u8>) -> YarResponse;
    fn get_name(&self) ->Vec<u8>;
//    fn encode(&self, request:&YarRequest, body:Vec<u8>)->Vec<u8>{
//
//    }
}

#[derive(Serialize,Debug,Default)]
pub struct YarHeader{
    pub id:       u32,
    pub version:  u16,
    pub magic_num:u32,
    pub reserved: u32,
    pub provider: [u8; 32],
    pub token:    [u8; 32],
    pub body_len: u32,
}
impl YarHeader {
    pub fn get_bytes(&self) -> Vec<u8>{
        config().big_endian().serialize(&self).unwrap()
    }
}
#[derive(Serialize,Debug,Default)]
pub struct YarProtocol{
    pub header:YarHeader,
    pub pack_name:[u8;8],
    pub body:String
}

impl YarProtocol{
    pub fn get_bytes(&self) -> Vec<u8>{
        config().big_endian().serialize(&self).unwrap()
    }
}


