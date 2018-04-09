use json;
use packager::Packager;
use Result;
use serde_json;
use transport::request::YarRequest;
use transport::response::YarResponse;
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
            i,
            m,
            p
        };
        serde_json::to_string(&json_body).unwrap().into_bytes()
    }

    fn unpack(&self, _content:Vec<u8>) -> YarResponse {
        let json_str = String::from_utf8(_content).unwrap();
        YarResponse{
            id: 0,
            status: String::new(),
            ret: String::new(),
            out_put: String::new(),
            err: String::new(),
        }
    }
    fn get_name(&self) -> Vec<u8>{
        String::from("JSON").into_bytes()
    }
}
