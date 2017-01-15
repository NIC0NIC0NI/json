mod json;

use ::std::fmt::{Display,Formatter,Result as FmtResult};
use super::super::FromPremitive;

/// JSON object type that preserves information
#[derive(PartialEq, Debug, Clone)]
pub enum PreservingJSON {
    Bool(bool),
    Number(String),  // to avoid overflow, just left strings
    String(String),
    Array(Vec<PreservingJSON>),
    Object(Vec<(String, PreservingJSON)>), // to preserve order
    Null
}

impl <P:Display> FromPremitive<P> for String {
    fn from_premitive(p: P) -> String {
        p.to_string()
    }
}

impl Display for PreservingJSON {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &PreservingJSON::Bool(b) => write!(f, "{}", b),
            &PreservingJSON::Number(ref n) => write!(f, "{}" ,n),
            &PreservingJSON::String(ref s) => write!(f, "\"{}\"", s),
            &PreservingJSON::Object(ref object) => {
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
            &PreservingJSON::Array(ref array) => {
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
            &PreservingJSON::Null => write!(f, "null")
        }
    }
}

macro_rules! is_type {
    ($name:ident, $pattern:ident) => {
        pub fn $name(&self) -> bool {
            match self {
                &PreservingJSON::$pattern(_) => true,
                _ => false,
            }
        }
    }
}
macro_rules! as_type {
    ($name1:ident, $t1:ty, $name2:ident, $t2:ty, $pattern:ident) => {
        pub fn $name1(&self) -> Option<$t1> {
            match self {
                &PreservingJSON::$pattern(ref x) => Some(x),
                _ => None,
            }
        }
        pub fn $name2(self) -> Option<$t2> {
            match self {
                PreservingJSON::$pattern(x) => Some(x),
                _ => None,
            }
        }
    }
}

impl PreservingJSON {
    pub fn is_null(&self) -> bool {
        match self {
            &PreservingJSON::Null => true,
            _ => false,
        }
    }
    is_type!{is_number, Number}
    is_type!{is_bool, Bool}
    is_type!{is_string, String}
    is_type!{is_object, Object}
    is_type!{is_array, Array}
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            &PreservingJSON::Bool(b) => Some(b),
            _ => None,
        }
    }
    as_type!{as_number, &str, into_number, String, Number}
    as_type!{as_string, &str, into_string, String, String}
    as_type!{as_object, &Vec<(String, PreservingJSON)>, into_object, Vec<(String, PreservingJSON)>, Object}
    as_type!{as_array, &Vec<PreservingJSON>, into_array, Vec<PreservingJSON>, Array}
}