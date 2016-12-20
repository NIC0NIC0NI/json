#[macro_use]
mod macros;
mod cast_from_json;
mod cast_to_json;

use ::std::collections::HashMap;
use ::std::fmt::{Display,Formatter,Result as FmtResult};

pub type NameValuePair = HashMap<String, JSON>;

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
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<JSON>),
    Object(NameValuePair),
    Null
}

/// Convert rust primitive types to JSON.
pub trait IntoJSON {
    fn into_json(self) -> JSON;
}

impl Display for JSON {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &JSON::Bool(b) => write!(f, "{}", b),
            &JSON::Int(i) => write!(f, "{}" ,i),
            &JSON::Float(fp) => write!(f, "{}", fp),
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