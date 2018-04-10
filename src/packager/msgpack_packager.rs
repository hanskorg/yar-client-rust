use packager::Packager;
use Result;

use transport::request::YarRequest;
use transport::response::YarResponse;
pub struct MsgPackPackager;

impl Packager for MsgPackPackager{

    fn pack(&self, request: &YarRequest) -> Vec<u8> {
        unimplemented!()
    }


    fn get_name(&self) -> Vec<u8>{
        String::from("MSGPACK").into_bytes()

    }

    fn unpack(&self, _: Vec<u8>) -> YarResponse {
        unimplemented!()
    }

}
