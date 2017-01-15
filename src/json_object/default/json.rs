use super::{DefaultJSON, Number, Map};
use super::super::super::type_adapt::{MakeJSON, JSONObject, JSONArray};
use super::super::super::from_json_str::NestedLevel;


impl JSONArray for Vec<DefaultJSON> {
    type JSON = DefaultJSON;
    fn new(_: &Vec<NestedLevel<Self::JSON>>) -> Self {
        Vec::new()
    }
    fn add(&mut self, value: Self::JSON, _: &Vec<NestedLevel<Self::JSON>>) {
        self.push(value);
    }
}

impl JSONObject for Map{
    type JSON = DefaultJSON;
    fn new(_: &Vec<NestedLevel<Self::JSON>>) -> Self{
        Map::new()
    }
    fn add(&mut self, name: String, value: Self::JSON, _: &Vec<NestedLevel<Self::JSON>>){
        self.insert(name,value);
    }
}

impl MakeJSON for DefaultJSON {
    type Array = Vec<DefaultJSON>;
    type Object = Map;
    type Number = Number;
    fn make_number(n: Number, _: &Vec<NestedLevel<Self>>) -> Self {
        DefaultJSON::Number(n)
    }
    fn make_null(_: &Vec<NestedLevel<Self>>) -> Self {
        DefaultJSON::Null
    }
    fn make_string(s: String, _: &Vec<NestedLevel<Self>>) -> Self {
        DefaultJSON::String(s)
    }
    fn make_bool(b: bool, _: &Vec<NestedLevel<Self>>) -> Self {
        DefaultJSON::Bool(b)
    }
    fn make_array(array: Self::Array, _: &Vec<NestedLevel<Self>>) -> Self {
        DefaultJSON::Array(array)
    }
    fn make_object(object: Self::Object, _: &Vec<NestedLevel<Self>>) -> Self {
        DefaultJSON::Object(object)
    }
}

