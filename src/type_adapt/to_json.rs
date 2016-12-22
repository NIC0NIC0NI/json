use super::{MakeJSON, JSONObject, JSONArray};
use super::super::convert::{FromPremitive};

impl <'a, JSON> FromPremitive<&'a str > for JSON
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    fn from_premitive(s: &'a str) -> Self {
        <JSON as MakeJSON>::make_string(s.to_string())
    }
}

impl <JSON> FromPremitive<String> for JSON
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    fn from_premitive(s: String) -> Self {
        <JSON as MakeJSON>::make_string(s)
    }
}

impl <JSON> FromPremitive<bool> for JSON
    where JSON: MakeJSON,
          <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
          <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
    fn from_premitive(b: bool) -> Self {
        <JSON as MakeJSON>::make_bool(b)
    }
}

macro_rules! register_numeric_type {
    ($t:ty) => {
        impl <JSON> FromPremitive<$t> for JSON
            where JSON: MakeJSON,
                <JSON as MakeJSON>::Array : JSONArray<JSON=JSON>,
                <JSON as MakeJSON>::Object : JSONObject<JSON=JSON>{
            fn from_premitive(n: $t) -> JSON {
                <JSON as MakeJSON>::make_number(&n.to_string()).unwrap()
            }
        }
    }
}

register_numeric_type!{i8}
register_numeric_type!{i16}
register_numeric_type!{i32}
register_numeric_type!{i64}
register_numeric_type!{u8}
register_numeric_type!{u16}
register_numeric_type!{u32}
register_numeric_type!{u64}
register_numeric_type!{f32}
register_numeric_type!{f64}