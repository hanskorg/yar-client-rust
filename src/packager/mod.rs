// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use transport::request::YarRequest;
use transport::response::YarResponse;
use YarError;
use std::result;
use std::vec::Vec;
use std::rc::Rc;
use json;
use snowflake_multi_threaded::SnowFlakeId;

type Result<T> = result::Result<T, YarError>;

//pub fn generate_packager(package_name:&str) -> Packager {
//    match  package_name {
//        "JSON" => JSONPackager{},
//        "MsgPack" => panic!("unimplemented msgpack packager"),
//        _ => panic!(format!("can't use {} packager", package_name ))
//    }
//}

pub trait Packager:Sized{
    fn pack(request:&YarRequest) -> Result<Vec<u8>>;
    fn unpack( Vec<u8>) -> YarResponse;
}

pub struct JSONPackager;
impl Packager for JSONPackager{

     fn pack(request: &YarRequest) -> Result<Vec<u8>> {
        let body = object!{
            "i" => 1,
            "m" => "1",
            "p" => "aaa"
        };
        Ok(json::stringify(body).into_bytes())
    }

    fn unpack(content:Vec<u8>) -> YarResponse {
        YarResponse{
            id: 0,
            status: String::new(),
            ret: String::new(),
            out_put: String::new(),
            err: String::new(),
        }
    }
}

pub struct MsgPackPackager;
