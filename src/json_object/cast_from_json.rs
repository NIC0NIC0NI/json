use super::JSON;
use super::NameValuePair;

macro_rules! is_type {
    ($fn_name:ident, Null) => {
        pub fn $fn_name(&self) -> bool {
            match self {
                &JSON::Null => true,
                _ => true
            }
        }
    };
    ($fn_name:ident,  $json_type:ident) => {
        pub fn $fn_name(&self) -> bool {
            match self {
                &JSON::$json_type(_) => true,
                _ => false
            }
        }
    };
}

macro_rules! to_premitive {
    ($fn_name:ident, $rust_type:ty, $json_type:ident) => {
        pub fn $fn_name(&self) -> Option<$rust_type> {
            match self {
                &JSON::$json_type(x) => Some(x),
                _ => None
            }
        }
    };
}

macro_rules! to_object {
    ($as_ref_fn_name:ident, $into_fn_name:ident, String, $json_type:ident) => {
        /// This method returns `Option<&str>`, which is not empty if the object is a `JSONString`.
        ///
        /// It is quite different from [`json::JSON::to_string()`](https://doc.rust-lang.org/std/string/trait.ToString.html).
        pub fn $as_ref_fn_name(&self) -> Option<&str> { // String to &str
            match self {
                &JSON::$json_type(ref x) => Some(x),
                _ => None
            }
        }
        /// This method returns `Option<String>`, which is not empty if the object is a `JSONString`.
        ///
        /// It is quite different from [`json::JSON::to_string()`](https://doc.rust-lang.org/std/string/trait.ToString.html).
        pub fn $into_fn_name(self) -> Option<String> {
            match self {
                JSON::$json_type(x) => Some(x),
                _ => None
            }
        }
    };
    ($as_ref_fn_name:ident, $into_fn_name:ident, $rust_type:ty, $json_type:ident) => {
        pub fn $as_ref_fn_name(&self) -> Option<&$rust_type> {
            match self {
                &JSON::$json_type(ref x) => Some(x),
                _ => None
            }
        }
        pub fn $into_fn_name(self) -> Option<$rust_type> {
            match self {
                JSON::$json_type(x) => Some(x),
                _ => None
            }
        }
    };
}

impl JSON {
    /// Boolean, integer, floating point number or string
    pub fn is_primitive(&self) -> bool {
        match self {
            &JSON::Array(_) | &JSON::Object(_) | &JSON::Null => false,
            _ => true
        }
    }
    is_type!{is_null, Null}
    is_type!{is_integer, Int}
    is_type!{is_float, Float}
    is_type!{is_bool, Bool}
    is_type!{is_string, String}
    is_type!{is_array, Array}
    is_type!{is_object, Object}
    to_premitive!{as_i64, i64, Int}
    to_premitive!{as_f64, f64, Float}
    to_premitive!{as_bool, bool, Bool}
    to_object!{as_str, into_string, String, String}
    to_object!{as_vec, into_vec, Vec<JSON>, Array}
    to_object!{as_map, into_map, NameValuePair, Object}
}
