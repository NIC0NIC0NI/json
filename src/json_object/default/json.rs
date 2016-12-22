use super::{DefaultJSON as JSON, Number, Map};
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

impl JSONObject for Map{
    type JSON = JSON;
    fn new() -> Self{
        Map::new()
    }
    fn add(&mut self, name: String, value: Self::JSON){
        self.insert(name,value);
    }
}

impl MakeJSON for JSON {
    type Array = Vec<JSON>;
    type Object = Map;
    fn make_number(s: &str) -> Option<Self> {
        if s.contains('.') {
            s.parse::<f64>().ok().map(|f| JSON::Number(Number::Float(f)))
        } else if let Ok(i) = s.parse::<i64>() {
            Some(JSON::Number(Number::Int(i)))
        } else {
            s.parse::<f64>().ok().map(|f| JSON::Number(Number::Float(f)))
        }
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