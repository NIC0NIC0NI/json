#[cfg(test)]
#[macro_use]
pub mod test;

use ::std::fmt::{Display, Formatter, Result as FmtResult};
use super::super::json_object::{JSON, JSONNumber};

#[derive(PartialEq, Debug)]
pub enum JSONToken {
    LeftBrace,
    RightBrace,
    LeftBracket, 
    RightBracket,
    Comma,
    Colon,
    StringToken(String),
    BoolToken(bool),
    NumberToken(JSONNumber),
    NullToken,
}

impl JSONToken {
    pub fn is_primitive_value(&self) -> bool {
        match self {
            &JSONToken::StringToken(_) | &JSONToken::BoolToken(_) | 
                &JSONToken::NumberToken(_) |
                    &JSONToken::NullToken => true,
            _ => false
        }
    }
    pub fn into_primitive_value(self) -> Option<JSON> {
        match self {
            JSONToken::StringToken(s) => Some(JSON::String(s)),
            JSONToken::BoolToken(b) => Some(JSON::Bool(b)),
            JSONToken::NumberToken(n) => Some(JSON::Number(n)),
            JSONToken::NullToken => Some(JSON::Null),
            _ => None
        }
    }
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
            &JSONToken::NumberToken(n) => write!(f, "{}", n),
            &JSONToken::NullToken => write!(f, "null")
        }
    }
}
