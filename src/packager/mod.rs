// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod json_packager;
mod msgpack_packager;

use bincode::{config,serialize,deserialize};

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
}

#[derive(Serialize,Deserialize,Debug,Default)]
pub struct YarHeader{
    pub id:       i32,
    pub version:  i16,
    pub magic_num:i32,
    pub reserved: i32,
    pub provider: [u8; 32],
    pub token:    [u8; 32],
    pub body_len: i32,
}
impl YarHeader {
    pub fn get_bytes(&self) -> Vec<u8>{
        config().big_endian().serialize(&self).unwrap()
    }
}
#[derive(Serialize,Deserialize,Debug,Default)]
pub struct YarProtocol{
    pub header:YarHeader,
    pub pack_name:[u8;8],
    pub body:Vec<u8>
}

impl YarProtocol{
    pub fn get_bytes(&self) -> Vec<u8>{
        config().big_endian().serialize(&self).unwrap()
    }
    pub fn to_protocol(bytes: Vec<u8>) -> YarProtocol{
        let bs      = bytes.as_slice();

        let mut ins = YarProtocol{
            header:config().big_endian().deserialize(&bs[0..82]).unwrap(),
            pack_name:config().big_endian().deserialize(&bs[82..90]).unwrap(),
            ..Default::default()
        };
        ins.body.extend_from_slice(&bs[90..]);
        ins
    }
}


