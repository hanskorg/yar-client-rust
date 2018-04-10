// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

///
/// When a server response a result, it will send the struct
///
#[derive(Deserialize, Debug)]
pub struct YarResponse{
    pub i: i32,
    pub s: i32,
//    pub r: i32,
//    pub o: String,
//    pub e: String
}