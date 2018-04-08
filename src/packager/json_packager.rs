use json;
use packager::Packager;
use Result;

use transport::request::YarRequest;
use transport::response::YarResponse;
pub struct JSONPackager;

impl Packager for JSONPackager{

    fn pack(&self, request: &YarRequest) -> Result<Vec<u8>> {
        let body = object!{
            "i" => request.id,
            "m" => "1",
            "p" => "aaa"
        };
        Ok(json::stringify(body).into_bytes())
    }

    fn unpack(&self, _content:Vec<u8>) -> YarResponse {
        YarResponse{
            id: 0,
            status: String::new(),
            ret: String::new(),
            out_put: String::new(),
            err: String::new(),
        }
    }
}
