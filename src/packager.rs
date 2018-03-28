// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


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
    provider: [char; 32],
    token:    [char; 32],
    body_len: u32,
}
///
/// When a Client request a remote server, it will send a struct (in PHP):
pub  struct YarRequest<'a>{
    i: u64,
    m: String,
    p: Vec<&'a str>
}
///
/// When a server response a result, it will send the struct
///
pub struct YarResponse <T: ?Sized>{
    i: u64,
    s: &'a str,
    r: T,
    o: &'a str,
    e: &'a str
}