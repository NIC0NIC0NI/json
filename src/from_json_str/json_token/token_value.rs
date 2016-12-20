use super::super::JSONToken;
use super::TokenValue;

impl <'arbitrary> TokenValue for &'arbitrary str {
    fn to_token(self) -> JSONToken{
        JSONToken::StringToken(self.to_string())
    }
}

impl TokenValue for String {
    fn to_token(self) -> JSONToken{
        JSONToken::StringToken(self)
    }
}

macro_rules! type_map {
    ($rust_type: ty, $inter_type: ty, $json_type: ident) => {
        impl TokenValue for $rust_type {
            fn to_token(self) -> JSONToken{
                JSONToken::$json_type(self as $inter_type)
            }
        }
    };
    ($rust_type: ty, $json_type: ident) => {
        impl TokenValue for $rust_type {
            fn to_token(self) -> JSONToken{
                JSONToken::$json_type(self)
            }
        }
    };
}

type_map!{bool, BoolToken}
type_map!{i8, i64, IntToken}
type_map!{i16, i64, IntToken}
type_map!{i32, i64, IntToken}
type_map!{i64, i64, IntToken}
type_map!{u8, i64, IntToken}
type_map!{u16, i64, IntToken}
type_map!{u32, i64, IntToken}
type_map!{u64, f64, FloatToken}
type_map!{f32, f64, FloatToken}
type_map!{f64, f64, FloatToken}

