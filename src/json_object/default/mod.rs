mod json;
mod number;
use super::super::FromJSONStr;

use ::std::collections::HashMap;
use ::std::fmt::{Display,Formatter,Result as FmtResult};
use ::std::str::FromStr;

pub use self::number::Number;

pub type Map = HashMap<String, DefaultJSON>;

/// Default JSON object type 
#[derive(PartialEq, Debug, Clone)]
pub enum DefaultJSON {
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<DefaultJSON>),
    Object(Map),
    Null
}

impl FromStr for DefaultJSON {
    type Err = <DefaultJSON as FromJSONStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        <DefaultJSON as FromJSONStr>::from_json_str(s)
    }
}
impl Display for DefaultJSON {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &DefaultJSON::Bool(b) => write!(f, "{}", b),
            &DefaultJSON::Number(n) => write!(f, "{}" ,n),
            &DefaultJSON::String(ref s) => write!(f, "\"{}\"", s),
            &DefaultJSON::Object(ref object) => {
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
            &DefaultJSON::Array(ref array) => {
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
            &DefaultJSON::Null => write!(f, "null")
        }
    }
}



macro_rules! is_type {
    ($name:ident, $pattern:ident) => {
        pub fn $name(&self) -> bool {
            match self {
                &DefaultJSON::$pattern(_) => true,
                _ => false,
            }
        }
    }
}
macro_rules! as_type {
    ($name1:ident, $t1:ty, $name2:ident, $t2:ty, $pattern:ident) => {
        pub fn $name1(&self) -> Option<$t1> {
            match self {
                &DefaultJSON::$pattern(ref x) => Some(x),
                _ => None,
            }
        }
        pub fn $name2(self) -> Option<$t2> {
            match self {
                DefaultJSON::$pattern(x) => Some(x),
                _ => None,
            }
        }
    };
    ($name:ident, $t:ty, $pattern:ident) => {
        pub fn $name(&self) -> Option<$t> {
            match self {
                &DefaultJSON::$pattern(x) => Some(x),
                _ => None,
            }
        }
    };
}

impl DefaultJSON {
    pub fn is_null(&self) -> bool {
        match self {
            &DefaultJSON::Null => true,
            _ => false,
        }
    }
    is_type!{is_number, Number}
    is_type!{is_bool, Bool}
    is_type!{is_string, String}
    is_type!{is_object, Object}
    is_type!{is_array, Array}
    as_type!{as_bool, bool, Bool}
    as_type!{as_number, Number, Number}
    as_type!{as_string, &str, into_string, String, String}
    as_type!{as_object, &Map, into_object, Map, Object}
    as_type!{as_array, &Vec<DefaultJSON>, into_array, Vec<DefaultJSON>, Array}
}