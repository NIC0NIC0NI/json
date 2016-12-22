mod to_json;

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
    fn make_number(s: &str) -> Option<Self> where Self:Sized;
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



