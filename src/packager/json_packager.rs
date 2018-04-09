use json;
use packager::Packager;
use Result;

use transport::request::YarRequest;
use transport::response::YarResponse;
pub struct JSONPackager;

impl Packager for JSONPackager{

    fn pack(&self, request: &YarRequest) -> Result<Vec<u8>> {
        let id = request.id.clone();
        let method = request.method.clone();
        let parameters = request.parameters.clone();
        let body = object!{
            "i" => id,
            "m" => method,
            "p" => parameters
        };
        Ok(json::stringify(body).into_bytes())
    }

    fn unpack(&self, _content:Vec<u8>) -> YarResponse {
        let json_str = String::from_utf8(_content).unwrap();
        println!("{:?}",json::parse(json_str.as_ref()));
        YarResponse{
            id: 0,
            status: String::new(),
            ret: String::new(),
            out_put: String::new(),
            err: String::new(),
        }
    }
}
