use super::{JSON, IntoJSON};
use super::super::number::Number;


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

impl IntoJSON for bool {
    fn into_json(self) -> JSON {
        JSON::Bool(self)
    }
}

macro_rules! number_type_map {
    ($rust_type: ty, $inter_type: ty, $json_type: ident) => {
        impl IntoJSON for $rust_type {
            fn into_json(self) -> JSON{
                JSON::Number(Number::$json_type(self as $inter_type))
            }
        }
    };
}

number_type_map!{i8, i64, Int}
number_type_map!{i16, i64, Int}
number_type_map!{i32, i64, Int}
number_type_map!{i64, i64, Int}
number_type_map!{u8, i64, Int}
number_type_map!{u16, i64, Int}
number_type_map!{u32, i64, Int}
number_type_map!{u64, f64, Float} // u64 to i64 may overflow
number_type_map!{f32, f64, Float}
number_type_map!{f64, f64, Float}

