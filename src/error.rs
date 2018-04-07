// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::error;
use std::num::ParseIntError;
use curl::Error;

/// Error in [`Client`] calls
///
/// [`Client`]: transport/struct.YarClient.html
///
#[derive(Debug)]
pub enum YarError {
    /// Error when url not contains http or https
    URLError(&'static str),
    /// The underlying error is num::ParseIntError.
    TimeError(ParseIntError),
    CURLError(Error),

}

impl From<ParseIntError> for YarError{
    fn from(err:ParseIntError) -> YarError {
        YarError::TimeError(err)
    }
}

impl From<Error> for YarError{
    fn from(err: Error) -> YarError {
        YarError::CURLError(err)
    }
}

impl error::Error for YarError{
    fn description(&self) -> &str {
        match *self {
            YarError::TimeError(ref err) => err.description(),
            YarError::CURLError(ref err) => err.description(),
            YarError::URLError(description) => description
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            YarError::URLError(_) => None,
            YarError::TimeError(ref err)=> Some(err as &error::Error),
            YarError::CURLError(ref err)=> Some(err as &error::Error),
        }
    }
}

use std::fmt;

impl fmt::Display for YarError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            YarError::TimeError(ref err) => fmt::Display::fmt(err, f),
            YarError::CURLError(ref err) => fmt::Display::fmt(err, f),
            YarError::URLError(desc) => f.write_str(desc)
        }
    }
}
