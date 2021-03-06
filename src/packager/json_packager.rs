use packager::Packager;
use serde_json;
use transport::{YarRequest, YarResponse};
pub struct JSONPackager;

#[derive(Serialize, Deserialize, Debug)]
struct JsonBody{
    i:u32,
    m:String,
    p:Vec<String>
}

impl Packager for JSONPackager{

    fn pack(&self, request: &YarRequest) -> Vec<u8> {
        let i = request.id.clone() as u32 ;
        let m = request.method.clone();
        let p = request.parameters.clone();
//        let body = object!{
//            "i" => id,
//            "m" => method,
//            "p" => parameters
//        };
        let json_body = JsonBody{
            p,
            i,
            m
        };
        serde_json::to_string(&json_body).unwrap().into_bytes()
    }

    fn unpack(&self, _content:Vec<u8>) -> YarResponse {
        let resp:YarResponse = serde_json::from_str(String::from_utf8(_content).unwrap().as_str()).unwrap();
        resp
    }
    fn get_name(&self) -> Vec<u8>{
        String::from("JSON").into_bytes()
    }
}
