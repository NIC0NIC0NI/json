use super::JSON;
use super::JSONValue;


impl <'arbitrary> JSONValue for &'arbitrary str {
    fn to_json(self) -> JSON{
        JSON::JSONString(self.to_string())
    }
}

impl JSONValue for String {
    fn to_json(self) -> JSON{
        JSON::JSONString(self)
    }
}

macro_rules! type_map {
    ($rust_type: ty, $inter_type: ty, $json_type: ident) => {
        impl JSONValue for $rust_type {
            fn to_json(self) -> JSON{
                $json_type(self as $inter_type)
            }
        }
    };
    ($rust_type: ty, $json_type: ident) => {
        impl JSONValue for $rust_type {
            fn to_json(self) -> JSON{
                $json_type(self)
            }
        }
    };
}

use JSON::*;

type_map!{bool, JSONBool}
type_map!{i8, i64, JSONInt}
type_map!{i16, i64, JSONInt}
type_map!{i32, i64, JSONInt}
type_map!{i64, i64, JSONInt}
type_map!{u8, i64, JSONInt}
type_map!{u16, i64, JSONInt}
type_map!{u32, i64, JSONInt}
type_map!{u64, f64, JSONFloat}
type_map!{f32, f64, JSONFloat}
type_map!{f64, f64, JSONFloat}

