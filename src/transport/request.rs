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

use packager::Packager;
use packager::YarHeader;
use std::boxed::Box;

use Result;

#[derive(Debug)]
pub struct YarRequest{
    pub id: u64,
    pub method: String,
    pub parameters: Vec<String>,
}

impl YarRequest{
    pub fn new(id:u64, method:String, parameters:Vec<String> ) -> YarRequest{
        YarRequest{
            id,
            method,
            parameters
        }
    }

    pub fn encode(&self, packager:&Box<Packager>,token:&str, provider:&str )->Result<Vec<u8>>{
        let raw_data = packager.pack(&self);
        self.encode_header(raw_data.as_ref().unwrap().len() as u32,token, provider);
        raw_data
    }

    fn encode_header(&self,len:u32, token:&str, provider:&str )->Vec<u8>{
        let mut bytes = Vec::<u8>::new();
        let mut yar_header = YarHeader{
            id:self.id as u32,
            version:1,
            magic_num:0x80DFEC60,
            reserved: 0,
            body_len: 0,
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

        println!("{:?}", yar_header);
        bytes
    }

}

