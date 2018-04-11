// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// When a Client request a remote server, it will send a struct (in PHP):

use packager::{YarHeader,Packager,YarProtocol};
use Result;

use std::boxed::Box;

#[derive(Debug)]
pub struct YarRequest{
    pub id: u32,
    pub method: String,
    pub parameters: Vec<String>,
}

impl YarRequest{
    pub fn new(id:u32, method:String, parameters:Vec<String> ) -> YarRequest{
        YarRequest{
            id,
            method,
            parameters
        }
    }

    pub fn encode(&self, packager:&Box<Packager>,token:&str, provider:&str )->Result<Vec<u8>>{
        let mut raw_data:Vec<u8> = packager.pack(&self);
        let mut protocol = YarProtocol{
            header: self.make_header(raw_data.len() as i32,token, provider),
            pack_name:[0;8],
            body: None
        };
        protocol.pack_name[0..4].copy_from_slice("JSON".as_bytes());
        let mut  bytes = protocol.get_bytes();
        bytes.pop();
        bytes.append(raw_data.as_mut());
        Ok(bytes)
    }

    fn make_header(&self, len:i32, token:&str, provider:&str )->YarHeader{
        let mut yar_header = YarHeader{
            id:self.id as i32,
            version:10,
            magic_num:0x80DFEC60,
            reserved: 0,
            body_len: len,
            ..Default::default()
        };

        let mut max_len ;
        let provider_bytes = provider.as_bytes();
        if provider_bytes.len() > 32 {
            max_len = 32;
        }else{
            max_len = provider_bytes.len();
        }
        yar_header.provider[0..max_len].copy_from_slice(provider_bytes);
        let token_bytes = token.as_bytes();
        if token_bytes.len() > 32 {
            max_len = 32;
        }else{
            max_len = token_bytes.len();
        }
        yar_header.token[0..max_len].copy_from_slice(token_bytes);
        yar_header
    }

}

