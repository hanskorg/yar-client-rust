// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
use snowflake_multi_threaded::SnowFlakeId;

/// When a Client request a remote server, it will send a struct (in PHP):
pub struct YarRequest<'a>{
    pub id: u64,
    pub method: String,
    pub parameters: Vec<&'a str>
}

impl <'a> YarRequest <'a>{
    pub fn new(id:u64, method:String, parameters:Vec<&'a str> ) -> YarRequest{
        YarRequest{
            id,
            method,
            parameters
        }
    }

    pub fn encode(){

    }
}

