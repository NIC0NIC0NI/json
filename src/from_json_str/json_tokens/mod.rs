mod token_value;

use super::JSONToken;
use ::std::fmt::{Display, Debug, Formatter, Result as FmtResult};

pub trait TokenValue {
     fn to_token(self) -> JSONToken;
}

// Explanations see `json_object` macro
// Mainly for testing purposes
#[macro_export]
macro_rules! json_tokens {
    ( [$($item:tt),+] ) => {{
        let mut tokens = vec![$crate::from_json_str::JSONToken::LeftBracket];
        $(
            tokens.extend(json_tokens!($item).into_iter());
            tokens.push($crate::from_json_str::JSONToken::Comma);
        )*
        tokens.pop();
        tokens.push($crate::from_json_str::JSONToken::RightBracket);
        tokens
    }};
    ( [] ) => {
        vec![$crate::from_json_str::JSONToken::LeftBracket, $crate::from_json_str::JSONToken::RightBracket]
    };


    ( {$($name:ident : $value:tt),+} ) => {{
        let mut tokens = vec![$crate::from_json_str::JSONToken::LeftBrace];
        $(
            tokens.push($crate::from_json_str::JSONToken::StringToken(stringify!($name).to_string()));
            tokens.push($crate::from_json_str::JSONToken::Colon);
            tokens.extend(json_tokens!($value).into_iter());
            tokens.push($crate::from_json_str::JSONToken::Comma);
        )*
        tokens.pop();
        tokens.push($crate::from_json_str::JSONToken::RightBrace);
        tokens
    }};
    ( {} ) => {
        vec![$crate::from_json_str::JSONToken::LeftBrace, $crate::from_json_str::JSONToken::RightBrace]
    };


    (null) => {
        vec![$crate::from_json_str::JSONToken::NullToken]
    };
    ($x:expr) => {
        vec![$crate::from_json_str::json_tokens::TokenValue::to_token($x)]
    };
}

impl Display for JSONToken {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &JSONToken::LeftBrace => write!(f, "{{"),
            &JSONToken::RightBrace => write!(f, "}}"),
            &JSONToken::LeftBracket => write!(f, "["),
            &JSONToken::RightBracket => write!(f, "]"),
            &JSONToken::Comma => write!(f, ", "),
            &JSONToken::Colon => write!(f, ": "),
            &JSONToken::StringToken(ref s) => write!(f, "\"{}\"", s),
            &JSONToken::BoolToken(s) => write!(f, "{}", s),
            &JSONToken::IntToken(s) => write!(f, "{}", s),
            &JSONToken::FloatToken(s) => write!(f, "{}", s),
            &JSONToken::NullToken => write!(f, "null")
        }
    }
}

impl Debug for JSONToken {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &JSONToken::LeftBrace => write!(f, "{{"),
            &JSONToken::RightBrace => write!(f, "}}"),
            &JSONToken::LeftBracket => write!(f, "["),
            &JSONToken::RightBracket => write!(f, "]"),
            &JSONToken::Comma => write!(f, ", "),
            &JSONToken::Colon => write!(f, ": "),
            &JSONToken::StringToken(ref s) => write!(f, "\"{:?}\"", s),
            &JSONToken::BoolToken(s) => write!(f, "{:?}", s),
            &JSONToken::IntToken(s) => write!(f, "{:?}", s),
            &JSONToken::FloatToken(s) => write!(f, "{:?}", s),
            &JSONToken::NullToken => write!(f, "null")
        }
    }
}

impl PartialEq for JSONToken {
    fn eq(&self, other: &JSONToken) -> bool{
        match self {
            &JSONToken::LeftBrace => {
                match other {
                    &JSONToken::LeftBrace => true,
                    _ => false
                }
            },
            &JSONToken::RightBrace => {
                match other {
                    &JSONToken::RightBrace => true,
                    _ => false
                }
            },&JSONToken::LeftBracket => {
                match other {
                    &JSONToken::LeftBracket => true,
                    _ => false
                }
            },&JSONToken::RightBracket => {
                match other {
                    &JSONToken::RightBracket => true,
                    _ => false
                }
            },
            &JSONToken::Colon => {
                match other {
                    &JSONToken::Colon => true,
                    _ => false
                }
            },
            &JSONToken::Comma => {
                match other {
                    &JSONToken::Comma => true,
                    _ => false
                }
            },
            &JSONToken::NullToken => {
                match other {
                    &JSONToken::NullToken => true,
                    _ => false
                }
            },
            &JSONToken::StringToken(ref s1) => {
                match other {
                    &JSONToken::StringToken(ref s2) => s1 == s2,
                    _ => false
                }
            },
            &JSONToken::BoolToken(b1) => {
                match other {
                    &JSONToken::BoolToken(b2) => b1 == b2,
                    _ => false
                }
            },
            &JSONToken::IntToken(i1) => {
                match other {
                    &JSONToken::IntToken(i2) => i1 == i2,
                    _ => false
                }
            },
            &JSONToken::FloatToken(f1) => {
                match other {
                    &JSONToken::FloatToken(f2) => f1 == f2,
                    _ => false
                }
            },
        }
    }
}