use super::JSON;
use super::super::number::Number;

impl From<String> for JSON {
    fn from(s: String) -> Self {
        JSON::String(s)
    }
}

impl <'arbitrary> From<&'arbitrary str> for JSON {
    fn from(s: &'arbitrary str) -> Self {
        JSON::String(s.to_string())
    }
}

impl From<bool> for JSON {
    fn from(b: bool) -> Self {
        JSON::Bool(b)
    }
}


macro_rules! number_type_map {
    ($rust_type: ty, $inter_type: ty, $json_type: ident) => {
        impl From<$rust_type> for JSON {
            fn from(n: $rust_type) -> Self {
                JSON::Number(Number::$json_type(n as $inter_type))
            }
        }
    };
}

number_type_map!{i8,  i64, Int}
number_type_map!{i16, i64, Int}
number_type_map!{i32, i64, Int}
number_type_map!{i64, i64, Int}
number_type_map!{u8,  u64, UInt}
number_type_map!{u16, u64, UInt}
number_type_map!{u32, u64, UInt}
number_type_map!{u64, u64, UInt}
number_type_map!{f32, f64, Float}
number_type_map!{f64, f64, Float}

