// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::vec::Vec;
use transport::request::YarRequest;
///
/// typedef struct _yar_header {
///   unsigned int   id;
///   unsigned short version;
///   unsigned int   magic_num;
///   unsigned int   reserved;
///   unsigned char  provider[32];
///   unsigned char  token[32];
///   unsigned int   body_len;
///   }
pub struct YarHeader{
    id:       u32,
    version:  u8,
    magic_num:u32,
    reserved: u32,
    provider: [u8; 32],
    token:    [u8; 32],
    body_len: u32,
}


pub fn encode(request:YarRequest)->Vec<u8>{
    let bytes = Vec::<u8>::new();
    let mut yar_header = YarHeader{
        id:request.id as u32,
        version:1,
        magic_num:0x80DFEC60,
        reserved: 0,
        provider:Default::default(),
        token:Default::default(),
        body_len:32,
    };
    yar_header.provider.copy_from_slice("ddd".as_bytes());
    yar_header.token.copy_from_slice("ddd".as_bytes());
    bytes
}
