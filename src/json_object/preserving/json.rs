use super::PreservingJSON;
use super::super::super::type_adapt::{MakeJSON, JSONObject, JSONArray};
use super::super::super::from_json_str::NestedLevel;


impl JSONArray for Vec<PreservingJSON> {
    type JSON = PreservingJSON;
    fn new(_: &Vec<NestedLevel<Self::JSON>>) -> Self {
        Vec::new()
    }
    fn add(&mut self, value: Self::JSON, _: &Vec<NestedLevel<Self::JSON>>) {
        self.push(value);
    }
}

impl JSONObject for Vec<(String, PreservingJSON)>{
    type JSON = PreservingJSON;
    fn new(_: &Vec<NestedLevel<Self::JSON>>) -> Self{
        Vec::new()
    }
    fn add(&mut self, name: String, value: Self::JSON, _: &Vec<NestedLevel<Self::JSON>>){
        self.push((name,value));
    }
}

impl MakeJSON for PreservingJSON {
    type Array = Vec<PreservingJSON>;
    type Object = Vec<(String, PreservingJSON)>;
    type Number = String;
    fn make_number(s: String, _: &Vec<NestedLevel<Self>>) -> Self {
        PreservingJSON::Number(s)
    }
    fn make_null(_: &Vec<NestedLevel<Self>>) -> Self {
        PreservingJSON::Null
    }
    fn make_string(s: String, _: &Vec<NestedLevel<Self>>) -> Self {
        PreservingJSON::String(s)
    }
    fn make_bool(b: bool, _: &Vec<NestedLevel<Self>>) -> Self {
        PreservingJSON::Bool(b)
    }
    fn make_array(array: Self::Array, _: &Vec<NestedLevel<Self>>) -> Self {
        PreservingJSON::Array(array)
    }
    fn make_object(object: Self::Object, _: &Vec<NestedLevel<Self>>) -> Self {
        PreservingJSON::Object(object)
    }
}
