// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use transport::request::YarRequest;
use transport::response::YarResponse;
use Result;
use std::vec::Vec;

mod json_packager;
mod msgpack_packager;

pub use self::json_packager::JSONPackager;
pub use self::msgpack_packager::MsgPackPackager;

mod protocol;


pub trait Packager {
    fn pack(&self,request:&YarRequest) -> Result<Vec<u8>>;
    fn unpack(&self, Vec<u8>) -> YarResponse;
}

