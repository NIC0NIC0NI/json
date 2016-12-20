use super::JSON;
use super::IntoJSON;


impl <'arbitrary> IntoJSON for &'arbitrary str {
    fn into_json(self) -> JSON{
        JSON::String(self.to_string())
    }
}

impl IntoJSON for String {
    fn into_json(self) -> JSON{
        JSON::String(self)
    }
}

macro_rules! type_map {
    ($rust_type: ty, $inter_type: ty, $json_type: ident) => {
        impl IntoJSON for $rust_type {
            fn into_json(self) -> JSON{
                JSON::$json_type(self as $inter_type)
            }
        }
    };
    ($rust_type: ty, $json_type: ident) => {
        impl IntoJSON for $rust_type {
            fn into_json(self) -> JSON{
                JSON::$json_type(self)
            }
        }
    };
}

type_map!{bool, Bool}
type_map!{i8, i64, Int}
type_map!{i16, i64, Int}
type_map!{i32, i64, Int}
type_map!{i64, i64, Int}
type_map!{u8, i64, Int}
type_map!{u16, i64, Int}
type_map!{u32, i64, Int}
type_map!{u64, f64, Float}
type_map!{f32, f64, Float}
type_map!{f64, f64, Float}

