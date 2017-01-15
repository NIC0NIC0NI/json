mod to_json;

use ::std::str::FromStr;
use ::std::error::Error;
use super::from_json_str::NestedLevel;

/// For customized parsing
pub trait JSONArray 
    where Self::JSON:MakeJSON,
          <<Self::JSON as MakeJSON>::Number as FromStr>::Err: Error{
    type JSON;
    fn new(nested: &Vec<NestedLevel<Self::JSON>>) -> Self;
    fn add(&mut self, value: Self::JSON, nested: &Vec<NestedLevel<Self::JSON>>);
}

/// For customized parsing
pub trait JSONObject 
    where Self::JSON:MakeJSON,
          <<Self::JSON as MakeJSON>::Number as FromStr>::Err: Error{
    type JSON;
    fn new(nested: &Vec<NestedLevel<Self::JSON>>) -> Self;
    fn add(&mut self, name: String, value: Self::JSON, nested: &Vec<NestedLevel<Self::JSON>>);
}

/// For customized parsing
pub trait MakeJSON 
    where Self: Sized, 
          Self::Array: JSONArray<JSON=Self>,
          Self::Object: JSONObject<JSON=Self>,
          Self::Number: FromStr,
          <Self::Number as FromStr>::Err: Error {
    type Array;
    type Object;
    type Number;
    fn make_number(n: Self::Number, nested: &Vec<NestedLevel<Self>>) -> Self;
    fn make_null(nested: &Vec<NestedLevel<Self>>) -> Self;
    fn make_string(s: String, nested: &Vec<NestedLevel<Self>>) -> Self;
    fn make_bool(b: bool, nested: &Vec<NestedLevel<Self>>) -> Self;
    fn make_array(arr: Self::Array, nested: &Vec<NestedLevel<Self>>) -> Self;
    fn make_object(nvp: Self::Object, nested: &Vec<NestedLevel<Self>>) -> Self;
}



