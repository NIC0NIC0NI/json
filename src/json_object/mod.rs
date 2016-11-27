mod json_value;
pub use self::json_value::JSONValue;

use ::std::collections::HashMap;
use ::std::fmt::{Display,Debug,Formatter,Result as FmtResult};

pub type NameValuePair = HashMap<String, JSON>;

pub enum JSON {
    JSONBool(bool),
    JSONInt(i64),
    JSONFloat(f64),
    JSONString(String),
    JSONArray(Vec<JSON>),
    JSONObject(NameValuePair),
    JSONNull
}

impl Display for JSON {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &JSON::JSONBool(b) => write!(f, "{}", b),
            &JSON::JSONInt(i) => write!(f, "{}" ,i),
            &JSON::JSONFloat(fp) => write!(f, "{}", fp),
            &JSON::JSONString(ref s) => write!(f, "\"{}\"", s),
            &JSON::JSONObject(ref object) => {
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
            &JSON::JSONArray(ref array) => {
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
            &JSON::JSONNull => write!(f, "null")
        }
    }
}

impl Debug for JSON{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &JSON::JSONBool(b) => write!(f, "{:?}", b),
            &JSON::JSONInt(i) => write!(f, "{:?}" ,i),
            &JSON::JSONFloat(fp) => write!(f, "{:?}", fp),
            &JSON::JSONString(ref s) => write!(f, "\"{:?}\"", s),
            &JSON::JSONObject(ref object) => {
                let mut first = true;
                try!(write!(f, "{{"));
                for (name, value) in object {
                    if first {
                        first = false;
                    } else {
                        try!(write!(f, ","));
                    }
                    try!(write!(f, "\"{:?}\":{:?}", name, value));
                }
                write!(f, "}}")
            },
            &JSON::JSONArray(ref array) => {
                let mut first = true;
                try!(write!(f, "["));
                for item in array {
                    if first {
                        first = false;
                    } else {
                        try!(write!(f, ","));
                    }
                    try!(write!(f, "{:?}", item));
                }
                write!(f, "]")
            },
            &JSON::JSONNull => write!(f, "null")
        }
    }
}

impl PartialEq for JSON {
    fn eq(&self, other: &JSON) -> bool{
        match self {
            &JSON::JSONBool(b1) => {
                match other {
                    &JSON::JSONBool(b2) => b1 == b2,
                    _ => false
                }
            },
            &JSON::JSONInt(i1) => {
                match other {
                    &JSON::JSONInt(i2) => i1 == i2,
                    _ => false
                }
            },
            &JSON::JSONFloat(f1) => {
                match other {
                    &JSON::JSONFloat(f2) => f1 == f2,
                    _ => false
                }
            },
            &JSON::JSONString(ref s1) => {
                match other {
                    &JSON::JSONString(ref s2) => s1 == s2 as &str,
                    _ => false
                }
            },
            &JSON::JSONArray(ref a1) => {
                match other {
                    &JSON::JSONArray(ref a2) => a1 == a2,
                    _ => false
                }
            },
            &JSON::JSONObject(ref nvp1) => {
                match other {
                    &JSON::JSONObject(ref nvp2) => nvp1 == nvp2,
                    _ => false
                }
            },
            &JSON::JSONNull => {
                match other {
                    &JSON::JSONNull => true,
                    _ => false
                }
            }
        }
    }
}


