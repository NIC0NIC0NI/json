
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
impl TokenValue for bool {
    fn to_token(self) -> JSONToken{
        JSONToken::BoolToken(self)
    }
}

macro_rules! register_numeric_type {
    ($rust_type: ty) => {
        impl TokenValue for $rust_type {
            fn to_token(self) -> JSONToken{
                JSONToken::NumberToken(self.to_string())
            }
        }
    };
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

