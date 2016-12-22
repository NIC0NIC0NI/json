use super::PreservingJSON as JSON;
use super::super::super::type_adapt::{MakeJSON, JSONObject, JSONArray};

use ::std::fmt::{Display,Formatter,Result as FmtResult};

impl JSONArray for Vec<JSON> {
    type JSON = JSON;
    fn new() -> Self {
        Vec::new()
    }
    fn add(&mut self, value: Self::JSON) {
        self.push(value);
    }
}

impl JSONObject for Vec<(String, JSON)>{
    type JSON = JSON;
    fn new() -> Self{
        Vec::new()
    }
    fn add(&mut self, name: String, value: Self::JSON){
        self.push((name,value));
    }
}

impl MakeJSON for JSON {
    type Array = Vec<JSON>;
    type Object = Vec<(String, JSON)>;
    type Number = String;
    fn make_number(s: String) -> Self {
        JSON::Number(s)
    }
    fn make_null() -> Self {
        JSON::Null
    }
    fn make_string(s: String) -> Self {
        JSON::String(s)
    }
    fn make_bool(b: bool) -> Self {
        JSON::Bool(b)
    }
    fn make_array(array: Self::Array) -> Self {
        JSON::Array(array)
    }
    fn make_object(object: Self::Object) -> Self {
        JSON::Object(object)
    }
}



impl Display for JSON {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &JSON::Bool(b) => write!(f, "{}", b),
            &JSON::Number(ref n) => write!(f, "{}" ,n),
            &JSON::String(ref s) => write!(f, "\"{}\"", s),
            &JSON::Object(ref object) => {
                let mut first = true;
                try!(write!(f, "{{"));
                for &(ref name, ref value) in object {
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