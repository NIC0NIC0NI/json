mod to_json;

use ::std::str::FromStr;
use ::std::error::Error;

pub trait JSONArray {
    type JSON;
    fn new() -> Self;
    fn add(&mut self, value: Self::JSON)
         where Self::JSON:MakeJSON;
}

pub trait JSONObject {
    type JSON;
    fn new() -> Self;
    fn add(&mut self, name: String, value: Self::JSON)
         where Self::JSON:MakeJSON;
}

pub trait MakeJSON {
    type Array;
    type Object;
    type Number;
    fn make_number(n: Self::Number) -> Self
        where Self::Number: FromStr,
              <Self::Number as FromStr>::Err: Error + 'static;
    fn make_null() -> Self;
    fn make_string(s: String) -> Self;
    fn make_bool(b: bool) -> Self;
    fn make_array(arr: Self::Array) -> Self
        where Self:Sized,
              Self::Array: JSONArray<JSON=Self>;
    fn make_object(nvp: Self::Object) -> Self
        where Self:Sized,
              Self::Object: JSONObject<JSON=Self>;
}



