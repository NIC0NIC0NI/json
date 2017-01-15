use super::{MakeJSON};
use super::super::convert::{FromPremitive};

use ::std::error::Error;
use ::std::str::FromStr;

impl <JSON> FromPremitive<bool> for JSON
    where JSON: MakeJSON ,
          <<JSON as MakeJSON>::Number as FromStr>::Err: Error + 'static{
    fn from_premitive(b: bool) -> Self {
        <JSON as MakeJSON>::make_bool(b, &Vec::new())
    }
}

impl <JSON> FromPremitive<String> for JSON
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err: Error + 'static{
    fn from_premitive(s: String) -> Self {
        <JSON as MakeJSON>::make_string(s, &Vec::new())
    }
}

impl <'a, JSON> FromPremitive<&'a str> for JSON
    where JSON: MakeJSON,
          <<JSON as MakeJSON>::Number as FromStr>::Err: Error + 'static{
    fn from_premitive(s: &'a str) -> Self {
        <JSON as MakeJSON>::make_string(s.to_string(), &Vec::new())
    }
}



macro_rules! register_numeric_type {
    ($t:ty) => {
        impl <JSON> FromPremitive<$t> for JSON
            where JSON: MakeJSON,
                <JSON as MakeJSON>::Number : FromStr + FromPremitive<$t>,
                <<JSON as MakeJSON>::Number as FromStr>::Err: Error + 'static {
            fn from_premitive(n: $t) -> JSON {
                <JSON as MakeJSON>::make_number(
                    <<JSON as MakeJSON>::Number as FromPremitive<$t>>::from_premitive(n),
                    &Vec::new()
                )
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
