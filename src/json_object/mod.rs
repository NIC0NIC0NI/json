#[macro_use]
mod macros;
mod cast_from_json;
mod cast_to_json;

use super::number::Number;
use ::std::collections::HashMap;
use ::std::fmt::{Display,Formatter,Result as FmtResult};

/// The type that represents a JSON object.
/// Has implemented `FromStr` trait thus is able to parse from string.
/// # Examples
/// ```
/// extern crate json;
/// use json::JSON;
/// fn main(){
///     let json_str = stringify!(
///         { "first_property" : "good", "second_property" : [1, 2, 3, false, null]} 
///     );
///     if let Ok(json) = json_str.parse::<JSON>() {
///         println!("{}", json.to_string());
///     }
/// }
/// ```
#[derive(PartialEq, Debug, Clone)]
pub enum JSON {
    Bool(bool),
    Number(JSONNumber),
    String(String),
    Array(JSONArray),
    Object(JSONObject),
    Null
}

pub type JSONNumber = Number;
pub type JSONObject = HashMap<String, JSON>;
pub type JSONArray = Vec<JSON>;


impl Display for JSON {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &JSON::Bool(b) => write!(f, "{}", b),
            &JSON::Number(n) => write!(f, "{}" ,n),
            &JSON::String(ref s) => write!(f, "\"{}\"", s),
            &JSON::Object(ref object) => {
                let mut first = true;
                try!(write!(f, "{{"));
                for (name, value) in object {
                    if first {
                        first = false;
                    } else {
                        try!(write!(f, ","));
                    }
                    try!(write!(f, "\"{}\":{}", name, value));
                }
                write!(f, "}}")
            },
            &JSON::Array(ref array) => {
                let mut first = true;
                try!(write!(f, "["));
                for item in array {
                    if first {
                        first = false;
                    } else {
                        try!(write!(f, ","));
                    }
                    try!(write!(f, "{}", item));
                }
                write!(f, "]")
            },
            &JSON::Null => write!(f, "null")
        }
    }
}