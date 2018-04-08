use packager::Packager;
use Result;

use transport::request::YarRequest;
use transport::response::YarResponse;
pub struct MsgPackPackager;

impl Packager for MsgPackPackager{

    fn pack(&self, request: &YarRequest) -> Result<Vec<u8>> {
        Ok(vec![1u8])
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
